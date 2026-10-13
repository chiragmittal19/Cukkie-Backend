use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Language {
    #[default]
    En,
    Hi,
    Ar,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Hi => "hi",
            Language::Ar => "ar",
        }
    }

    pub fn from_header(header: Option<&str>) -> Self {
        match header {
            Some(h) => {
                let lower = h.to_lowercase();
                if lower.starts_with("hi") {
                    Language::Hi
                } else if lower.starts_with("ar") {
                    Language::Ar
                } else {
                    Language::En
                }
            }
            None => Language::En,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LanguageHeader(pub Language);

#[async_trait]
impl<S> FromRequestParts<S> for LanguageHeader
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let lang = parts
            .headers
            .get(axum::http::header::ACCEPT_LANGUAGE)
            .and_then(|v| v.to_str().ok())
            .map(|s| Language::from_header(Some(s)))
            .unwrap_or(Language::En);

        Ok(LanguageHeader(lang))
    }
}
