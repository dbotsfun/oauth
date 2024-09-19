//! Bridged support for the `reqwest` HTTP client.

use crate::constants::{BASE_ME_URI, BASE_TOKEN_URI};
use crate::model::{
    AccessTokenExchangeRequest, AccessTokenResponse, AuthDiscordUser, RefreshTokenRequest,
};
use crate::Result;
use reqwest::blocking::Client as ReqwestClient;
use reqwest::header::CONTENT_TYPE;

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
    fn exchange_code(&self, request: &AccessTokenExchangeRequest) -> Result<AccessTokenResponse>;

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
    fn exchange_refresh_token(&self, request: &RefreshTokenRequest) -> Result<AccessTokenResponse>;

    /// Fetches the user's information using the provided access token.
    /// This is useful for verifying the user's identity.
    /// This method does not return the user's information; it only ensures
    /// that the user is valid.
    ///
    /// # Examples
    /// Fetch a user's information:
    ///
    /// ```rust,no_run
    /// extern crate reqwest;
    /// extern crate serenity_oauth;
    ///
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use reqwest::Client;
    /// use serenity_oauth::DiscordOAuthReqwestRequester;
    ///
    /// let client = Client::new();
    /// let user = client.fetch_user("user access token")?;
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    fn fetch_user(&self, token: &str) -> Result<AuthDiscordUser>;
}

impl DiscordOAuthReqwestRequester for ReqwestClient {
    fn exchange_code(&self, request: &AccessTokenExchangeRequest) -> Result<AccessTokenResponse> {
        let body = serde_urlencoded::to_string(request)?;

        let response = self
            .post(BASE_TOKEN_URI)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .query(&body)
            .send()?
            .json::<AccessTokenResponse>()?;

        Ok(response)
    }

    fn exchange_refresh_token(&self, request: &RefreshTokenRequest) -> Result<AccessTokenResponse> {
        let body = serde_urlencoded::to_string(request)?;

        let response = self
            .post(BASE_TOKEN_URI)
            .query(&body)
            .send()?
            .json::<AccessTokenResponse>()?;

        Ok(response)
    }

    fn fetch_user(&self, token: &str) -> Result<AuthDiscordUser> {
        let response = self
            .get(BASE_ME_URI)
            .header("Authorization", format!("Bearer {}", token))
            .send()?
            .json::<AuthDiscordUser>()?;

        Ok(response)
    }
}
