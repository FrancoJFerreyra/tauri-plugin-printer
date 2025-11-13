

use std::process::Command;
use std::{sync::mpsc, io::Write};
use std::thread;
use std::fs::{File};
use std::env;
use std::path::{Path, PathBuf};
use crate::declare::{PrintOptions, PrintHtmlOptions};
use crate::{ fsys::remove_file};
use std::os::windows::process::CommandExt;

fn wkhtmltopdf_path() -> PathBuf {
    let exe = env::current_exe().unwrap();
    let exe_dir = exe.parent().unwrap();

    // First try next to the .exe
    let direct = exe_dir.join("wkhtmltopdf.exe");
    if direct.exists() {
        return direct;
    }

    // If not there, try in bin/
    let bin = exe_dir.join("bin").join("wkhtmltopdf.exe");
    if bin.exists() {
        return bin;
    }

    // If still not found → clearer error
    panic!("wkhtmltopdf.exe not found next to the executable or in bin/");
}

/**
 * Create sm.exe to temp
 */
fn create_file(path: String, bin: &[u8]) -> std::io::Result<()> {
    let mut f = File::create(format!("{}sm.exe", path))?;
    f.write_all(bin)?;
  
    f.sync_all()?;
    Ok(())
}

/**
 * init sm.exe
 */
pub fn init_windows() {
    let sm = include_bytes!("bin/sm");
    let dir: std::path::PathBuf = env::temp_dir();
    let result: Result<(), std::io::Error>  = create_file(dir.display().to_string(),sm);
    if result.is_err() {
        panic!("Failed to initialize sm.exe")
    }
}

/**
 * Get printers on windows using powershell
 */
pub fn get_printers() -> String {
    // Create a channel for communication
    let (sender, receiver) = mpsc::channel();

    // Spawn a new thread
    thread::spawn(move || {
        let output = Command::new("powershell")
            .args(["-Command", "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-Printer | Select-Object Name, DriverName, JobCount, PrintProcessor, PortName, ShareName, ComputerName, PrinterStatus, Shared, Type, Priority | ConvertTo-Json"])
            .output().unwrap();
        
        let output_string = String::from_utf8_lossy(&output.stdout).to_string();
        sender.send(output_string).unwrap();
    });

    // Receive the result from the spawned thread
    let result: String = receiver.recv().unwrap();

    return result;
}

/**
 * Get printers by name on windows using powershell
 */
pub fn get_printers_by_name(printername: String) -> String {
    // Create a channel for communication
    let (sender, receiver) = mpsc::channel();

    // Spawn a new thread
    thread::spawn(move || {
        let output = Command::new("powershell")
            .args(["-Command", &format!("[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-Printer -Name '{}' | Select-Object Name, DriverName, JobCount, PrintProcessor, PortName, ShareName, ComputerName, PrinterStatus, Shared, Type, Priority | ConvertTo-Json", printername)])
            .output().unwrap();

        let output_string = String::from_utf8_lossy(&output.stdout).to_string();
        sender.send(output_string).unwrap();
    });

    // Receive the result from the spawned thread
    let result: String = receiver.recv().unwrap();

    return result;
}

/**
 * Print pdf file 
 */
pub fn print_pdf (options: PrintOptions) -> String {
    println!("options id {}", options.id);
    println!("options print_setting {}", options.print_setting);

    let dir: std::path::PathBuf = env::temp_dir();
    let print_setting: String = options.print_setting;
    let shell_command = if print_setting.is_empty() {
        format!("{}sm.exe -print-to-default -silent \"{}\"", dir.display(), options.path)
    } else {
        format!("{}sm.exe -print-to \"{}\" -silent \"{}\"", dir.display(), print_setting, options.path)
    };
    

    // Create a channel for communication
    let (sender, receiver) = mpsc::channel();
    println!("{}", shell_command);
    // Spawn a new thread
    thread::spawn(move || {
        let output = Command::new("powershell")
                        .creation_flags(0x08000000) //prevents open console window on windows
                        .args([shell_command])
                        .output()
                        .unwrap();

        sender.send(String::from_utf8(output.stdout).unwrap()).unwrap();
    });

    // Do other non-blocking work on the main thread

    // Receive the result from the spawned thread
    let result = receiver.recv().unwrap();
    
    

    if options.remove_after_print == true {
        let _ = remove_file(&options.path);
    }
    
    return result;
}

