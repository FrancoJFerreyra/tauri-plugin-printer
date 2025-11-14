<div align="center">

# Tauri Plugin Printer WKHTML-BIN (Tauri V2)

[![Crates.io](https://img.shields.io/crates/v/tauri-plugin-printer-wkhtml-bin.svg)](https://crates.io/crates/tauri-plugin-printer-wkhtml-bin)
[![NPM](https://img.shields.io/npm/v/tauri-plugin-printer-wkhtml-bin.svg)](https://www.npmjs.com/package/tauri-plugin-printer-wkhtml-bin)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-v2.0-orange.svg)](https://tauri.app/)

**A powerful Tauri V2 printer plugin**

Supports PDF/HTML printing, printer management, print job control, and more.

[Installation](#-installation) • [Usage](#-usage) • [API Documentation](#-api-documentation) • [Examples](#-examples)

</div>

---

## 🚀 Quick Start

### 5-Minute Getting Started Guide

**Important:** Make sure to place the `wkhtmltopdf.exe` binary inside the `src-tauri/bin` folder of your project. The plugin will automatically look for it there.

1. **Install the plugin**
   ```bash
   cargo add tauri-plugin-printer-wkhtml-bin
   npm i tauri-plugin-printer-wkhtml-bin
   ```

2. **Register the plugin**
   ```rust
   // src-tauri/src/lib.rs
   use tauri_plugin_printer_wkhtml_bin::init;
   
   #[cfg_attr(mobile, tauri::mobile_entry_point)]
   pub fn run() {
       tauri::Builder::default()
           .plugin(init())
           .run(tauri::generate_context!())
           .expect("error while running tauri application");
   }
   ```

3. **Configure permissions**
   ```json
   // src-tauri/capabilities/default.json
   {
     "permissions": [
       "printer-wkhtml-bin:allow-get-printers",
       "printer-wkhtml-bin:allow-print-pdf",
       "printer-wkhtml-bin:allow-print-html"
     ]
   }
   ```

4. **Start using**
   ```javascript
   import { getPrinters, printPdf } from 'tauri-plugin-printer-wkhtml-bin';
   
   // Get printers
   const printers = JSON.parse(await getPrinters());
   console.log('Available printers:', printers);
   
   // Print PDF
   await printPdf({
     path: '/path/to/document.pdf',
     printer: printers[0].name
   });
   ```

🎉 **Congratulations!** You have successfully integrated the printer plugin and can now start printing documents.

## Acknowledgements

This project is based on the following open source projects:
- [alfianlensundev/tauri-plugin-printer](https://github.com/alfianlensundev/tauri-plugin-printer)
- [adao99/tauri-plugin-printer-v2](https://github.com/adao99/tauri-plugin-printer-v2)
- [chen-collab's](https://github.com/chen-collab/tauri-plugin-printer)

Thanks to the original authors for their contributions and open source spirit.

## ✨ Features

- 📦 Looks for `wkhtmltopdf` in `bin` folder inside `src-tauri`
- 🛠️ Fixed TypeScript types
- 📝 Updated comments from Chinese to English

## 📦 Installation

**Important:** Make sure to place the `wkhtmltopdf.exe` binary inside the `src-tauri/bin` folder of your project. The plugin will automatically look for it there.

### Method 1: Using crates.io (Recommended)

```bash
# Add Rust dependency
cargo add tauri-plugin-printer-wkhtml-bin

# Install frontend API
npm i tauri-plugin-printer-wkhtml-bin
```

### Method 2: Using Tauri CLI

```bash
npx tauri add https://github.com/FrancoJFerreyra/tauri-plugin-printer
```

### Method 3: Manual Installation

1. Add dependency in `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-printer-wkhtml-bin = "0.1.3"
# Or use Git version
# tauri-plugin-printer-wkhtml-bin = { git = "https://github.com/FrancoJFerreyra/tauri-plugin-printer" }
```

2. Register the plugin in `src-tauri/src/lib.rs`:

```rust
use tauri_plugin_printer_wkhtml_bin::init;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

3. Add permissions in `src-tauri/capabilities/default.json`:

```json
{
  "permissions": [
    "printer-wkhtml-bin:default"
  ]
}
```

4. Install frontend dependency:

```bash
npm i tauri-plugin-printer-wkhtml-bin
```

## 🚀 Usage

### Basic Example

```javascript
import { ping, getPrinters } from 'tauri-plugin-printer-wkhtml-bin';

// Test plugin connection
const response = await ping({ value: 'Hello from frontend!' });
console.log(response);

// Get printer list
const printers = await getPrinters();
console.log('Available printers:', printers);
```

### Full API Example

```javascript
import {
  ping,
  getPrinters,
  getPrintersByName,
  printPdf,
  printHtml,
  getJobs,
  getJobsById,
  resumeJob,
  restartJob,
  pauseJob,
  removeJob
} from 'tauri-plugin-printer-wkhtml-bin';

// 1. Get all printers
const allPrinters = await getPrinters();

// 2. Get specific printer by name
const specificPrinter = await getPrintersByName('Microsoft Print to PDF');

// 3. Print PDF file
const printResult = await printPdf({
  path: '/path/to/your/file.pdf',
  printer: 'Microsoft Print to PDF',
  pages: '1-3',
  subset: 'odd'
});

// 4. Print HTML content
const htmlPrintResult = await printHtml({
  html: '<h1>Hello World</h1><p>This is a HTML print test</p>',
  printer: 'Microsoft Print to PDF'
});

// 5. Get print jobs
const jobs = await getJobs('Microsoft Print to PDF');

// 6. Manage print jobs
const jobId = '123';
const printerName = 'Microsoft Print to PDF';

// Pause job
await pauseJob(printerName, jobId);

// Resume job
await resumeJob(printerName, jobId);

// Restart job
await restartJob(printerName, jobId);

// Remove job
await removeJob(printerName, jobId);
```

## 📋 Examples

### Real-world Use Cases

#### 1. Print Invoice or Report

```javascript
import { printHtml, getPrinters } from 'tauri-plugin-printer-wkhtml-bin';

// Generate invoice HTML
const invoiceHtml = `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Invoice</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .header { text-align: center; margin-bottom: 30px; }
        .invoice-details { margin-bottom: 20px; }
        table { width: 100%; border-collapse: collapse; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
    </style>
</head>
<body>
    <div class="header">
        <h1>Invoice</h1>
        <p>Invoice No: INV-2024-001</p>
    </div>
    <div class="invoice-details">
        <p><strong>Customer:</strong> Zhang San</p>
        <p><strong>Date:</strong> 2024-01-15</p>
    </div>
    <table>
        <tr><th>Item</th><th>Qty</th><th>Price</th><th>Total</th></tr>
        <tr><td>Product A</td><td>2</td><td>¥100</td><td>¥200</td></tr>
        <tr><td>Product B</td><td>1</td><td>¥150</td><td>¥150</td></tr>
    </table>
    <p style="text-align: right; margin-top: 20px;"><strong>total: ¥350</strong></p>
</body>
</html>
`;

// Print invoice
try {
    const result = await printHtml({
        html: invoiceHtml,
        printer: 'Microsoft Print to PDF' // Or choose another printer
    });
    console.log('Invoice printed successfully:', result);
} catch (error) {
    console.error('Printing failed:', error);
}
```

#### 2. Batch Print PDF Files

```javascript
import { printPdf, getPrinters } from 'tauri-plugin-printer-wkhtml-bin';

const pdfFiles = [
    '/path/to/document1.pdf',
    '/path/to/document2.pdf',
    '/path/to/document3.pdf'
];

// Get default printer
const printers = JSON.parse(await getPrinters());
const defaultPrinter = printers.find(p => p.is_default)?.name || printers[0]?.name;

// Batch print
for (const filePath of pdfFiles) {
    try {
        await printPdf({
            path: filePath,
            printer: defaultPrinter,
            pages: '1-10' // Print only the first 10 pages
        });
        console.log(`Printed: ${filePath}`);
    } catch (error) {
        console.error(`Print failed ${filePath}:`, error);
    }
}
```

#### 3. Printer Status Monitoring

```javascript
import { getPrinters, getJobs } from 'tauri-plugin-printer-wkhtml-bin';

// Monitor printer status
async function monitorPrinters() {
    try {
        const printers = JSON.parse(await getPrinters());
        
        for (const printer of printers) {
            console.log(`Printer: ${printer.name}`);
            console.log(`Status: ${printer.status}`);
            console.log(`Default: ${printer.is_default ? 'Yes' : 'No'}`);
            
            // Get print jobs
            const jobs = JSON.parse(await getJobs(printer.name));
            console.log(`Pending jobs: ${jobs.length}`);
            
            console.log('---');
        }
    } catch (error) {
        console.error('Failed to get printer info:', error);
    }
}

// Check every 30 seconds
setInterval(monitorPrinters, 30000);
```

## 📚 API Documentation

### `ping(request: PingRequest): Promise<PingResponse>`
Test plugin connection status.

### `getPrinters(): Promise<string>`
Get all available printers in the system, returned as a JSON string.

### `getPrintersByName(name: string): Promise<string>`
Get info of a specific printer by name.

### `printPdf(options: PrintOptions): Promise<string>`
Print PDF files.

**PrintOptions parameters:**
- `path`: PDF file path
- `printer`: Printer name
- `pages`: Page range (optional)
- `subset`: Page subset (optional)

### `printHtml(options: HtmlPrintOptions): Promise<string>`
Print HTML content.

**HtmlPrintOptions parameters:**
- `html`: HTML content string
- `printer`: Printer name

### Print Job Management

- `getJobs(printer: string): Promise<string>` - Get all jobs for a printer
- `getJobsById(printer: string, jobId: string): Promise<string>` - Get info for a specific job
- `pauseJob(printer: string, jobId: string): Promise<string>` - Pause a print job
- `resumeJob(printer: string, jobId: string): Promise<string>` - Resume a print job
- `restartJob(printer: string, jobId: string): Promise<string>` - Restart a print job
- `removeJob(printer: string, jobId: string): Promise<string>` - Delete a print job

## 🛠️ Development

### Run Example Application

```bash
# Clone repository
git clone https://github.com/FrancoJFerreyra/tauri-plugin-printer.git
cd tauri-plugin-printer

# Build plugin
npm run build

# Run example app
cd examples/tauri-app
npm install
npm run tauri:dev
```

### Project Structure

```
tauri-plugin-printer/
├── src/ # Rust source code
│ ├── lib.rs # Main plugin entrypoint
│ ├── commands.rs # Tauri command definitions
│ ├── desktop.rs # Desktop implementation
│ ├── windows.rs # Windows specific implementation
│ └── ...
├── guest-js/ # JavaScript API
│ └── index.ts # Frontend API definition
├── permissions/ # Permission config
├── examples/ # Sample apps
│ └── tauri-app/ # Vue.js example
└── dist-js/ # Built JS files
```

## 🔧 Permission Configuration

The plugin uses the following permissions:

```toml
[default]
description = "Default permissions for the plugin"
permissions = [
  "allow-ping",
  "allow-create-temp-file", 
  "allow-remove-temp-file", 
  "allow-get-printers", 
  "allow-get-printers-by-name", 
  "allow-print-pdf", 
  "allow-print-html",
  "allow-get-jobs", 
  "allow-get-jobs-by-id", 
  "allow-resume-job", 
  "allow-restart-job", 
  "allow-pause-job", 
  "allow-remove-job"
]
```

## 🐛 Known Issues

- Currently supports Windows systems mainly
- Some printer drivers may not be fully compatible
- Printing large files may need extra memory management
- HTML printing may have formatting issues with complex layouts

## 🔧 Troubleshooting

### Common Issues

#### Q: Cannot retrieve printer list
**A:** Check:
- Ensure a printer is installed on your system
- Ensure the printer service is running correctly
- Confirm the app has printer access permissions

#### Q: PDF printing fails
**A:** Possible solutions:
- Ensure the PDF file path is correct and file exists
- Check if the PDF file is corrupted
- Ensure a PDF reader (like Adobe Reader) is installed
- Try using absolute paths rather than relative

#### Q: HTML print formatting issues
**A:** Suggestions:
- Use simple CSS styles, avoid complex layouts
- Set appropriate page size and margins
- Preview by printing to PDF first
- Avoid JavaScript and external resources

#### Q: Permission configuration issues
**A:** Ensure configuration in `src-tauri/capabilities/default.json`:
```json
{
  "permissions": [
    "printer-wkhtml-bin:allow-ping",
    "printer-wkhtml-bin:allow-get-printers",
    "printer-wkhtml-bin:allow-get-printers-by-name",
    "printer-wkhtml-bin:allow-print-pdf",
    "printer-wkhtml-bin:allow-print-html",
    "printer-wkhtml-bin:allow-get-jobs",
    "printer-wkhtml-bin:allow-restart-job",
    "printer-wkhtml-bin:allow-pause-job",
    "printer-wkhtml-bin:allow-resume-job",
    "printer-wkhtml-bin:allow-remove-job"
  ]
}
```

### Debugging Tips

1. **Enable detailed logs**:
   ```bash
   RUST_LOG=debug npm run tauri dev
   ```

2. **Check printer status**:
   ```javascript
   const printers = JSON.parse(await getPrinters());
   console.log('Available printers:', printers);
   ```

3. **Test connection**:
   ```javascript
   try {
     const result = await ping();
     console.log('Plugin connected:', result);
   } catch (error) {
     console.error('Plugin connection failed:', error);
   }
   ```

## ⚡ Performance Optimization

### Best Practices

#### 1. Optimize large file printing
```javascript
// For large PDF files, suggest printing by page batches
const largePdfPath = '/path/to/large-document.pdf';

// Print in batches, 10 pages each time
for (let startPage = 1; startPage <= totalPages; startPage += 10) {
    const endPage = Math.min(startPage + 9, totalPages);
    
    await printPdf({
        path: largePdfPath,
        printer: printerName,
        pages: `${startPage}-${endPage}`
    });
    
    // Add delay to avoid overloading the print queue
    await new Promise(resolve => setTimeout(resolve, 1000));
}
```

#### 2. HTML print optimization

```javascript
// Optimized HTML structure
const optimizedHtml = `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <style>
        /* Print styles */
        @media print {
            body { margin: 0; padding: 20px; }
            .no-print { display: none; }
            .page-break { page-break-before: always; }
        }
        
        /* Basic style */
        body {
            font-family: 'Arial', sans-serif;
            font-size: 12pt;
            line-height: 1.4;
        }
        
        table {
            width: 100%;
            border-collapse: collapse;
            margin-bottom: 20px;
        }
        
        th, td {
            border: 1px solid #000;
            padding: 8px;
            text-align: left;
        }
    </style>
</head>
<body>
    <!-- Content -->
</body>
</html>
`;
```

#### 3. Error Handling and Retries

```javascript
// Print function with retry logic
async function printWithRetry(printFunction, options, maxRetries = 3) {
    for (let attempt = 1; attempt <= maxRetries; attempt++) {
        try {
            const result = await printFunction(options);
            return result;
        } catch (error) {
             console.warn(`Print attempt ${attempt} failed:`, error.message);
            
            if (attempt === maxRetries) {
                throw new Error(`Print failed after ${maxRetries} retries: ${error.message}`);
            }
            
            // Exponential backoff delay
            const delay = Math.pow(2, attempt) * 1000;
            await new Promise(resolve => setTimeout(resolve, delay));
        }
    }
}

// Usage
try {
    await printWithRetry(printPdf, {
        path: '/path/to/document.pdf',
        printer: 'HP LaserJet'
    });
} catch (error) {
    console.error('Final print failed:', error);
}
```

#### 4. Print Queue Management 

```javascript
class PrintQueue {
    constructor(maxConcurrent = 2) {
        this.queue = [];
        this.running = [];
        this.maxConcurrent = maxConcurrent;
    }
    
    async add(printTask) {
        return new Promise((resolve, reject) => {
            this.queue.push({ task: printTask, resolve, reject });
            this.process();
        });
    }
    
    async process() {
        if (this.running.length >= this.maxConcurrent || this.queue.length === 0) {
            return;
        }
        
        const { task, resolve, reject } = this.queue.shift();
        const promise = task().then(resolve).catch(reject);
        
        this.running.push(promise);
        
        promise.finally(() => {
            this.running = this.running.filter(p => p !== promise);
            this.process();
        });
    }
}

// Using the print queue
const printQueue = new PrintQueue(2); // up to 2 concurrent print jobs

// Add print tasks
const files = ['file1.pdf', 'file2.pdf', 'file3.pdf'];
for (const file of files) {
    printQueue.add(() => printPdf({ path: file, printer: 'Default' }));
}
```

### Memory Management

- **Avoid storing large amounts of HTML in memory**
- **Clean up print job references promptly**
- **Use streaming for large file handling**
- **Regularly monitor and clear the print queue**

## 🤝 Contributing

Issues and Pull Requests are welcome!

## 📄 License

MIT License

## 🙏 Acknowledgements

This project is based on:

- [Alfian Lensun's Original Plugin Repository](https://github.com/alfianlensundev/tauri-plugin-printer)
- [adao99's Fork for Tauri V2 Beta](https://github.com/adao99/tauri-plugin-printer-v2)
- [chen-collab's](https://github.com/chen-collab/tauri-plugin-printer)

Thanks to the original authors!

## 📝 Changelog

### v0.1.3 (current)
- 🛠️ Update README.md file

### v0.1.2
- 🛠️ Update README.md file

### v0.1.1
- 🔧 Fixes TypeScript types in the API

### v0.1.0
- 🎉 Initial release of this fork
- 🔧 Fixes TypeScript types in the API
- 🛠️ Updates comments from Chinese to English
- 📦 Looks for `wkhtmltopdf` in the `bin` folder inside `src-tauri`