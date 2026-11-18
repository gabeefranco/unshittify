//! One module per screen type. Each page function returns `Markup` for `<main>`.

pub mod example;

use maud::{html, Markup};

use crate::{html::link, router::Req, state::AppState};

pub async fn home(_req: &Req, _state: &AppState) -> anyhow::Result<Markup> {
    Ok(html! {
        h1 { "app" }
        p { "A native, text-first client. Replace this page with the app's real home." }
        h2 { "Contents" }
        ul {
            li { (link("/notes", "notes")) " — example list, search, detail page and actions" }
            li { (link("/about", "about")) " — keys and how this app works" }
        }
    })
}

pub fn about() -> Markup {
    html! {
        h1 { "about" }
        h2 { "Name" }
        p { "app — a text-first native client" }
        h2 { "Description" }
        p {
            "Pages are rendered by Rust and shown as plain HTML. "
            "Every underlined word is a link; words in [brackets] are actions."
        }
        h2 { "Keys" }
        dl {
            dt { "Alt+← / Alt+→" } dd { "back / forward" }
            dt { "/" } dd { "focus the search box on the current page" }
            dt { "Tab" } dd { "move between links and buttons" }
        }
    }
}

/// The status line under the header. Empty by default; show global state here
/// (now playing, logged-in user, unread count).
pub async fn status(_state: &AppState) -> Markup {
    html! {}
}

pub fn not_found(req: &Req) -> Markup {
    html! {
        h1 { "not found" }
        p { "No page at " code { (req.path) } "." }
        p { (link("/", "home")) }
    }
}

pub fn error(req: &Req, err: &anyhow::Error) -> Markup {
    html! {
        h1 { "error" }
        pre { (format!("{err:#}")) }
        p { (link(&req.path, "retry")) " · " (link("/", "home")) }
    }
}
