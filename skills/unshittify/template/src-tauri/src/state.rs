use std::path::PathBuf;
use std::sync::Mutex;

/// Everything shared between requests. Add API credentials and caches here.
pub struct AppState {
    /// Per-user app data directory. Store credentials here (see data-access.md), never in the project.
    #[allow(dead_code)]
    pub data_dir: PathBuf,
    pub http: reqwest::Client,
    flash: Mutex<Option<String>>,
    /// Example data for pages/example.rs. Delete together with it.
    pub notes: Mutex<Vec<Note>>,
}

#[derive(Clone)]
pub struct Note {
    pub id: usize,
    pub title: String,
    pub body: String,
}

impl AppState {
    pub fn new(data_dir: PathBuf) -> Self {
        let http = reqwest::Client::builder()
            .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("failed to build HTTP client");
        let notes = (1..=25)
            .map(|id| Note {
                id,
                title: format!("Example note {id}"),
                body: format!("This is the body of note {id}.\nIt is plain text."),
            })
            .collect();
        AppState {
            data_dir,
            http,
            flash: Mutex::new(None),
            notes: Mutex::new(notes),
        }
    }

    /// Set a one-shot message shown at the top of the next rendered page.
    pub fn flash(&self, msg: impl Into<String>) {
        *self.flash.lock().unwrap() = Some(msg.into());
    }

    pub fn take_flash(&self) -> Option<String> {
        self.flash.lock().unwrap().take()
    }
}
