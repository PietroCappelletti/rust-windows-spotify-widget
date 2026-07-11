mod auth;
mod api;
mod models;
mod token_store;

pub use models::Track;

pub struct SpotifyClient {
  access_token: String,
  #[allow(dead_code)]
  refresh_token: Option<String>,
  #[allow(dead_code)]
  client_id: String,
  #[allow(dead_code)]
  client_secret: String,
  #[allow(dead_code)]
  redirect_uri: String,
}

impl SpotifyClient {
  /// Connects to Spotify, reusing a stored session if possible instead
  /// of always opening the browser for a fresh login.
  pub async fn connect(
    client_id: String,
    client_secret: String,
    redirect_uri: String,
  ) -> Result<Self, String> {
    if let Some(stored) = token_store::load() {
      if let Some(refresh_token) = &stored.refresh_token {
        match auth::refresh(refresh_token, &client_id, &client_secret, &redirect_uri).await {
          Ok(tokens) => {
            let _ = token_store::save(&tokens);
            eprintln!("[auth] Reused stored session — no browser login needed.");
            return Ok(Self {
              access_token: tokens.access_token,
              refresh_token: tokens.refresh_token,
              client_id,
              client_secret,
              redirect_uri,
            });
          }
          Err(e) => {
            eprintln!("[auth] Stored session invalid ({e}), falling back to browser login.");
          }
        }
      }
    }

    let tokens = auth::authorize(&client_id, &client_secret, &redirect_uri).await?;
    let _ = token_store::save(&tokens);

    Ok(Self {
      access_token: tokens.access_token,
      refresh_token: tokens.refresh_token,
      client_id,
      client_secret,
      redirect_uri,
    })
  }

  pub async fn play(&self) -> Result<(), String> {
    api::play(&self.access_token).await
  }

  pub async fn pause(&self) -> Result<(), String> {
    api::pause(&self.access_token).await
  }

  pub async fn next_track(&self) -> Result<(), String> {
    api::next(&self.access_token).await
  }

  pub async fn previous_track(&self) -> Result<(), String> {
    api::previous(&self.access_token).await
  }

  pub async fn get_current_track(&self) -> Result<Option<Track>, String> {
    api::current_playback(&self.access_token).await
  }
}