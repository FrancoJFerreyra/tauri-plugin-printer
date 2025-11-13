mod declare;
mod fsys;
mod windows;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

use std::env;

pub use crate::models::*;
use crate::declare::PrintHtmlOptions;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Printer;
#[cfg(mobile)]
use mobile::Printer;

/**
 * Test printer connection
 */
#[tauri::command]
async fn ping<R: Runtime>(app: tauri::AppHandle<R>, payload: PingRequest) -> Result<PingResponse> {
    app.printer().ping(payload)
}

/**
 * Print HTML content
 */
#[tauri::command(rename_all = "snake_case")]
async fn print_html<R: Runtime>(app: tauri::AppHandle<R>, options: PrintHtmlOptions) -> Result<String> {
    println!("print_html: {:?}", options.print_settings);
    app.printer().print_html(options)
}


/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the printer APIs.
pub trait PrinterExt<R: Runtime> {
    fn printer(&self) -> &Printer<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PrinterExt<R> for T {
    fn printer(&self) -> &Printer<R> {
        self.state::<Printer<R>>().inner()
    }
}

/**
 * Create a temporary file
 * @param buffer_data base64 string
 * @param filename file name
 * @returns temporary file path
 */
#[tauri::command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer-wkhtml-bin|create_temp_file')`.
fn create_temp_file(buffer_data: String, filename: String) -> String {
    let dir = env::temp_dir();
    let result = fsys::create_file_from_base64(
        buffer_data.as_str(),
        format!("{}{}", dir.display(), filename).as_str(),
    );
    if result.is_ok() {
        return format!("{}{}", dir.display(), filename);
    }
    return "".to_owned();
}

/**
 * Remove temporary file
 * @param filename file name
 * @returns result of deletion
 */
#[tauri::command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer-wkhtml-bin|create_temp_file')`.
fn remove_temp_file(filename: String) -> bool {
    let dir = env::temp_dir();
    let result = fsys::remove_file(format!("{}{}", dir.display(), filename).as_str());
    if result.is_ok() {
        return true;
    }
    return false;
}

/**
 * Get printer list
 */
#[tauri::command]
// this will be accessible with `invoke('plugin:printer-wkhtml-bin|get_printers')`.
fn get_printers() -> String {
    if cfg!(windows) {
        return windows::get_printers();
    }

    return "Unsupported OS".to_string();
}

/**
 * Get printer list by name
 * @param printername printer name
 * @returns printer list
 */
#[tauri::command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer-wkhtml-bin|get_printer_by_name')`.
fn get_printers_by_name(printername: String) -> String {
    println!("Getting printer list: {}", printername);
    if cfg!(windows) {
        return windows::get_printers_by_name(printername);
    }

    return "Unsupported OS".to_string();
}

/**
 * Print PDF
 * @param id printer ID
 * @param path PDF file path
 * @param printer_setting printer settings
 * @param remove_after_print remove file after printing
 * @returns print result
 */
#[tauri::command(rename_all = "snake_case")]    
// this will be accessible with `invoke('plugin:printer-wkhtml-bin|print_pdf')`.
fn print_pdf(
    id: String,
    path: String,
    printer_setting: String,
    remove_after_print: bool,
) -> String {
     
    if cfg!(windows) {
        let options = declare::PrintOptions { 
            id,
            path,
            print_setting: printer_setting,
            remove_after_print: remove_after_print,
        };
        return windows::print_pdf(options);
    }

    return "Unsupported OS".to_string();
}

#[tauri::command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer-wkhtml-bin|get_jobs')`.
fn get_jobs(printername: String) -> String {
    if cfg!(windows) {
        return windows::get_jobs(printername);
    }
    return "Unsupported OS".to_string();
}

/**
 * Get print job list
 * @param printername printer name
 * @param jobid print job ID
 * @returns job list
 */
#[tauri::command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer-wkhtml-bin|get_jobs_by_id')`.
fn get_jobs_by_id(printername: String, jobid: String) -> String {
    if cfg!(windows) {
        return windows::get_jobs_by_id(printername, jobid);
    }
    return "Unsupported OS".to_string();
}

/**
 * Resume print job
 * @param printername printer name
 * @param jobid print job ID
 * @returns resume result
 */
#[tauri::command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer-wkhtml-bin|restart_job')`.
fn resume_job(printername: String, jobid: String) -> String {
    if cfg!(windows) {
        return windows::resume_job(printername, jobid);
    }
    return "Unsupported OS".to_string();
}

/**
 * Restart print job
 * @param printername printer name
 * @param jobid print job ID
 * @returns restart result
 */
#[tauri::command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer-wkhtml-bin|restart_job')`.
fn restart_job(printername: String, jobid: String) -> String {
    if cfg!(windows) {
        return windows::restart_job(printername, jobid);
    }
    return "Unsupported OS".to_string();
}

/**
 * Pause print job
 * @param printername printer name
 * @param jobid print job ID
 * @returns pause result
 */
#[tauri::command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer-wkhtml-bin|pause_job')`.
fn pause_job(printername: String, jobid: String) -> String {
    if cfg!(windows) {
        return windows::pause_job(printername, jobid);
    }
    return "Unsupported OS".to_string();
}

/**
 * Remove print job
 * @param printername printer name
 * @param jobid print job ID
 * @returns remove result
 */
#[tauri::command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer-wkhtml-bin|remove_job')`.
fn remove_job(printername: String, jobid: String) -> String {
    if cfg!(windows) {
        return windows::remove_job(printername, jobid);
    }
    return "Unsupported OS".to_string();
}

/**
 * Get printer list by name
 * @param printername printer name
 * @returns printer list
 */
pub fn custom_get_printers_by_name(printername: String) -> String {
    if cfg!(windows) {
        return windows::get_printers_by_name(printername);
    }

    return "Unsupported OS".to_string();
}

/**
 * Print PDF
 * @param id printer ID
 * @param path PDF file path
 * @param printer_setting printer settings
 * @param remove_after_print remove file after printing
 * @returns print result
 */
pub fn custom_print_pdf(
    id: String,
    path: String,
    printer_setting: String,
    remove_after_print: bool,
) -> String {
    if cfg!(windows) {
        let options = declare::PrintOptions {
            id,
            path,
            print_setting: printer_setting,
            remove_after_print: remove_after_print,
        };
        return windows::print_pdf(options);
    }

    return "Unsupported OS".to_string();
}

/**
 * Initialize the plugin
 * @returns initialization result
 */
/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  if cfg!(windows) {
    windows::init_windows();
  }
    Builder::new("printer-wkhtml-bin")
        .invoke_handler(tauri::generate_handler![
            ping,
            print_html,
            create_temp_file,
            remove_temp_file,
            get_printers,
            get_printers_by_name,
            print_pdf,
            get_jobs,
            get_jobs_by_id,
            resume_job,
            restart_job,
            pause_job,
            remove_job
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let printer = mobile::init(app, api)?;
            #[cfg(desktop)]
            let printer = desktop::init(app, api)?;
            app.manage(printer);
            Ok(())
        })
        .build()
}
