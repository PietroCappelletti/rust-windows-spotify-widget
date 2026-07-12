use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};
use std::time::{Duration, SystemTime};
use url::Url;

const SPOTIFY_AUTH_URL: &str = "https://accounts.spotify.com/authorize";
const SPOTIFY_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

// Spotify usually reports 3600s (1 hour), but if the field is ever missing,
// assume a conservative default rather than treating it as never-expiring.
const DEFAULT_TOKEN_LIFETIME: Duration = Duration::from_secs(3600);

/// Tokens returned after a successful OAuth exchange.
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: SystemTime,
}

pub async fn authorize(
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
) -> Result<AuthTokens, String> {
    let auth_url = AuthUrl::new(SPOTIFY_AUTH_URL.to_string()).map_err(|e| e.to_string())?;
    let token_url = TokenUrl::new(SPOTIFY_TOKEN_URL.to_string()).map_err(|e| e.to_string())?;
    let redirect_url = RedirectUrl::new(redirect_uri.to_string()).map_err(|e| e.to_string())?;

    let client = BasicClient::new(ClientId::new(client_id.to_string()))
        .set_client_secret(ClientSecret::new(client_secret.to_string()))
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user-read-playback-state".to_string()))
        .add_scope(Scope::new("user-modify-playback-state".to_string()))
        .add_scope(Scope::new("user-read-currently-playing".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    if webbrowser::open(auth_url.as_str()).is_err() {
        eprintln!(
            "[auth] Couldn't open a browser automatically. Visit this URL manually:\n{}",
            auth_url
        );
    } else {
        eprintln!("[auth] Opened browser for Spotify login...");
    }

    let port = redirect_port(redirect_uri)?;
    let expected_state = csrf_token.secret().clone();

    let (code, state) = tokio::task::spawn_blocking(move || wait_for_redirect(port))
        .await
        .map_err(|e| format!("redirect listener task panicked: {e}"))??;

    if state != expected_state {
        return Err("CSRF state mismatch — aborting login for safety".to_string());
    }

    let http_client = oauth2::reqwest::ClientBuilder::new()
        .redirect(oauth2::reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())?;

    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await
        .map_err(|e| format!("token exchange failed: {e}"))?;

    let lifetime = token_result.expires_in().unwrap_or(DEFAULT_TOKEN_LIFETIME);

    Ok(AuthTokens {
        access_token: token_result.access_token().secret().clone(),
        refresh_token: token_result.refresh_token().map(|t| t.secret().clone()),
        expires_at: SystemTime::now() + lifetime,
    })
}

/// Exchanges a stored refresh token for a fresh access token, without
/// needing the user to log in again.
pub async fn refresh(
    refresh_token: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
) -> Result<AuthTokens, String> {
    let auth_url = AuthUrl::new(SPOTIFY_AUTH_URL.to_string()).map_err(|e| e.to_string())?;
    let token_url = TokenUrl::new(SPOTIFY_TOKEN_URL.to_string()).map_err(|e| e.to_string())?;
    let redirect_url = RedirectUrl::new(redirect_uri.to_string()).map_err(|e| e.to_string())?;

    let client = BasicClient::new(ClientId::new(client_id.to_string()))
        .set_client_secret(ClientSecret::new(client_secret.to_string()))
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url);

    let http_client = oauth2::reqwest::ClientBuilder::new()
        .redirect(oauth2::reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())?;

    let token_result = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token.to_string()))
        .request_async(&http_client)
        .await
        .map_err(|e| format!("token refresh failed: {e}"))?;

    let lifetime = token_result.expires_in().unwrap_or(DEFAULT_TOKEN_LIFETIME);

    Ok(AuthTokens {
        access_token: token_result.access_token().secret().clone(),
        refresh_token: token_result
            .refresh_token()
            .map(|t| t.secret().clone())
            .or_else(|| Some(refresh_token.to_string())),
        expires_at: SystemTime::now() + lifetime,
    })
}

fn redirect_port(redirect_uri: &str) -> Result<u16, String> {
    Url::parse(redirect_uri)
        .map_err(|e| e.to_string())?
        .port()
        .ok_or_else(|| "redirect URI must include a port, e.g. :8888".to_string())
}

fn wait_for_redirect(port: u16) -> Result<(String, String), String> {
    let server = tiny_http::Server::http(("127.0.0.1", port))
        .map_err(|e| format!("failed to bind local server on port {port}: {e}"))?;

    let request = server.recv().map_err(|e| e.to_string())?;

    let full_url = format!("http://127.0.0.1{}", request.url());
    let parsed = Url::parse(&full_url).map_err(|e| e.to_string())?;

    let mut code = None;
    let mut state = None;
    for (key, value) in parsed.query_pairs() {
        match key.as_ref() {
            "code" => code = Some(value.to_string()),
            "state" => state = Some(value.to_string()),
            _ => {}
        }
    }

    let response = tiny_http::Response::from_string(
        "Spotify authorization complete — you can close this tab and return to the widget.",
    );
    let _ = request.respond(response);

    let code = code.ok_or_else(|| "redirect missing `code` parameter".to_string())?;
    let state = state.ok_or_else(|| "redirect missing `state` parameter".to_string())?;

    Ok((code, state))
}