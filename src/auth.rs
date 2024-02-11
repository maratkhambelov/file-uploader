use crate::config::{self, AppState};
use uuid::Uuid;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};

// use jsonwebtoken as jwt;

const SECRET: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";
const TOKEN_PREFIX: &'static str = "Token ";


#[derive(Debug, Deserialize, Serialize)]
pub struct GuardedRequest {
    pub user_id: Uuid,
    // id: Uuid,
    // timestamp
    // pub exp: i64,
    // user id
    // pub username: String,
}


// impl GuardedRequest {
    // pub fn token(&self, secret: &[u8]) -> String {
    //     let encoding_key = EncodingKey::from_base64_secret(std::str::from_utf8(secret).unwrap());
    //     encode(&Header::default(), self, &encoding_key.unwrap()).expect("jwt")
    // }
// }

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuardedRequest {
    type Error = ();

    /// Extract GuardedRequest token from the "Authorization" header.
    ///
    /// Handlers with GuardedRequest guard will fail with 503 error.
    /// Handlers with Option<GuardedRequest> will be called with None.
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // let state = req.rocket().state::<AppState>().unwrap();
        
        if let Some(user_data) = extract_auth_from_request(req, SECRET.to_string().as_bytes()) {
            Outcome::Success(Self { user_id: user_data.id })
        } else {
            eprintln!("Not Authorized");
            Outcome::Error((Status::Forbidden, ()))
        }
    }
}
fn extract_auth_from_request(request: &Request, secret: &[u8]) -> Option<DecodedToken> {
    //TODO: add expiration check
    let authorization_header: Option<&str> =      request
    .headers()
    .get_one("authorization");

    println!("{}", authorization_header.unwrap_or_default());

    authorization_header.and_then(extract_token_from_header)
        .and_then(|token| decode_token(token, secret))
}



#[derive(Debug, Deserialize, Serialize)]
struct DecodedToken {
    pub exp: i64,
    pub id: Uuid,
    pub username: String,
    // timestamp
    // user id
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(TOKEN_PREFIX) {
        Some(&header[TOKEN_PREFIX.len()..])
    } else {
        eprintln!("There is no token");

        None
    }
}


pub fn generate_token(uuid: &Uuid, username: &str, secret: &[u8]) -> String {
    let exp = Utc::now() + Duration::days(60); // TODO: move to config

    let token = DecodedToken {
        id: *uuid,
        username: String::from(username),
        exp: exp.timestamp(),
    };

    let encoding_key = EncodingKey::from_base64_secret(std::str::from_utf8(secret).unwrap());
    encode(&Header::default(), &token, &encoding_key.unwrap()).expect("jwt")
}

pub fn decode_token(token: &str, secret: &[u8]) -> Option<DecodedToken> {
    use jsonwebtoken::{Algorithm, Validation};

    let decoding_key = DecodingKey::from_base64_secret(std::str::from_utf8(secret).unwrap());

    decode(
        token,
        &decoding_key.unwrap(),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|err| {
        eprintln!("Auth decode error: {:?}", err);
    })
    .ok()
    .map(|token_data| token_data.claims)
}