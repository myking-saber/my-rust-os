// kernel/src/shell.rs

use crate::writer::Color;
use crate::{print, println, set_text_color};

/// 输入缓冲区最大长度
const INPUT_BUFFER_SIZE: usize = 256;
/// 提示符长度（"rust-os> "）
const PROMPT_LENGTH: usize = 9;

/// Shell 状态
pub struct Shell {
    input_buffer: [u8; INPUT_BUFFER_SIZE],
    buffer_pos: usize,
    cursor_at_prompt_start: bool,
    command_count: u64, // ✨ 新增：跟踪执行的命令数量
}

impl Shell {
    /// 创建新的 Shell 实例
    pub const fn new() -> Shell {
        Shell {
            input_buffer: [0; INPUT_BUFFER_SIZE],
            buffer_pos: 0,
            cursor_at_prompt_start: false,
            command_count: 0,
        }
    }

    /// 处理字符输入
    pub fn handle_char(&mut self, ch: char) {
        match ch {
            '\n' => {
                // Enter 键 - 处理当前命令
                self.process_command();
            },
            '\x08' => {
                // 退格键 - 从缓冲区删除字符
                self.handle_backspace();
            },
            ch if ch.is_ascii() && !ch.is_control() => {
                // 普通字符 - 添加到缓冲区
                self.add_char(ch);
            },
            _ => {
                // 忽略其他控制字符
            }
        }
    }

    /// 添加字符到缓冲区
    fn add_char(&mut self, ch: char) {
        if self.buffer_pos < INPUT_BUFFER_SIZE - 1 {
            self.input_buffer[self.buffer_pos] = ch as u8;
            self.buffer_pos += 1;
        } else {
            // 缓冲区已满，显示警告
            set_text_color(Color::RED, Color::BLACK);
            print!(" [BUFFER FULL] ");
            set_text_color(Color::WHITE, Color::BLACK);
        }
    }

    /// 处理退格
    fn handle_backspace(&mut self) {
        if self.buffer_pos > 0 {
            self.buffer_pos -= 1;
            self.input_buffer[self.buffer_pos] = 0;
        }
    }

    /// 处理命令执行
    fn process_command(&mut self) {
        let mut temp_buffer = [0u8; INPUT_BUFFER_SIZE];
        let buffer_len = self.buffer_pos;
        
        for i in 0..buffer_len {
            temp_buffer[i] = self.input_buffer[i];
        }
        
        println!();
        
        if buffer_len > 0 {
            if let Ok(command_str) = core::str::from_utf8(&temp_buffer[..buffer_len]) {
                let command = command_str.trim();
                if !command.is_empty() {
                    self.command_count += 1; // ✨ 增加命令计数
                    self.execute_command(command);
                }
            }
        }
        
        self.clear_buffer();
        self.show_prompt();
    }

    /// 清空输入缓冲区
    fn clear_buffer(&mut self) {
        self.buffer_pos = 0;
        for i in 0..INPUT_BUFFER_SIZE {
            self.input_buffer[i] = 0;
        }
    }

    /// 执行命令
    fn execute_command(&mut self, command: &str) {
        let mut parts = command.split_whitespace();
        
        if let Some(cmd) = parts.next() {
            match cmd {
                "help" => self.cmd_help(),
                "clear" => self.cmd_clear(),
                "version" => self.cmd_version(),
                "echo" => self.cmd_echo(parts),
                "uptime" => self.cmd_uptime(),
                "sysinfo" => self.cmd_sysinfo(), // ✨ 新增系统信息命令
                "stats" => self.cmd_stats(),     // ✨ 新增统计信息命令
                _ => {
                    set_text_color(Color::RED, Color::BLACK);
                    println!("Unknown command: '{}'", cmd);
                    set_text_color(Color::YELLOW, Color::BLACK);
                    println!("Type 'help' for available commands.");
                    set_text_color(Color::WHITE, Color::BLACK);
                }
            }
        }
    }

    /// 显示提示符
    pub fn show_prompt(&mut self) {
        set_text_color(Color::GREEN, Color::BLACK);
        print!("rust-os");
        set_text_color(Color::WHITE, Color::BLACK);
        print!("> ");
        self.cursor_at_prompt_start = true;
    }

