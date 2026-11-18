//! Small HTML helpers shared by all pages. maud escapes everything by default:
//! never wrap API data in `PreEscaped`.

use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

use crate::router::Req;

/// A link to an in-app route. Routes live in the URL hash, so the webview's
/// own history gives back/forward for free.
pub fn link(path: &str, text: impl Render) -> Markup {
    html! { a href={ "#" (path) } { (text) } }
}

/// A text button that POSTs to an action route, rendered like `[label]`.
/// It is a `<form>`, which HTML forbids inside `<p>`: put it in a `div.actions`,
/// `li`, `td` or directly in the page.
pub fn action(path: &str, label: &str, fields: &[(&str, &str)]) -> Markup {
    html! {
        form.inline method="post" action=(path) {
            @for (name, value) in fields {
                input type="hidden" name=(name) value=(value);
            }
            button { "[" (label) "]" }
        }
    }
}

/// `« prev  page N  next »` links that keep the rest of the query string.
pub fn pager(req: &Req, has_next: bool) -> Markup {
    let page = req.page();
    html! {
        p.pager {
            @if page > 1 { (link(&req.with_query("page", &(page - 1).to_string()), "« prev")) " " }
            "page " (page)
            @if has_next { " " (link(&req.with_query("page", &(page + 1).to_string()), "next »")) }
        }
    }
}

/// Milliseconds as `m:ss` (or `h:mm:ss`).
#[allow(dead_code)]
pub fn duration(ms: u64) -> String {
    let s = ms / 1000;
    match (s / 3600, s / 60 % 60, s % 60) {
        (0, m, s) => format!("{m}:{s:02}"),
        (h, m, s) => format!("{h}:{m:02}:{s:02}"),
    }
}

/// A full document for `--render`: the page body with the app's stylesheet inlined.
pub fn standalone(path: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                title { (path) }
                style { (PreEscaped(include_str!("../../ui/style.css"))) }
            }
            body {
                header { p.bar { b { "--render " (path) } } }
                main { (body) }
            }
        }
    }
}
