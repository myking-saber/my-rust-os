// kernel/src/shell.rs

use crate::writer::Color;
use crate::{print, println, set_text_color};

/// 輸入緩衝區最大長度
const INPUT_BUFFER_SIZE: usize = 256;
/// 提示符長度（"rust-os> "）
const PROMPT_LENGTH: usize = 9;

/// Shell 狀態
pub struct Shell {
    input_buffer: [u8; INPUT_BUFFER_SIZE],
    buffer_pos: usize,
    cursor_at_prompt_start: bool, // 記錄光標是否在提示符開始位置
}

impl Shell {
    /// 創建新的 Shell 實例
    pub const fn new() -> Shell {
        Shell {
            input_buffer: [0; INPUT_BUFFER_SIZE],
            buffer_pos: 0,
            cursor_at_prompt_start: false,
        }
    }

    /// 處理字符輸入
    pub fn handle_char(&mut self, ch: char) {
        match ch {
            '\n' => {
                // Enter 鍵 - 處理當前命令
                self.process_command();
            },
            '\x08' => {
                // 退格鍵 - 從緩衝區刪除字符
                self.handle_backspace();
            },
            ch if ch.is_ascii() && !ch.is_control() => {
                // 普通字符 - 添加到緩衝區
                self.add_char(ch);
            },
            _ => {
                // 忽略其他控制字符
            }
        }
    }

    /// 添加字符到緩衝區
    fn add_char(&mut self, ch: char) {
        if self.buffer_pos < INPUT_BUFFER_SIZE - 1 {
            self.input_buffer[self.buffer_pos] = ch as u8;
            self.buffer_pos += 1;
        } else {
            // 緩衝區已滿，顯示警告
            set_text_color(Color::RED, Color::BLACK);
            print!(" [BUFFER FULL] ");
            set_text_color(Color::WHITE, Color::BLACK);
        }
    }

    /// 處理退格
    fn handle_backspace(&mut self) {
        if self.buffer_pos > 0 {
            self.buffer_pos -= 1;
            self.input_buffer[self.buffer_pos] = 0;
            // 允許退格，但會在中斷處理程序中檢查是否到達提示符
        }
    }

    /// 處理命令執行
    fn process_command(&mut self) {
        // 創建一個臨時緩衝區來避免借用衝突
        let mut temp_buffer = [0u8; INPUT_BUFFER_SIZE];
        let buffer_len = self.buffer_pos;
        
        // 複製當前輸入到臨時緩衝區
        for i in 0..buffer_len {
            temp_buffer[i] = self.input_buffer[i];
        }
        
        // 換行
        println!();
        
        // 如果命令不為空，處理它
        if buffer_len > 0 {
            // 從臨時緩衝區創建字符串切片
            if let Ok(command_str) = core::str::from_utf8(&temp_buffer[..buffer_len]) {
                let command = command_str.trim();
                if !command.is_empty() {
                    self.execute_command(command);
                }
            }
        }
        
        // 清空緩衝區並顯示新提示符
        self.clear_buffer();
        self.show_prompt();
    }

    /// 獲取當前輸入
    fn get_current_input(&self) -> &str {
        let valid_bytes = &self.input_buffer[..self.buffer_pos];
        // 使用 from_utf8 安全轉換，失敗時返回空字符串
        core::str::from_utf8(valid_bytes).unwrap_or("")
    }

    /// 清空輸入緩衝區
    fn clear_buffer(&mut self) {
        self.buffer_pos = 0;
        for i in 0..INPUT_BUFFER_SIZE {
            self.input_buffer[i] = 0;
        }
    }

    /// 執行命令
    fn execute_command(&mut self, command: &str) {
        // 使用 no_std 兼容的方式解析命令
        let mut parts = command.split_whitespace();
        
        if let Some(cmd) = parts.next() {
            match cmd {
                "help" => self.cmd_help(),
                "clear" => self.cmd_clear(),
                "version" => self.cmd_version(),
                "echo" => self.cmd_echo(parts),
                _ => {
                    // 未知命令
                    set_text_color(Color::RED, Color::BLACK);
                    println!("Unknown command: '{}'", cmd);
                    set_text_color(Color::YELLOW, Color::BLACK);
                    println!("Type 'help' for available commands.");
                    set_text_color(Color::WHITE, Color::BLACK);
                }
            }
        }
    }

    /// 顯示提示符
    pub fn show_prompt(&mut self) {
        set_text_color(Color::GREEN, Color::BLACK);
        print!("rust-os");
        set_text_color(Color::WHITE, Color::BLACK);
        print!("> ");
        self.cursor_at_prompt_start = true; // 標記光標在提示符後
    }

    /// 檢查是否可以退格（不能刪除提示符）
    pub fn can_backspace(&self) -> bool {
        self.buffer_pos > 0
    }

    // === 基本命令實現 ===

    /// help 命令
    fn cmd_help(&self) {
        set_text_color(Color::CYAN, Color::BLACK);
        println!("=== Rust OS Shell Commands ===");
        set_text_color(Color::WHITE, Color::BLACK);
        println!("help              - Show this help message");
        println!("clear             - Clear the screen");
        println!("version           - Show OS version information");
        println!("echo <message>    - Display a message");
        println!();
        set_text_color(Color::YELLOW, Color::BLACK);
        println!("Examples:");
        println!("  echo Hello World!");
        println!("  echo \"Multiple words\"");
        println!();
        println!("Tip: All keyboard features still work!");
        println!("- Shift/Caps Lock for uppercase");  
        println!("- Backspace to delete");
        println!("- Tab for indentation");
        set_text_color(Color::WHITE, Color::BLACK);
    }

    /// clear 命令
    fn cmd_clear(&mut self) {
        // 使用全局 Writer 清屏
        if let Some(ref mut writer) = crate::WRITER.lock().as_mut() {
            writer.clear_screen();
        }
        
        // 顯示歡迎信息
        set_text_color(Color::CYAN, Color::BLACK);
        println!("=== Rust OS v0.2.1 - Shell ===");
        set_text_color(Color::WHITE, Color::BLACK);
        println!("Screen cleared. Type 'help' for commands.");
    }

    /// version 命令
    fn cmd_version(&self) {
        set_text_color(Color::CYAN, Color::BLACK);
        println!("=== Rust OS Version Information ===");
        set_text_color(Color::WHITE, Color::BLACK);
        println!("OS Name:      Rust OS");
        println!("Version:      0.2.1");
        println!("Architecture: x86_64");
        println!("Build:        Debug");
        println!();
        set_text_color(Color::GREEN, Color::BLACK);
        println!("Features:");
        set_text_color(Color::WHITE, Color::BLACK);
        println!("✓ Graphical framebuffer output");
        println!("✓ 8259 PIC interrupt handling"); 
        println!("✓ PS/2 keyboard driver");
        println!("✓ Full keyboard layout support");
        println!("✓ Interactive shell interface");
        println!("✓ Command parsing and execution");
    }

    /// echo 命令 - 顯示文字
    fn cmd_echo(&self, mut args: core::str::SplitWhitespace) {
        set_text_color(Color::WHITE, Color::BLACK);
        
        // 手動拼接參數，用空格分隔
        let mut first = true;
        for arg in args {
            if !first {
                print!(" ");
            }
            print!("{}", arg);
            first = false;
        }
        println!(); // 最後換行
    }
}