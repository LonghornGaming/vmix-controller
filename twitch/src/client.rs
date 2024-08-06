use anyhow::{Context, Result};
use oauth2::{basic::BasicClient, url::Url, AuthUrl, AuthorizationCode, ClientId, CsrfToken, TokenResponse};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use log::info;

pub struct Client {
    client: BasicClient,
}

impl Client {
    pub fn new(client_id: &str) -> Self {
        let client = BasicClient::new(
            ClientId::new(client_id.to_string()),
            None,
            AuthUrl::new("https://id.twitch.tv/oauth2/authorize".to_string()).expect("Redirect URL is valid"),
            None,
        );
        Self { client }
    }

    pub fn auth(&self) -> Result<AuthorizationCode>{
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .use_implicit_flow()
            .url();

        // println!("Opening authorization link in browser");
        // open::that(auth_url.as_str())?;
        println!("Open this URL in your browser:\n{auth_url}\n");

        let (code, state) = {
            // A very naive implementation of the redirect server.
            let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

            // The server will terminate itself after collecting the first code.
            let Some(mut stream) = listener.incoming().flatten().next() else {
                panic!("listener terminated without accepting a connection");
            };

            let mut reader = BufReader::new(&stream);

            let mut request_line = String::new();
            reader.read_line(&mut request_line).unwrap();

            let redirect_url = request_line.split_whitespace().nth(1).unwrap();
            let url = Url::parse(&("http://localhost:3000".to_string() + redirect_url)).unwrap();

            let code = url
                .query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
                .unwrap();

            let state = url
                .query_pairs()
                .find(|(key, _)| key == "state")
                .map(|(_, state)| CsrfToken::new(state.into_owned()))
                .unwrap();

            let message = "You can close this window now.";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            (code, state)
        };

        assert_eq!(state.secret(), csrf_token.secret(), "CSRF Tokens must match!");

        Ok(code)
        // let token_result = self.client
        //     .exchange_code(code)
        //     .request(oauth2::reqwest::http_client)?;
        //
        //
        //
        // Ok(token_result.)
    }
}