/**
 * Print HTML content
 * 
 * Optimizations:
 * - Improved error handling and resource management
 * - Better temporary file cleanup mechanism
 * - Enhanced wkhtmltopdf parameter configuration
 * - Support for more print options and margin units
 */
pub fn print_html(options: PrintHtmlOptions) -> String {
    // Use Result type for better error handling
    match print_html_internal(options) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("HTML printing failed: {}", e);
            format!("Printing failed: {}", e)
        }
    }
}

/// Generate unique temporary file path
fn generate_temp_file_path(extension: &str) -> Result<PathBuf, String> {
    let temp_dir = env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Failed to get timestamp: {}", e))?
        .as_nanos();
    let filename = format!("tauri_printer_{}_{}.{}", std::process::id(), timestamp, extension);
    Ok(temp_dir.join(filename))
}

/// Internal implementation function, uses Result for error handling
fn print_html_internal(options: PrintHtmlOptions) -> Result<String, String> {
    // Validate HTML content
    if options.html.trim().is_empty() {
        return Err("HTML content cannot be empty".to_string());
    }

    // Check wkhtmltopdf availability
    check_wkhtmltopdf_availability()?;

    // Generate temporary file paths
    let html_path = generate_temp_file_path("html")?;
    let pdf_path = generate_temp_file_path("pdf")?;
    
    println!("html_path: {:?}, pdf_path: {:?}", html_path, pdf_path);

    // Write HTML content to temp file
    std::fs::write(&html_path, &options.html)
        .map_err(|e| format!("Failed to write HTML content: {}", e))?;

    // Build wkhtmltopdf command arguments
    let args = build_wkhtmltopdf_args(&options, &html_path, &pdf_path)?;

    println!("wkhtmltopdf args: {:?}", args);

    // Execute HTML to PDF conversion
    let conversion_result = execute_wkhtmltopdf(&args);
    
    // If conversion fails, clean HTML file and return error
    if let Err(e) = conversion_result {
        let _ = remove_file(&html_path.to_string_lossy());
        return Err(e);
    }

    // Check if PDF file was successfully generated
    if !pdf_path.exists() {
        let _ = remove_file(&html_path.to_string_lossy());
        return Err("PDF file generation failed".to_string());
    }
    
    println!("PDF file generated successfully: {:?}", pdf_path);

    // Create print options and execute printing
    let print_options = PrintOptions {
        path: pdf_path.to_string_lossy().to_string(),
        id: options.printer_id.unwrap_or_default(),
        print_setting: options.print_settings.unwrap_or_default(),
        remove_after_print: options.remove_after_print.unwrap_or(true),
    };

    // Execute print
    let result = print_pdf(print_options);

    // Clean HTML temp file (PDF handled by print_pdf based on remove_after_print)
    let _ = remove_file(&html_path.to_string_lossy());
    
    Ok(result)
}

/// Check wkhtmltopdf availability
fn check_wkhtmltopdf_availability() -> Result<(), String> {
    Command::new(wkhtmltopdf_path())
        .creation_flags(0x08000000) //prevents open console window on windows
        .arg("--version")
        .output()
        .map_err(|_| "wkhtmltopdf is not installed or not in PATH. Please install wkhtmltopdf first.".to_string())?;
    Ok(())
}