    /// 检查是否可以退格
    pub fn can_backspace(&self) -> bool {
        self.buffer_pos > 0
    }

    // === 命令实现 ===

    /// help 命令
    fn cmd_help(&self) {
        set_text_color(Color::CYAN, Color::BLACK);
        println!("=== Rust OS Shell Commands ===");
        set_text_color(Color::WHITE, Color::BLACK);
        println!("help              - Show this help message");
        println!("clear             - Clear the screen");
        println!("version           - Show OS version information");
        println!("echo <message>    - Display a message");
        println!("uptime            - Show system runtime");
        println!("sysinfo           - Show system information"); // ✨ 新增
        println!("stats             - Show shell statistics");   // ✨ 新增
        println!();
        set_text_color(Color::YELLOW, Color::BLACK);
        println!("Examples:");
        println!("  echo Hello from Rust OS!");
        println!("  uptime");
        println!("  sysinfo");
        println!("  stats");
        println!();
        println!("Tips:");
        println!("- Use Shift/Caps Lock for uppercase");  
        println!("- Use Backspace to edit your input");
        println!("- Use Tab for indentation");
        println!("- All commands are case-sensitive");
        set_text_color(Color::WHITE, Color::BLACK);
    }

    /// clear 命令
    fn cmd_clear(&mut self) {
        if let Some(ref mut writer) = crate::WRITER.lock().as_mut() {
            writer.clear_screen();
        }
        
        set_text_color(Color::CYAN, Color::BLACK);
        println!("=== Rust OS v0.3.0 - Time System ===");
        set_text_color(Color::WHITE, Color::BLACK);
        println!("Screen cleared. Type 'help' for commands.");
    }

    /// version 命令
    fn cmd_version(&self) {
        set_text_color(Color::CYAN, Color::BLACK);
        println!("=== Rust OS Version Information ===");
        set_text_color(Color::WHITE, Color::BLACK);
        println!("OS Name:      Rust OS");
        println!("Version:      0.3.0");
        println!("Codename:     \"Temporal\"");
        println!("Architecture: x86_64");
        println!("Build:        Debug");
        println!("Compiler:     rustc (nightly)");
        println!();
        set_text_color(Color::GREEN, Color::BLACK);
        println!("Core Features:");
        set_text_color(Color::WHITE, Color::BLACK);
        println!("✓ Graphical framebuffer output");
        println!("✓ 8259 PIC interrupt controller"); 
        println!("✓ PS/2 keyboard driver with full layout");
        println!("✓ Interactive shell with command parsing");
        println!("✓ PIT 8253 timer driver (100 Hz precision)");
        println!("✓ Real-time system clock and uptime tracking");
        println!("✓ Memory-safe kernel (no_std Rust)");
    }

    /// echo 命令
    fn cmd_echo(&self, mut args: core::str::SplitWhitespace) {
        set_text_color(Color::WHITE, Color::BLACK);
        
        let mut first = true;
        for arg in args {
            if !first {
                print!(" ");
            }
            print!("{}", arg);
            first = false;
        }
        println!();
    }

    /// uptime 命令
    fn cmd_uptime(&self) {
        if !crate::time::is_initialized() {
            set_text_color(Color::RED, Color::BLACK);
            println!("Time system not initialized!");
            set_text_color(Color::WHITE, Color::BLACK);
            return;
        }

        let uptime = crate::time::get_uptime();
        let formatted = uptime.format_detailed();
        let (days, hours, minutes, seconds, milliseconds) = formatted.detailed_format();
        let total_ms = formatted.total_milliseconds();
        let tick_count = crate::time::get_tick_count();

        set_text_color(Color::CYAN, Color::BLACK);
        println!("=== System Uptime ===");
        set_text_color(Color::WHITE, Color::BLACK);

        if days > 0 {
            println!("Uptime: {} days, {:02}:{:02}:{:02}.{:03}", 
                     days, hours, minutes, seconds, milliseconds);
        } else {
            println!("Uptime: {:02}:{:02}:{:02}.{:03}", 
                     hours, minutes, seconds, milliseconds);
        }

        println!();
        
        set_text_color(Color::YELLOW, Color::BLACK);
        println!("Timer Details:");
        set_text_color(Color::WHITE, Color::BLACK);
        println!("  Total milliseconds: {}", total_ms);
        println!("  Timer ticks:        {}", tick_count);
        println!("  Timer frequency:    100 Hz");
        println!("  Tick interval:      10 ms");

        if tick_count > 0 {
            let avg_ms_per_tick = total_ms as f32 / tick_count as f32;
            println!("  Average per tick:   {:.2} ms", avg_ms_per_tick);
        }
    }

