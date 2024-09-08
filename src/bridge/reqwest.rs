
//! Bridged support for the `reqwest` HTTP client.

use reqwest::Client as ReqwestClient;
use reqwest::header::CONTENT_TYPE;
use crate::constants::BASE_TOKEN_URI;
use crate::model::{
    AccessTokenExchangeRequest,
    AccessTokenResponse,
    RefreshTokenRequest,
};
use crate::Result;

/// A trait used that implements methods for interacting with Discord's OAuth2
/// API on Reqwest's client.
///
/// # Examples
///
/// Bringing in the trait and creating a client. Since the trait is in scope,
/// the instance of reqwest's Client will have those methods available:
///
/// ```rust,no_run
/// extern crate reqwest;
/// extern crate serenity_oauth;
///
/// # fn main() {
/// use reqwest::Client;
///
/// let client = Client::new();
///
/// // At this point, the methods defined by the trait are not in scope. By
/// // using the trait, they will be.
/// use serenity_oauth::DiscordOAuthReqwestRequester;
///
/// // The methods defined by `DiscordOAuthReqwestRequester` are now in scope and
/// // implemented on the instance of reqwest's `Client`.
/// # }
/// ```
///
/// For examples of how to use the trait with the Client, refer to the trait's
/// methods.
pub trait DiscordOAuthReqwestRequester {
    /// Exchanges a code for the user's access token.
    ///
    /// # Examples
    ///
    /// Exchange a code for an access token:
    ///
    /// ```rust,no_run
    /// extern crate reqwest;
    /// extern crate serenity_oauth;
    ///
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use reqwest::Client;
    /// use serenity_oauth::model::AccessTokenExchangeRequest;
    /// use serenity_oauth::DiscordOAuthReqwestRequester;
    ///
    /// let request_data = AccessTokenExchangeRequest::new(
    ///     249608697955745802,
    ///     "dd99opUAgs7SQEtk2kdRrTMU5zagR2a4",
    ///     "user code here",
    ///     "https://myapplication.website",
    /// );
    ///
    /// let client = Client::new();
    /// let response = client.exchange_code(&request_data)?;
    ///
    /// println!("Access token: {}", response.access_token);
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    fn exchange_code(&self, request: &AccessTokenExchangeRequest)
        -> impl std::future::Future<Output = Result<AccessTokenResponse>> + Send;

    /// Exchanges a refresh token, returning a new refresh token and fresh
    /// access token.
    ///
    /// # Examples
    ///
    /// Exchange a refresh token:
    ///
    /// ```rust,no_run
    /// extern crate reqwest;
    /// extern crate serenity_oauth;
    ///
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use reqwest::Client;
    /// use serenity_oauth::model::RefreshTokenRequest;
    /// use serenity_oauth::DiscordOAuthReqwestRequester;
    ///
    /// let request_data = RefreshTokenRequest::new(
    ///     249608697955745802,
    ///     "dd99opUAgs7SQEtk2kdRrTMU5zagR2a4",
    ///     "user code here",
    ///     "https://myapplication.website",
    /// );
    ///
    /// let client = Client::new();
    /// let response = client.exchange_refresh_token(&request_data)?;
    ///
    /// println!("Fresh access token: {}", response.access_token);
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    fn exchange_refresh_token(&self, request: &RefreshTokenRequest)
        -> impl std::future::Future<Output = Result<AccessTokenResponse>> + Send;
}

impl DiscordOAuthReqwestRequester for ReqwestClient {
    async fn exchange_code(&self, request: &AccessTokenExchangeRequest)
        -> Result<AccessTokenResponse> {
        let body = serde_urlencoded::to_string(request)?;

        let response = self.post(BASE_TOKEN_URI)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .query(&body)
            .send().await?.json::<AccessTokenResponse>().await?;

        Ok(response)
    }

    async fn exchange_refresh_token(&self, request: &RefreshTokenRequest)
        -> Result<AccessTokenResponse> {
        let body = serde_urlencoded::to_string(request)?;

        let response = self.post(BASE_TOKEN_URI)
            .query(&body)
            .send().await?.json::<AccessTokenResponse>().await?;

        Ok(response)
    }
}
