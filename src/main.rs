use std::env;
use std::process::Command;

fn main() {
    // 獲取磁盤映像路徑
    let bios_path = env!("BIOS_DISK_IMAGE");
    
    // 使用 QEMU 運行
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-drive").arg(format!("format=raw,file={}", bios_path));
    
    let status = cmd.status().expect("Failed to run QEMU");
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}