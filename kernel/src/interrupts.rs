// kernel/src/interrupts.rs

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use spin::Mutex;
use crate::pic::{self, KEYBOARD_INTERRUPT_ID};
use crate::keyboard::{self, KeyboardState};
use crate::{print, println, set_text_color, handle_backspace, handle_shell_char, SHELL};
use crate::writer::Color;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        
        // 異常處理
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        
        // 硬件中斷處理
        idt[KEYBOARD_INTERRUPT_ID as usize].set_handler_fn(keyboard_interrupt_handler);
        
        idt
    };
    
    // 全局鍵盤狀態
    static ref KEYBOARD_STATE: Mutex<KeyboardState> = Mutex::new(KeyboardState::new());
}

/// 初始化中斷系統
pub fn init() {
    println!("Setting up IDT...");
    IDT.load();
    
    println!("Initializing PIC...");
    pic::init();
    
    println!("Enabling keyboard interrupt...");
    pic::enable_keyboard();
    
    println!("Enabling interrupts...");
    x86_64::instructions::interrupts::enable();
    
    println!("Interrupt system ready!");
}

/// 斷點異常處理程序
extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT");
}

/// 鍵盤中斷處理程序 - 修改為 Shell 模式
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    
    // 從鍵盤控制器讀取掃描碼
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    
    // 獲取鍵盤狀態
    let mut keyboard_state = KEYBOARD_STATE.lock();
    
    // 處理修飾鍵 (Shift, Ctrl, Alt, Caps Lock)
    if keyboard::handle_modifier_key(&mut keyboard_state, scancode) {
        // 如果是 Caps Lock，顯示狀態變化
        if scancode == 0x3A { // Caps Lock 鍵
            set_text_color(Color::YELLOW, Color::BLACK);
            if keyboard_state.caps_lock {
                print!(" [CAPS ON] ");
            } else {
                print!(" [CAPS OFF] ");
            }
            set_text_color(Color::WHITE, Color::BLACK);
        }
        
        // 修飾鍵處理完成，發送中斷結束信號並返回
        pic::end_of_interrupt(KEYBOARD_INTERRUPT_ID);
        return;
    }
    
    // 只處理按下的鍵（忽略釋放事件）
    if scancode < 0x80 {
        // 嘗試轉換為字符，考慮 Shift 和 Caps Lock 狀態
        if let Some(ch) = keyboard::scancode_to_char(scancode, keyboard_state.shift_pressed, keyboard_state.caps_lock) {
            // 處理特殊字符
            match ch {
                '\x08' => { // 退格鍵
                    // 檢查 Shell 是否允許退格
                    if SHELL.lock().can_backspace() {
                        // 發送給 Shell 處理
                        handle_shell_char('\x08');
                        // 同時在屏幕上執行退格
                        handle_backspace();
                    }
                    // 如果不能退格，忽略這個按鍵
                },
                '\n' => { // 回車鍵
                    // 發送給 Shell 處理命令
                    handle_shell_char('\n');
                },
                '\t' => { // Tab 鍵
                    // Tab 仍然直接輸出，不加入緩衝區
                    set_text_color(Color::YELLOW, Color::BLACK);
                    print!(">   "); // > + 3 個空格 = 4 個字符寬度的縮進
                    set_text_color(Color::WHITE, Color::BLACK);
                },
                ch => { // 普通字符
                    // 發送給 Shell 緩衝區
                    handle_shell_char(ch);
                    
                    // 在屏幕上顯示字符（帶顏色）
                    if keyboard_state.caps_lock && ch.is_ascii_alphabetic() {
                        set_text_color(Color::RED, Color::BLACK);   // Caps Lock 字母用紅色
                    } else if keyboard_state.shift_pressed {
                        set_text_color(Color::BLUE, Color::BLACK);  // Shift + 字符用藍色
                    } else {
                        set_text_color(Color::GREEN, Color::BLACK); // 普通字符用綠色
                    }
                    print!("{}", ch);
                    set_text_color(Color::WHITE, Color::BLACK);
                }
            }
        } else {
            // 未知鍵，顯示掃描碼
            set_text_color(Color::YELLOW, Color::BLACK);
            print!("[{}]", scancode);
            set_text_color(Color::WHITE, Color::BLACK);
        }
    }
    
    // 發送中斷結束信號
    pic::end_of_interrupt(KEYBOARD_INTERRUPT_ID);
}