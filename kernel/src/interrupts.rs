// kernel/src/interrupts.rs

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use spin::Mutex;
use crate::pic::{self, KEYBOARD_INTERRUPT_ID, TIMER_INTERRUPT_ID}; // ✨ 新增 TIMER_INTERRUPT_ID
use crate::keyboard::{self, KeyboardState};
use crate::{print, println, set_text_color, handle_backspace, handle_shell_char, SHELL};
use crate::writer::Color;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        
        // 异常处理
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        
        // 硬件中断处理
        idt[KEYBOARD_INTERRUPT_ID as usize].set_handler_fn(keyboard_interrupt_handler);
        idt[TIMER_INTERRUPT_ID as usize].set_handler_fn(timer_interrupt_handler); // ✨ 新增定时器中断
        
        idt
    };
    
    // 全局键盘状态
    static ref KEYBOARD_STATE: Mutex<KeyboardState> = Mutex::new(KeyboardState::new());
}

/// 初始化中断系统
pub fn init() {
    println!("Setting up IDT...");
    IDT.load();
    
    println!("Initializing PIC...");
    pic::init();
    
    println!("Enabling keyboard interrupt...");
    pic::enable_keyboard();
    
    // ✨ 启用定时器中断
    println!("Enabling timer interrupt...");
    pic::enable_timer();
    
    println!("Enabling interrupts...");
    x86_64::instructions::interrupts::enable();
    
    println!("Interrupt system ready!");
}

/// 断点异常处理程序
extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT");
}

/// ✨ 定时器中断处理程序 - 新增
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // 更新系统时间
    crate::time::tick();
    
    // 发送中断结束信号
    pic::end_of_interrupt(TIMER_INTERRUPT_ID);
}

/// 键盘中断处理程序 - Shell 模式
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    
    // 从键盘控制器读取扫描码
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    
    // 获取键盘状态
    let mut keyboard_state = KEYBOARD_STATE.lock();
    
    // 处理修饰键 (Shift, Ctrl, Alt, Caps Lock)
    if keyboard::handle_modifier_key(&mut keyboard_state, scancode) {
        // 如果是 Caps Lock，显示状态变化
        if scancode == 0x3A { // Caps Lock 键
            set_text_color(Color::YELLOW, Color::BLACK);
            if keyboard_state.caps_lock {
                print!(" [CAPS ON] ");
            } else {
                print!(" [CAPS OFF] ");
            }
            set_text_color(Color::WHITE, Color::BLACK);
        }
        
        // 修饰键处理完成，发送中断结束信号并返回
        pic::end_of_interrupt(KEYBOARD_INTERRUPT_ID);
        return;
    }
    
    // 只处理按下的键（忽略释放事件）
    if scancode < 0x80 {
        // 尝试转换为字符，考虑 Shift 和 Caps Lock 状态
        if let Some(ch) = keyboard::scancode_to_char(scancode, keyboard_state.shift_pressed, keyboard_state.caps_lock) {
            // 处理特殊字符
            match ch {
                '\x08' => { // 退格键
                    // 检查 Shell 是否允许退格
                    if SHELL.lock().can_backspace() {
                        // 发送给 Shell 处理
                        handle_shell_char('\x08');
                        // 同时在屏幕上执行退格
                        handle_backspace();
                    }
                    // 如果不能退格，忽略这个按键
                },
                '\n' => { // 回车键
                    // 发送给 Shell 处理命令
                    handle_shell_char('\n');
                },
                '\t' => { // Tab 键
                    // Tab 仍然直接输出，不加入缓冲区
                    set_text_color(Color::YELLOW, Color::BLACK);
                    print!(">   "); // > + 3 个空格 = 4 个字符宽度的缩进
                    set_text_color(Color::WHITE, Color::BLACK);
                },
                ch => { // 普通字符
                    // 发送给 Shell 缓冲区
                    handle_shell_char(ch);
                    
                    // 在屏幕上显示字符（带颜色）
                    if keyboard_state.caps_lock && ch.is_ascii_alphabetic() {
                        set_text_color(Color::RED, Color::BLACK);   // Caps Lock 字母用红色
                    } else if keyboard_state.shift_pressed {
                        set_text_color(Color::BLUE, Color::BLACK);  // Shift + 字符用蓝色
                    } else {
                        set_text_color(Color::GREEN, Color::BLACK); // 普通字符用绿色
                    }
                    print!("{}", ch);
                    set_text_color(Color::WHITE, Color::BLACK);
                }
            }
        } else {
            // 未知键，显示扫描码
            set_text_color(Color::YELLOW, Color::BLACK);
            print!("[{}]", scancode);
            set_text_color(Color::WHITE, Color::BLACK);
        }
    }
    
    // 发送中断结束信号
    pic::end_of_interrupt(KEYBOARD_INTERRUPT_ID);
}