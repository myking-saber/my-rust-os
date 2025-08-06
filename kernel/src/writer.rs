// kernel/src/writer.rs

use crate::font::Font8x8;
use bootloader_api::info::FrameBufferInfo;
use core::fmt;

/// 顏色定義
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
    pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };
}

/// 文字輸出管理器
pub struct Writer {
    buffer: &'static mut [u8],
    info: FrameBufferInfo,
    cursor_x: usize,
    cursor_y: usize,
    fg_color: Color,
    bg_color: Color,
    char_width: usize,
    char_height: usize,
    scale: usize,
}

impl Writer {
    /// 創建新的 Writer
    pub fn new(
        buffer: &'static mut [u8],
        info: FrameBufferInfo,
    ) -> Writer {
        let scale = 2;
        Writer {
            buffer,
            info,
            cursor_x: 0,
            cursor_y: 0,
            fg_color: Color::WHITE,
            bg_color: Color::BLACK,
            char_width: Font8x8::WIDTH * scale,
            char_height: Font8x8::HEIGHT * scale,
            scale,
        }
    }

    /// 設置前景色
    pub fn set_fg_color(&mut self, color: Color) {
        self.fg_color = color;
    }

    /// 設置背景色
    pub fn set_bg_color(&mut self, color: Color) {
        self.bg_color = color;
    }

    /// 清屏
    pub fn clear_screen(&mut self) {
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let total_pixels = self.info.width * self.info.height;
        let expected_size = total_pixels * bytes_per_pixel;

        if self.buffer.len() >= expected_size {
            for i in (0..expected_size).step_by(bytes_per_pixel) {
                if i + bytes_per_pixel <= self.buffer.len() {
                    self.write_pixel_at_offset(i, self.bg_color);
                }
            }
        }
        
        // 重置光標
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    /// 換行
    pub fn newline(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += self.char_height;
        
        // 如果超出屏幕底部，滾動屏幕
        if self.cursor_y + self.char_height > self.info.height {
            self.scroll_up();
        }
    }

    /// 向上滾動一行
    fn scroll_up(&mut self) {
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let line_bytes = self.info.width * bytes_per_pixel;
        let scroll_bytes = line_bytes * self.char_height;

        // 將所有行向上移動
        for y in 0..(self.info.height - self.char_height) {
            let src_start = (y + self.char_height) * line_bytes;
            let dst_start = y * line_bytes;
            
            for x in 0..line_bytes {
                if src_start + x < self.buffer.len() && dst_start + x < self.buffer.len() {
                    self.buffer[dst_start + x] = self.buffer[src_start + x];
                }
            }
        }

        // 清空最後幾行
        let clear_start = (self.info.height - self.char_height) * line_bytes;
        for i in (clear_start..self.buffer.len()).step_by(bytes_per_pixel) {
            if i + bytes_per_pixel <= self.buffer.len() {
                self.write_pixel_at_offset(i, self.bg_color);
            }
        }

        // 調整光標位置
        self.cursor_y = self.info.height - self.char_height;
    }

    /// 寫入單個字符
    pub fn write_char(&mut self, ch: char) {
        match ch {
            '\n' => self.newline(),
            '\r' => self.cursor_x = 0,
            ch => {
                // 檢查是否需要換行
                if self.cursor_x + self.char_width > self.info.width {
                    self.newline();
                }

                // 繪製字符
                self.draw_char(ch, self.cursor_x, self.cursor_y);
                
                // 移動光標
                self.cursor_x += self.char_width;
            }
        }
    }

    /// 寫入字符串
    pub fn write_string(&mut self, s: &str) {
        for ch in s.chars() {
            self.write_char(ch);
        }
    }

    /// 在指定位置繪製字符
    fn draw_char(&mut self, ch: char, start_x: usize, start_y: usize) {
        let char_bitmap = Font8x8::get_char(ch);
        
        for (row, &bitmap_row) in char_bitmap.iter().enumerate() {
            for col in 0..8 {
                let pixel_on = (bitmap_row >> col) & 1;
                
                // 繪製放大的像素塊
                for dy in 0..self.scale {
                    for dx in 0..self.scale {
                        let x = start_x + col * self.scale + dx;
                        let y = start_y + row * self.scale + dy;
                        
                        if x < self.info.width && y < self.info.height {
                            let color = if pixel_on == 1 {
                                self.fg_color
                            } else {
                                self.bg_color
                            };
                            self.write_pixel(x, y, color);
                        }
                    }
                }
            }
        }
    }

    /// 寫入像素
    fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let pixel_offset = (y * self.info.width + x) * bytes_per_pixel;
        
        if pixel_offset + bytes_per_pixel <= self.buffer.len() {
            self.write_pixel_at_offset(pixel_offset, color);
        }
    }

    /// 在指定偏移處寫入像素
    fn write_pixel_at_offset(&mut self, offset: usize, color: Color) {
        let bytes_per_pixel = self.info.bytes_per_pixel;
        
        if offset + bytes_per_pixel <= self.buffer.len() {
            // BGR(A) 格式
            self.buffer[offset] = color.b;     // Blue
            if bytes_per_pixel > 1 {
                self.buffer[offset + 1] = color.g; // Green
            }
            if bytes_per_pixel > 2 {
                self.buffer[offset + 2] = color.r; // Red
            }
            if bytes_per_pixel > 3 {
                self.buffer[offset + 3] = 255;     // Alpha
            }
        }
    }
}

/// 實現 fmt::Write trait，支持格式化輸出
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}