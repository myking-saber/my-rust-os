use std::env;
use std::path::PathBuf;

fn main() {
    // 獲取核心二進制文件的路徑
    let kernel_path = env::var_os("CARGO_BIN_FILE_KERNEL_kernel")
        .expect("CARGO_BIN_FILE_KERNEL_kernel env var not set");
    
    // 將 PathBuf 轉換為字符串
    let kernel_path = PathBuf::from(kernel_path);
    
    let out_dir = out_dir();
    
    // 定義磁盤映像的路徑
    let bios_path = out_dir.join("bios.img");
    let uefi_path = out_dir.join("uefi.img");
    
    // 創建 BIOS 磁盤映像
    bootloader::BiosBoot::new(&kernel_path)
        .create_disk_image(&bios_path)
        .expect("Failed to create BIOS disk image");
    
    // 創建 UEFI 磁盤映像
    bootloader::UefiBoot::new(&kernel_path)
        .create_disk_image(&uefi_path)
        .expect("Failed to create UEFI disk image");
    
    println!("BIOS bootable disk image: {}", bios_path.display());
    println!("UEFI bootable disk image: {}", uefi_path.display());
    
    // 設置默認運行的映像（BIOS）
    println!("cargo:rustc-env=BIOS_DISK_IMAGE={}", bios_path.display());
}

fn out_dir() -> PathBuf {
    PathBuf::from(env::var_os("OUT_DIR").unwrap())
}