#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();
        
        // 安全地清屏 - 設置為藍色背景
        let bytes_per_pixel = info.bytes_per_pixel;
        let stride = info.stride;
        
        for y in 0..info.height {
            for x in 0..info.width {
                let pixel_offset = (y * stride + x) * bytes_per_pixel;
                
                // 確保不會越界
                if pixel_offset + bytes_per_pixel <= buffer.len() {
                    // 設置為藍色 (BGR 格式)
                    buffer[pixel_offset] = 255;     // Blue
                    if bytes_per_pixel > 1 {
                        buffer[pixel_offset + 1] = 0;   // Green
                    }
                    if bytes_per_pixel > 2 {
                        buffer[pixel_offset + 2] = 0;   // Red
                    }
                    if bytes_per_pixel > 3 {
                        buffer[pixel_offset + 3] = 255; // Alpha
                    }
                }
            }
        }
        
        // 在屏幕中央畫一個簡單的白色矩形
        let rect_width = 200;
        let rect_height = 100;
        let rect_x = (info.width - rect_width) / 2;
        let rect_y = (info.height - rect_height) / 2;
        
        for y in rect_y..(rect_y + rect_height) {
            for x in rect_x..(rect_x + rect_width) {
                let pixel_offset = (y * stride + x) * bytes_per_pixel;
                
                if pixel_offset + bytes_per_pixel <= buffer.len() {
                    // 設置為白色
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
    
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}