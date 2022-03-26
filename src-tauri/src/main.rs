#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use qrcode_generator::QrCodeEcc;
use base64;
use native_dialog;

#[tauri::command]
async fn create_qr_code(text: String) -> String {
    let result: Vec<u8> = qrcode_generator::to_png_to_vec(text, QrCodeEcc::Low, 1024).unwrap();

    base64::encode(result)
}

#[tauri::command]
async fn save_qr_code(text: String) {
    let path = native_dialog::FileDialog::new()
        .set_location("~/Desktop")
        .set_filename("qrcode.jpg")
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .show_save_single_file()
        .unwrap();
    
    match path {
        Some(src)=>qrcode_generator::to_png_to_file(text,QrCodeEcc::Low,1024,src).unwrap(),
        None => (),
    }
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_qr_code, save_qr_code])    
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
