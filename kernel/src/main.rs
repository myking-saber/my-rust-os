#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};

mod font;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();
        
        // 清屏為黑色
        clear_screen(buffer, info);
        
        // 測試字符串顯示
        draw_string(buffer, info, "Hello World!", 50, 50);
        draw_string(buffer, info, "Rust OS is working!", 50, 70);
        draw_string(buffer, info, "ABCDEFGHIJKLMNOPQRSTUVWXYZ", 50, 90);
        draw_string(buffer, info, "0123456789", 50, 110);
    }
    
    loop {}
}

fn clear_screen(buffer: &mut [u8], info: bootloader_api::info::FrameBufferInfo) {
    let bytes_per_pixel = info.bytes_per_pixel;
    let total_pixels = info.width * info.height;
    let expected_size = total_pixels * bytes_per_pixel;
    
    if buffer.len() >= expected_size {
        for i in (0..expected_size).step_by(bytes_per_pixel) {
            if i + bytes_per_pixel <= buffer.len() {
                buffer[i] = 0;       // Blue (黑色)
                if bytes_per_pixel > 1 {
                    buffer[i + 1] = 0;   // Green
                }
                if bytes_per_pixel > 2 {
                    buffer[i + 2] = 0;   // Red
                }
                if bytes_per_pixel > 3 {
                    buffer[i + 3] = 255; // Alpha
                }
            }
        }
    }
}

fn draw_char_simple(
    buffer: &mut [u8],
    info: bootloader_api::info::FrameBufferInfo,
    ch: char,
    start_x: usize,
    start_y: usize,
) {
    let char_bitmap = font::Font8x8::get_char(ch);
    let bytes_per_pixel = info.bytes_per_pixel;
    let scale = 2; // 2x 放大，讓字符變為 16x16
    
    // 繪製 8x8 字符，每個像素放大為 2x2
    for (row, &bitmap_row) in char_bitmap.iter().enumerate() {
        for col in 0..8 {
            // 修正位提取順序 - 嘗試另一個方向
            let pixel_on = (bitmap_row >> col) & 1; // 改為從低位開始
            
            // 繪製放大的像素塊
            for dy in 0..scale {
                for dx in 0..scale {
                    let x = start_x + col * scale + dx;
                    let y = start_y + row * scale + dy;
                    
                    // 確保不超出屏幕邊界
                    if x < info.width && y < info.height {
                        let pixel_offset = (y * info.width + x) * bytes_per_pixel;
                        
                        if pixel_offset + bytes_per_pixel <= buffer.len() {
                            if pixel_on == 1 {
                                // 繪製白色像素（前景）
                                buffer[pixel_offset] = 255;     // Blue
                                if bytes_per_pixel > 1 {
                                    buffer[pixel_offset + 1] = 255; // Green
                                }
                                if bytes_per_pixel > 2 {
                                    buffer[pixel_offset + 2] = 255; // Red  
                                }
                                if bytes_per_pixel > 3 {
                                    buffer[pixel_offset + 3] = 255; // Alpha
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn draw_string(
    buffer: &mut [u8],
    info: bootloader_api::info::FrameBufferInfo,
    text: &str,
    mut x: usize,
    y: usize,
) {
    for ch in text.chars() {
        let char_width = 16; // 因為我們放大了 2 倍，所以寬度是 16
        if x + char_width > info.width {
            break;
        }
        draw_char_simple(buffer, info, ch, x, y);
        x += char_width; // 移動到下一個字符位置
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}