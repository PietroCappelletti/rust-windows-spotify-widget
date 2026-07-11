/// Handles the Spotify OAuth2 flow: opening the browser, catching
/// the redirect, and exchanging/refreshing tokens.
pub struct AuthTokens {
  pub access_token: String,
  pub refresh_token: String,
}

pub async fn authorize(client_id: &str, client_secret: &str) -> Result<AuthTokens, String> {
  todo!("run oauth2 flow with local redirect server")
}

pub async fn refresh(refresh_token: &str, client_id: &str, client_secret: &str) -> Result<AuthTokens, String> {
  todo!("refresh access token")
}