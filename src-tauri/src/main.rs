// This attribute configures the application to not show a console window when running in release mode on Windows.
// It's a conditional attribute that checks if the build is a release (`not(debug_assertions)`) and if so,
// sets the application type to `windows`, which creates a GUI application without a console window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager; // Imports the Manager trait, which provides methods to manage application windows, among other things.

// Entry point for the Rust application.
fn main() {
    // Initializes a Tauri application builder with default settings.
    tauri::Builder::default()
        // Setup closure called when the application is ready.
        // This is used to configure the application window and to run any startup code.
        .setup(|app| {
            // Retrieves the main application window. The unwrap call is used to assert that the window definitely exists.
            // In a real-world application, you might want to handle the potential `None` case more gracefully.
            let window = app.get_window("main").unwrap();
            // Executes JavaScript within the context of the webview to navigate to a specified URL.
            // This effectively replaces the current content of the webview with the content of the remote URL.
            // The `expect` call here is used to catch any errors that occur during the evaluation of the JavaScript code.
            // In a production scenario, you might want to handle such errors more gracefully.
            window.eval("window.location.replace('https://play.afterlife-online.com')")
                  .expect("failed to navigate to remote content");
            // Indicates successful setup by returning `Ok(())`.
            Ok(())
        })
        // Finalizes the builder and runs the Tauri application.
        // The `generate_context!()` macro is used to pass configuration data to the application,
        // which includes settings defined in `tauri.conf.json` among others.
        .run(tauri::generate_context!())
        // Handles any errors that occur while running the application.
        // The `expect` call will cause the application to panic and terminate if an error is encountered,
        // displaying the provided error message.
        .expect("error while running tauri application");
}
