mod auth;
mod api;
mod models;
mod token_store;

pub use models::Track;

use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

// Refresh this many seconds *before* actual expiry, so we don't cut it too close.
const REFRESH_BUFFER: Duration = Duration::from_secs(60);

struct TokenState {
    access_token: String,
    refresh_token: Option<String>,
    expires_at: SystemTime,
}

/// High-level client for controlling Spotify playback. Handles automatic
/// access-token refresh internally — callers never need to think about it.
pub struct SpotifyClient {
    state: RwLock<TokenState>,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl SpotifyClient {
    pub async fn connect(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Result<Self, String> {
        let tokens = if let Some(stored) = token_store::load() {
            if let Some(refresh_token) = &stored.refresh_token {
                match auth::refresh(refresh_token, &client_id, &client_secret, &redirect_uri).await {
                    Ok(tokens) => {
                        let _ = token_store::save(&tokens);
                        eprintln!("[auth] Reused stored session — no browser login needed.");
                        tokens
                    }
                    Err(e) => {
                        eprintln!("[auth] Stored session invalid ({e}), falling back to browser login.");
                        let tokens = auth::authorize(&client_id, &client_secret, &redirect_uri).await?;
                        let _ = token_store::save(&tokens);
                        tokens
                    }
                }
            } else {
                let tokens = auth::authorize(&client_id, &client_secret, &redirect_uri).await?;
                let _ = token_store::save(&tokens);
                tokens
            }
        } else {
            let tokens = auth::authorize(&client_id, &client_secret, &redirect_uri).await?;
            let _ = token_store::save(&tokens);
            tokens
        };

        Ok(Self {
            state: RwLock::new(TokenState {
                access_token: tokens.access_token,
                refresh_token: tokens.refresh_token,
                expires_at: tokens.expires_at,
            }),
            client_id,
            client_secret,
            redirect_uri,
        })
    }

    /// Returns a valid access token, transparently refreshing it first if
    /// it's expired or about to expire within `REFRESH_BUFFER`.
    async fn ensure_fresh_token(&self) -> Result<String, String> {
        {
            let state = self.state.read().await;
            let expiring_soon = state
                .expires_at
                .checked_sub(REFRESH_BUFFER)
                .map(|threshold| SystemTime::now() >= threshold)
                .unwrap_or(true);

            if !expiring_soon {
                return Ok(state.access_token.clone());
            }
        }

        // Token is expiring soon (or already expired) — refresh it.
        self.force_refresh().await
    }

    /// Unconditionally refreshes the access token and persists the result.
    /// Used both proactively (ensure_fresh_token) and reactively (on a 401).
    async fn force_refresh(&self) -> Result<String, String> {
        let mut state = self.state.write().await;

        // Another task may have already refreshed while we waited for the
        // write lock — re-check before hitting the network again.
        let still_expiring_soon = state
            .expires_at
            .checked_sub(REFRESH_BUFFER)
            .map(|threshold| SystemTime::now() >= threshold)
            .unwrap_or(true);

        if !still_expiring_soon {
            return Ok(state.access_token.clone());
        }

        let refresh_token = state
            .refresh_token
            .clone()
            .ok_or_else(|| "no refresh token available — full re-login required".to_string())?;

        let tokens = auth::refresh(&refresh_token, &self.client_id, &self.client_secret, &self.redirect_uri)
            .await?;

        let _ = token_store::save(&tokens);

        state.access_token = tokens.access_token.clone();
        state.refresh_token = tokens.refresh_token;
        state.expires_at = tokens.expires_at;

        Ok(state.access_token.clone())
    }

    /// Runs a Spotify API call, refreshing and retrying once if it fails
    /// due to an expired/invalid token that our proactive check missed
    /// (e.g. clock skew, or the token being revoked externally).
    async fn with_valid_token<T, F, Fut>(&self, call: F) -> Result<T, String>
    where
        F: Fn(String) -> Fut,
        Fut: std::future::Future<Output = Result<T, String>>,
    {
        let token = self.ensure_fresh_token().await?;
        match call(token).await {
            Err(e) if e.contains("access token expired or invalid") => {
                let fresh_token = self.force_refresh().await?;
                call(fresh_token).await
            }
            other => other,
        }
    }

    pub async fn play(&self) -> Result<(), String> {
        self.with_valid_token(|token| async move { api::play(&token).await }).await
    }

    pub async fn pause(&self) -> Result<(), String> {
        self.with_valid_token(|token| async move { api::pause(&token).await }).await
    }

    pub async fn next_track(&self) -> Result<(), String> {
        self.with_valid_token(|token| async move { api::next(&token).await }).await
    }

    pub async fn previous_track(&self) -> Result<(), String> {
        self.with_valid_token(|token| async move { api::previous(&token).await }).await
    }

    pub async fn get_current_track(&self) -> Result<Option<Track>, String> {
        self.with_valid_token(|token| async move { api::current_playback(&token).await }).await
    }
}