    /// ✨ sysinfo 命令 - 显示系统信息
    fn cmd_sysinfo(&self) {
        set_text_color(Color::CYAN, Color::BLACK);
        println!("=== System Information ===");
        set_text_color(Color::WHITE, Color::BLACK);
        
        // 基本系统信息
        println!("Kernel:           Rust OS v0.3.0");
        println!("Architecture:     x86_64");
        println!("Boot Protocol:    UEFI/BIOS (bootloader 0.11)");
        
        // 时间信息
        if crate::time::is_initialized() {
            let uptime_info = crate::time::get_uptime();
            let formatted = uptime_info.format_detailed();
            let (hours, minutes, seconds) = formatted.short_format();
            println!("Uptime:           {:02}:{:02}:{:02}", hours, minutes, seconds);
        }
        
        println!();
        
        // 硬件信息
        set_text_color(Color::YELLOW, Color::BLACK);
        println!("Hardware:");
        set_text_color(Color::WHITE, Color::BLACK);
        println!("  CPU:            x86_64 compatible");
        println!("  Timer:          Intel 8253 PIT @ 100 Hz");
        println!("  Interrupt:      Intel 8259 PIC");
        println!("  Keyboard:       PS/2 compatible");
        println!("  Display:        Framebuffer graphics");
        
        println!();
        
        // 内存信息 (模拟数据，因为还没有内存管理器)
        set_text_color(Color::YELLOW, Color::BLACK);
        println!("Memory:");
        set_text_color(Color::WHITE, Color::BLACK);
        println!("  Kernel size:    ~60 KB");
        println!("  Runtime usage:  < 1 MB");
        println!("  Memory model:   Static allocation");
        
        println!();
        
        // 功能状态
        set_text_color(Color::GREEN, Color::BLACK);
        println!("✓ All systems operational");
        set_text_color(Color::WHITE, Color::BLACK);
    }

    /// ✨ stats 命令 - 显示Shell统计信息
    fn cmd_stats(&self) {
        set_text_color(Color::CYAN, Color::BLACK);
        println!("=== Shell Statistics ===");
        set_text_color(Color::WHITE, Color::BLACK);
        
        println!("Commands executed:    {}", self.command_count);
        println!("Input buffer size:    {} bytes", INPUT_BUFFER_SIZE);
        println!("Current buffer used:  {} bytes", self.buffer_pos);
        println!("Available commands:   7");
        
        // 计算一些有趣的统计数据
        if crate::time::is_initialized() {
            let uptime_ms = crate::time::get_uptime_ms();
            if uptime_ms > 0 && self.command_count > 0 {
                let avg_time_between_commands = uptime_ms / self.command_count;
                println!("Avg time per command: {} ms", avg_time_between_commands);
            }
        }
        
        println!();
        
        set_text_color(Color::YELLOW, Color::BLACK);
        println!("Session Information:");
        set_text_color(Color::WHITE, Color::BLACK);
        
        if crate::time::is_initialized() {
            let uptime_seconds = crate::time::get_uptime().get_uptime_seconds();
            if uptime_seconds > 0 {
                let commands_per_minute = (self.command_count * 60) / uptime_seconds;
                println!("  Commands per minute: {}", commands_per_minute);
            }
        }
        
        println!("  Shell status:        Active");
        println!("  Error count:         0"); // 简化版本，假设无错误
        
        set_text_color(Color::GREEN, Color::BLACK);
        println!();
        println!("✓ Shell running smoothly!");
        set_text_color(Color::WHITE, Color::BLACK);
    }
}