/// Build wkhtmltopdf command arguments
fn build_wkhtmltopdf_args(
    options: &PrintHtmlOptions,
    html_path: &Path,
    pdf_path: &Path,
) -> Result<Vec<String>, String> {
    let mut args = vec![
        "--encoding".to_string(),
        "UTF-8".to_string(),
        "--enable-local-file-access".to_string(),
        "--disable-smart-shrinking".to_string(), // disable smart shrinking for better print quality
        "--print-media-type".to_string(),
        "--no-pdf-compression".to_string(),      // disable PDF compression for higher quality
        "--load-error-handling".to_string(),
        "ignore".to_string(),                    // ignore load errors
        "--load-media-error-handling".to_string(),
        "ignore".to_string(),                    // ignore media load errors
    ];

    // Set default margins
    let default_margin = "10mm";
    args.extend([
        "--margin-top".to_string(),
        default_margin.to_string(),
        "--margin-right".to_string(),
        default_margin.to_string(),
        "--margin-bottom".to_string(),
        default_margin.to_string(),
        "--margin-left".to_string(),
        default_margin.to_string(),
    ]);

    // Set page size
    if let Some(ref page_size) = options.page_size {
        args.extend(["--page-size".to_string(), page_size.clone()]);
    } else {
        args.extend(["--page-size".to_string(), "A4".to_string()]);
    }

    // Set orientation
    if let Some(ref orientation) = options.orientation {
        args.extend(["--orientation".to_string(), orientation.clone()]);
    } else {
        args.extend(["--orientation".to_string(), "Portrait".to_string()]);
    }

    // Set custom margins (overrides default margins)
    if let Some(ref margin) = options.margin {
        let unit = margin.unit.as_deref().unwrap_or("mm");
        
        if let Some(top) = margin.top {
            args.extend(["--margin-top".to_string(), format!("{}{}", top, unit)]);
        }
        if let Some(right) = margin.right {
            args.extend(["--margin-right".to_string(), format!("{}{}", right, unit)]);
        }
        if let Some(bottom) = margin.bottom {
            args.extend(["--margin-bottom".to_string(), format!("{}{}", bottom, unit)]);
        }
        if let Some(left) = margin.left {
            args.extend(["--margin-left".to_string(), format!("{}{}", left, unit)]);
        }
    }

    // Add input and output file paths
    args.push(html_path.to_string_lossy().to_string());
    args.push(pdf_path.to_string_lossy().to_string());

    Ok(args)
}

/// Execute wkhtmltopdf command
fn execute_wkhtmltopdf(args: &[String]) -> Result<(), String> {
    let output = Command::new(wkhtmltopdf_path())
        .creation_flags(0x08000000) //prevents open console window on windows
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute wkhtmltopdf: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "wkhtmltopdf conversion failed (exit code: {})\nStderr: {}\nStdout: {}",
            output.status.code().unwrap_or(-1),
            stderr,
            stdout
        ));
    }

    Ok(())
}

/**
 * Get printer job on windows using powershell
 */
pub fn get_jobs(printername: String) -> String {
    let output = Command::new("powershell").args([format!("Get-PrintJob -PrinterName \"{}\"  | Select-Object DocumentName,Id,TotalPages,Position,Size,SubmmitedTime,UserName,PagesPrinted,JobTime,ComputerName,Datatype,PrinterName,Priority,SubmittedTime,JobStatus | ConvertTo-Json", printername)]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Get printer job by id on windows using powershell
 */
pub fn get_jobs_by_id(printername: String, jobid: String) -> String {
    let output = Command::new("powershell").args([format!("Get-PrintJob -PrinterName \"{}\" -ID \"{}\"  | Select-Object DocumentName,Id,TotalPages,Position,Size,SubmmitedTime,UserName,PagesPrinted,JobTime,ComputerName,Datatype,PrinterName,Priority,SubmittedTime,JobStatus | ConvertTo-Json", printername, jobid)]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Resume printers job on windows using powershell
 */
pub fn resume_job(printername: String, jobid: String) -> String {
    let output = Command::new("powershell").args([format!("Resume-PrintJob -PrinterName \"{}\" -ID \"{}\" ", printername, jobid)]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Restart printers job on windows using powershell
 */
pub fn restart_job(printername: String, jobid: String) -> String {
    let output = Command::new("powershell").args([format!("Restart-PrintJob -PrinterName \"{}\" -ID \"{}\" ", printername, jobid)]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Pause printers job on windows using powershell
 */
pub fn pause_job(printername: String, jobid: String) -> String {
    let output = Command::new("powershell").args([format!("Suspend-PrintJob -PrinterName \"{}\" -ID \"{}\" ", printername, jobid)]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Remove printers job on windows using powershell
 */
pub fn remove_job(printername: String, jobid: String) -> String {
    let output = Command::new("powershell").args([format!("Remove-PrintJob -PrinterName \"{}\" -ID \"{}\" ", printername, jobid)]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}
