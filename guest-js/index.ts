import { invoke } from '@tauri-apps/api/core'

export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:printer-wkhtml-bin|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function getPrinters(): Promise<string> {
  return await invoke<string>('plugin:printer-wkhtml-bin|get_printers');
}

export async function getPrinterByName(printerName: string): Promise<string> {
  return await invoke<string>('plugin:printer-wkhtml-bin|get_printers_by_name', {
    printername: printerName,
  });
}

export interface PrintPdfOptions {
  id: string;
  path: string;
  printer_setting: string;
  remove_after_print: boolean;
}

export async function printPdf(options: PrintPdfOptions): Promise<string> {
  console.log('Print configuration pdf:', options);
  return await invoke<string>('plugin:printer-wkhtml-bin|print_pdf', {
    id: options.id,
    path: options.path,
    printer_setting: options.printer_setting,
    remove_after_print: options.remove_after_print,
  });
}

export interface PrintMargin {
  top: number;
  bottom: number;
  left: number;
  right: number;
  unit: string;
}

export interface PrintHtmlOptions {
  html: string
  printer: string 
  page_width?: number 
  page_height?: number  
  page_size?: string 
  orientation?: "portrait" | "landscape"  
  margin?: PrintMargin
  quality?: string
  grayscale?: string 
  copies?: number
}

export async function printHtml(options: PrintHtmlOptions): Promise<string> {
  console.log('Print configuration html:', options);
  return await invoke<string>('plugin:printer-wkhtml-bin|print_html', {
    options: options
  });
}
