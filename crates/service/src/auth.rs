use std::time::{Duration, SystemTime};

use dataset_server_sdk::error::{ForbiddenError, SigninError};
use derive_more::Debug;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub sk: String,
    pub pk: String,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("jwt error: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),
}

type Result<T> = std::result::Result<T, AuthError>;

const TOKEN_DURATION: u64 = 14 * 24 * 60 * 60;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub data: String,
}

#[derive(Debug, Clone)]
pub struct AuthSigner {
    #[allow(dead_code)]
    provider: String,
    #[debug(skip)]
    key: EncodingKey,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AuthVerifier {
    provider: String,
    #[debug(skip)]
    key: DecodingKey,
}

impl AuthSigner {
    pub fn try_new(provider: impl Into<String>, key: impl AsRef<[u8]>) -> Result<Self> {
        let key = EncodingKey::from_ed_pem(key.as_ref())?;

        Ok(Self {
            provider: provider.into(),
            key,
        })
    }

    pub fn sign(&self, data: String) -> Result<String> {
        let claims = Claims {
            sub: "auth".to_string(),
            exp: SystemTime::now()
                .checked_add(Duration::from_secs(TOKEN_DURATION))
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data,
        };
        let token = encode(&Header::new(Algorithm::EdDSA), &claims, &self.key)?;
        Ok(token)
    }
}

impl AuthVerifier {
    pub fn try_new(provider: impl Into<String>, key: impl AsRef<[u8]>) -> Result<Self> {
        let key = DecodingKey::from_ed_pem(key.as_ref())?;
        Ok(Self {
            provider: provider.into(),
            key,
        })
    }

    pub fn verify(&self, token: impl AsRef<str>) -> Result<Claims> {
        let token = token.as_ref();
        let validation = Validation::new(Algorithm::EdDSA);
        let claims = decode::<Claims>(token, &self.key, &validation)?;

        Ok(claims.claims)
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            sk: include_str!("../fixtures/sk.pem").to_string(),
            pk: include_str!("../fixtures/pk.pem").to_string(),
        }
    }
}

impl From<AuthError> for SigninError {
    fn from(e: AuthError) -> Self {
        Self::ForbiddenError(ForbiddenError {
            message: e.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {}
