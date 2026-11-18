//! The service's API: one function per endpoint, returning typed structs built
//! from the samples in docs/samples/. Split into api/<area>.rs as it grows.

#![allow(dead_code)]

use serde::de::DeserializeOwned;

use crate::state::AppState;

/// GET a JSON endpoint, turning failures into readable errors.
pub async fn get_json<T: DeserializeOwned>(state: &AppState, url: &str) -> anyhow::Result<T> {
    let resp = state.http.get(url).send().await?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        let excerpt: String = body.chars().take(300).collect();
        anyhow::bail!("{status} from {url}: {excerpt}");
    }
    Ok(resp.json().await?)
}
