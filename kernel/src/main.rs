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
mod shell;
mod pit;   // ✨ 新增 PIT 模块
mod time;  // ✨ 新增 时间模块

use writer::{Writer, Color};
use shell::Shell;

entry_point!(kernel_main);

// 全局 Writer 实例
pub static WRITER: Mutex<Option<Writer>> = Mutex::new(None);

// 全局 Shell 实例
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

/// 打印函数的内部实现
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

/// 设置文字颜色
pub fn set_text_color(fg: Color, bg: Color) {
    if let Some(ref mut writer) = WRITER.lock().as_mut() {
        writer.set_fg_color(fg);
        writer.set_bg_color(bg);
    }
}

/// 处理退格键 - 删除前一个字符
pub fn handle_backspace() {
    if let Some(ref mut writer) = WRITER.lock().as_mut() {
        writer.backspace();
    }
}

/// Shell 字符处理函数
pub fn handle_shell_char(ch: char) {
    SHELL.lock().handle_char(ch);
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // 初始化显示系统
    init_writer(boot_info);
    
    set_text_color(Color::CYAN, Color::BLACK);
    println!("=== Rust OS v0.3.0 - Time System ===");
    set_text_color(Color::WHITE, Color::BLACK);
    
    // 分步初始化系统
    println!("Initializing interrupt system...");
    interrupts::init();
    
    // ✨ 初始化时间系统
    println!("Initializing PIT (Programmable Interval Timer)...");
    pit::init();
    
    let (frequency, interval_ms) = pit::get_info();
    println!("PIT configured: {} Hz, {} ms per tick", frequency, interval_ms);
    
    println!("Initializing time management...");
    time::init(interval_ms);
    
    set_text_color(Color::GREEN, Color::BLACK);
    println!("✓ All systems initialized!");
    set_text_color(Color::WHITE, Color::BLACK);
    
    println!();
    println!("Welcome to Rust OS Interactive Shell!");
    println!("- Type commands and press Enter to execute");
    println!("- New: 'uptime' command shows system runtime");
    println!("- Timer now running at {} Hz", frequency);
    println!();
    
    set_text_color(Color::YELLOW, Color::BLACK);
    println!("Type 'help' to see available commands.");
    set_text_color(Color::WHITE, Color::BLACK);
    println!();
    
    // 显示第一个提示符
    SHELL.lock().show_prompt();
    
    // 主循环 - 等待键盘中断
    loop {
        x86_64::instructions::hlt(); // 等待中断
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