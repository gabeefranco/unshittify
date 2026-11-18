// Prevents an extra console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // `app --render /some/route` prints that page as a standalone HTML document
    // and exits, so routes can be checked without opening a window.
    let args: Vec<String> = std::env::args().collect();
    if let [_, flag, path] = args.as_slice() {
        if flag == "--render" {
            print!("{}", app_lib::render_standalone(path));
            return;
        }
    }
    app_lib::run();
}
