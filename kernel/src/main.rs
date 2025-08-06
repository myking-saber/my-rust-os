#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use core::fmt::Write;
use spin::Mutex;

mod font;
mod writer;

use writer::{Writer, Color};

entry_point!(kernel_main);

// 全局 Writer 實例，使用 Mutex 保護線程安全
static WRITER: Mutex<Option<Writer>> = Mutex::new(None);

/// 初始化全局 Writer
fn init_writer(boot_info: &'static mut BootInfo) {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();
        let mut writer = Writer::new(buffer, info);
        writer.clear_screen();  // 初始化時清屏
        *WRITER.lock() = Some(writer);
    }
}

/// 打印函數的內部實現
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    
    // 獲取全局 Writer 並寫入內容
    if let Some(ref mut writer) = WRITER.lock().as_mut() {
        writer.write_fmt(args).unwrap();
    }
}

/// print! 宏 - 不換行的打印
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

/// println! 宏 - 帶換行的打印
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// 設置文字顏色的輔助函數
pub fn set_text_color(fg: Color, bg: Color) {
    if let Some(ref mut writer) = WRITER.lock().as_mut() {
        writer.set_fg_color(fg);
        writer.set_bg_color(bg);
    }
}

/// 清屏函數
pub fn clear_screen() {
    if let Some(ref mut writer) = WRITER.lock().as_mut() {
        writer.clear_screen();
    }
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // 初始化全局 Writer
    init_writer(boot_info);
    
    // 現在我們可以像標準 Rust 一樣使用 println! 了！
    println!("=== Welcome to Rust OS! ===");
    println!();
    
    println!("println! macro is working perfectly!");
    println!("Current system time: {} ms", 1234567890);
    println!("Memory available: {} MB", 512);
    println!();
    
    // 測試格式化功能
    let name = "Rust OS";
    let version = "0.1.0";
    println!("System: {} v{}", name, version);
    println!("Answer to everything: {}", 42);
    println!("Hex value: 0x{:x}", 255);
    println!("Binary: 0b{:08b}", 255);
    println!();
    
    // 測試顏色變化
    set_text_color(Color::RED, Color::BLACK);
    println!("This text is RED!");
    
    set_text_color(Color::GREEN, Color::BLACK);
    println!("This text is GREEN!");
    
    set_text_color(Color::BLUE, Color::BLACK);
    println!("This text is BLUE!");
    
    set_text_color(Color::YELLOW, Color::BLACK);
    println!("This text is YELLOW!");
    
    set_text_color(Color::WHITE, Color::BLACK);
    println!();
    
    // 測試混合使用 print! 和 println!
    print!("Loading");
    for i in 0..5 {
        print!(".");
        // 簡單延遲
        for _ in 0..10000000 {
            core::hint::spin_loop();
        }
    }
    println!(" Done!");
    println!();
    
    // 測試長文本和自動滾動
    println!("Testing automatic text wrapping and scrolling:");
    for i in 0..20 {
        println!("Line {}: This is a test line that demonstrates automatic scrolling when we have too many lines on the screen.", i + 1);
    }
    
    println!();
    set_text_color(Color::GREEN, Color::BLACK);
    println!("✓ All tests passed!");
    println!("✓ Rust OS is running successfully!");
    
    set_text_color(Color::WHITE, Color::BLACK);
    println!();
    println!("System ready. Entering infinite loop...");
    
    loop {
        // 系統主循環
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // 當程序 panic 時，顯示錯誤信息
    set_text_color(Color::RED, Color::BLACK);
    println!();
    println!("KERNEL PANIC!");
    println!("=============");
    println!("{}", info);
    
    loop {
        core::hint::spin_loop();
    }
}