//! The single map of routes. Every page and action in the app is listed here.

use std::collections::HashMap;

use maud::{html, Markup};
use percent_encoding::percent_decode_str;

use crate::{pages, state::AppState};

/// A parsed in-app path: "/notes/3?page=2" → segs ["notes", "3"], query [("page", "2")].
pub struct Req {
    /// The full path including the query, as requested.
    pub path: String,
    /// The path without the query, still percent-encoded.
    pub route: String,
    pub segs: Vec<String>,
    pub query: Vec<(String, String)>,
}

impl Req {
    pub fn parse(path: &str) -> Req {
        let path = if path.starts_with('/') { path.to_string() } else { format!("/{path}") };
        let url = url::Url::parse(&format!("app://local{path}"))
            .unwrap_or_else(|_| url::Url::parse("app://local/").unwrap());
        let segs = url
            .path_segments()
            .map(|s| {
                s.filter(|s| !s.is_empty())
                    .map(|s| percent_decode_str(s).decode_utf8_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default();
        let query = url.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect();
        Req { path, route: url.path().to_string(), segs, query }
    }

    /// First value of a query parameter.
    pub fn q(&self, key: &str) -> Option<&str> {
        self.query.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }

    /// The 1-based `?page=` number.
    pub fn page(&self) -> usize {
        self.q("page").and_then(|p| p.parse().ok()).filter(|&p| p > 0).unwrap_or(1)
    }

    /// This path with one query parameter replaced, e.g. for pagination links.
    pub fn with_query(&self, key: &str, value: &str) -> String {
        let mut q = url::form_urlencoded::Serializer::new(String::new());
        for (k, v) in self.query.iter().filter(|(k, _)| k != key) {
            q.append_pair(k, v);
        }
        q.append_pair(key, value);
        format!("{}?{}", self.route, q.finish())
    }
}

/// GET: render a page. Errors become an error page, never a blank screen.
pub async fn render(path: &str, state: &AppState) -> Markup {
    let req = Req::parse(path);
    let segs: Vec<&str> = req.segs.iter().map(String::as_str).collect();

    let result = match segs.as_slice() {
        [] => pages::home(&req, state).await,
        ["about"] => Ok(pages::about()),
        // Example pages: delete these routes along with pages/example.rs.
        ["notes"] => pages::example::list(&req, state).await,
        ["notes", id] => pages::example::show(id, state).await,
        ["notes", id, "delete"] => pages::example::confirm_delete(id, state).await,
        _ => Ok(pages::not_found(&req)),
    };

    let body = result.unwrap_or_else(|e| pages::error(&req, &e));
    let flash = state.take_flash();
    html! {
        @if let Some(msg) = flash { p.flash { (msg) } }
        (body)
    }
}

/// POST: run an action and return the path to show next.
pub async fn act(
    path: &str,
    form: HashMap<String, String>,
    state: &AppState,
) -> anyhow::Result<String> {
    let req = Req::parse(path);
    let segs: Vec<&str> = req.segs.iter().map(String::as_str).collect();

    match segs.as_slice() {
        ["notes", "new"] => pages::example::create(form, state).await,
        ["notes", id, "delete"] => pages::example::delete(id, state).await,
        _ => anyhow::bail!("no action at {path}"),
    }
}
