//! Example pages showing every pattern the app needs: a searchable, paginated
//! list, a detail page, POST actions with flash messages, and a confirm page
//! for a destructive action. Delete this file, its routes in router.rs and
//! `notes` in state.rs once the real pages exist.

use std::collections::HashMap;

use anyhow::Context;
use maud::{html, Markup};

use crate::{
    html::{action, link, pager},
    router::Req,
    state::{AppState, Note},
};

const PER_PAGE: usize = 10;

pub async fn list(req: &Req, state: &AppState) -> anyhow::Result<Markup> {
    let query = req.q("q").unwrap_or("");
    let needle = query.to_lowercase();
    let matching: Vec<Note> = state
        .notes
        .lock()
        .unwrap()
        .iter()
        .filter(|n| n.title.to_lowercase().contains(&needle) || n.body.to_lowercase().contains(&needle))
        .cloned()
        .collect();
    let start = (req.page() - 1) * PER_PAGE;
    let shown = matching.iter().skip(start).take(PER_PAGE);
    let has_next = matching.len() > start + PER_PAGE;

    Ok(html! {
        h1 { "notes" }
        form method="get" action="/notes" {
            input type="search" name="q" value=(query) placeholder="search notes";
            " " button { "[search]" }
        }
        @if matching.is_empty() {
            p { "No notes." }
        } @else {
            p.muted { (matching.len()) " notes" }
            ol start=(start + 1) {
                @for n in shown { li { (link(&format!("/notes/{}", n.id), &n.title)) } }
            }
            (pager(req, has_next))
        }
        h2 { "New note" }
        form method="post" action="/notes/new" {
            p { input name="title" placeholder="title" required; }
            p { textarea name="body" rows="4" cols="60" {} }
            button { "[add]" }
        }
    })
}

pub async fn show(id: &str, state: &AppState) -> anyhow::Result<Markup> {
    let note = find(id, state)?;
    Ok(html! {
        h1 { (note.title) " — note" }
        p.text { (note.body) }
        p {
            (link(&format!("/notes/{}/delete", note.id), "[delete]"))
        }
        h2 { "See also" }
        p { (link("/notes", "all notes")) }
    })
}

pub async fn confirm_delete(id: &str, state: &AppState) -> anyhow::Result<Markup> {
    let note = find(id, state)?;
    Ok(html! {
        h1 { "delete note?" }
        p { "Delete " b { (note.title) } "? This can't be undone." }
        div.actions {
            (action(&format!("/notes/{}/delete", note.id), "delete", &[]))
            " · " (link(&format!("/notes/{}", note.id), "cancel"))
        }
    })
}

pub async fn create(form: HashMap<String, String>, state: &AppState) -> anyhow::Result<String> {
    let title = form.get("title").map(|s| s.trim()).unwrap_or_default();
    anyhow::ensure!(!title.is_empty(), "a note needs a title");
    let body = form.get("body").cloned().unwrap_or_default();
    let id = {
        let mut notes = state.notes.lock().unwrap();
        let id = notes.iter().map(|n| n.id).max().unwrap_or(0) + 1;
        notes.push(Note { id, title: title.to_string(), body });
        id
    };
    state.flash("Note added.");
    Ok(format!("/notes/{id}"))
}

pub async fn delete(id: &str, state: &AppState) -> anyhow::Result<String> {
    let id: usize = id.parse().context("bad note id")?;
    state.notes.lock().unwrap().retain(|n| n.id != id);
    state.flash("Note deleted.");
    Ok("/notes".to_string())
}

fn find(id: &str, state: &AppState) -> anyhow::Result<Note> {
    let id: usize = id.parse().context("bad note id")?;
    state
        .notes
        .lock()
        .unwrap()
        .iter()
        .find(|n| n.id == id)
        .cloned()
        .with_context(|| format!("no note with id {id}"))
}
