mod api;
mod html;
mod pages;
mod router;
mod state;

use std::collections::HashMap;

use state::AppState;
use tauri::Manager;

/// Render the page at `path` (e.g. "/notes?page=2") as an HTML fragment for `<main>`.
#[tauri::command]
async fn render(path: String, state: tauri::State<'_, AppState>) -> Result<String, ()> {
    Ok(router::render(&path, &state).await.into_string())
}

/// Run a POST action and return the path to show next. Errors become a flash
/// message on the page the user came `from`.
#[tauri::command]
async fn act(
    path: String,
    form: HashMap<String, String>,
    from: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, ()> {
    match router::act(&path, form, &state).await {
        Ok(next) => Ok(next),
        Err(e) => {
            state.flash(format!("Error: {e:#}"));
            Ok(from)
        }
    }
}

/// The persistent status line under the header (now playing, logged-in user, ...).
#[tauri::command]
async fn status(state: tauri::State<'_, AppState>) -> Result<String, ()> {
    Ok(pages::status(&state).await.into_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            #[cfg(debug_assertions)]
            eprintln!("app data dir: {}", data_dir.display());
            app.manage(AppState::new(data_dir));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![render, act, status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Render a route outside the GUI. Set APP_DATA_DIR to the dir printed by a
/// debug run to reuse the app's stored credentials.
pub fn render_standalone(path: &str) -> String {
    let data_dir: std::path::PathBuf = std::env::var_os("APP_DATA_DIR")
        .map(Into::into)
        .unwrap_or_else(|| ".appdata".into());
    let state = AppState::new(data_dir);
    let body = tauri::async_runtime::block_on(router::render(path, &state));
    html::standalone(path, body).into_string()
}
