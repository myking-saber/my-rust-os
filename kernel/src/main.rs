#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)] 

use bootloader_api::{entry_point, BootInfo};
use spin::Mutex;

mod font;
mod writer;
mod interrupts; 
mod pic;
mod keyboard;
mod shell;  // ✨ 新增 shell 模塊

use writer::{Writer, Color};
use shell::Shell;

entry_point!(kernel_main);

// 全局 Writer 實例
pub static WRITER: Mutex<Option<Writer>> = Mutex::new(None);

// 全局 Shell 實例 ✨ 新增
pub static SHELL: Mutex<Shell> = Mutex::new(Shell::new());

/// 初始化全局 Writer
fn init_writer(boot_info: &'static mut BootInfo) {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();
        let mut writer = Writer::new(buffer, info);
        writer.clear_screen();
        *WRITER.lock() = Some(writer);
    }
}

/// 打印函數的內部實現
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    
    if let Some(ref mut writer) = WRITER.lock().as_mut() {
        writer.write_fmt(args).unwrap();
    }
}

/// print! 宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

/// println! 宏
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// 設置文字顏色
pub fn set_text_color(fg: Color, bg: Color) {
    if let Some(ref mut writer) = WRITER.lock().as_mut() {
        writer.set_fg_color(fg);
        writer.set_bg_color(bg);
    }
}

/// 處理退格鍵 - 刪除前一個字符
pub fn handle_backspace() {
    if let Some(ref mut writer) = WRITER.lock().as_mut() {
        writer.backspace();
    }
}

/// Shell 字符處理函數 ✨ 新增
pub fn handle_shell_char(ch: char) {
    SHELL.lock().handle_char(ch);
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // 初始化顯示系統
    init_writer(boot_info);
    
    set_text_color(Color::CYAN, Color::BLACK);
    println!("=== Rust OS v0.2.1 - Shell Mode ===");
    set_text_color(Color::WHITE, Color::BLACK);
    
    // 分步初始化中斷系統
    println!("Initializing interrupt system...");
    interrupts::init();
    
    set_text_color(Color::GREEN, Color::BLACK);
    println!("✓ All systems initialized!");
    set_text_color(Color::WHITE, Color::BLACK);
    
    println!();
    println!("Welcome to Rust OS Interactive Shell!");
    println!("- Type commands and press Enter to execute");
    println!("- Use Backspace to edit your input");
    println!("- All keyboard features still work");
    println!();
    
    set_text_color(Color::YELLOW, Color::BLACK);
    println!("Type 'help' to see available commands.");
    set_text_color(Color::WHITE, Color::BLACK);
    println!();
    
    // 顯示第一個提示符
    SHELL.lock().show_prompt();
    
    // 主循環 - 等待鍵盤中斷
    loop {
        x86_64::instructions::hlt(); // 等待中斷
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    set_text_color(Color::RED, Color::BLACK);
    println!();
    println!("KERNEL PANIC!");
    println!("=============");
    println!("{}", info);
    
    loop {
        x86_64::instructions::hlt();
    }
}