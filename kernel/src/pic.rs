// kernel/src/pic.rs

use x86_64::instructions::port::Port;
use spin::Mutex;

/// 8259 PIC 的端口地址
const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

/// PIC 初始化命令
const ICW1_INIT: u8 = 0x11;
const ICW4_8086: u8 = 0x01;

/// 中断向量偏移
pub const PIC1_OFFSET: u8 = 32;  // 主 PIC 中断号从 32 开始
pub const PIC2_OFFSET: u8 = 40;  // 从 PIC 中断号从 40 开始

/// ✨ 中断号定义
pub const TIMER_INTERRUPT_ID: u8 = PIC1_OFFSET + 0;     // IRQ0 = 32 (定时器)
pub const KEYBOARD_INTERRUPT_ID: u8 = PIC1_OFFSET + 1;  // IRQ1 = 33 (键盘)

pub struct Pics {
    pic1_command: Port<u8>,
    pic1_data: Port<u8>,
    pic2_command: Port<u8>,
    pic2_data: Port<u8>,
}

impl Pics {
    pub const fn new() -> Pics {
        Pics {
            pic1_command: Port::new(PIC1_COMMAND),
            pic1_data: Port::new(PIC1_DATA),
            pic2_command: Port::new(PIC2_COMMAND),
            pic2_data: Port::new(PIC2_DATA),
        }
    }

    /// 初始化 PIC
    pub unsafe fn initialize(&mut self) {
        // 禁用所有中断
        self.pic1_data.write(0xFF);
        self.pic2_data.write(0xFF);

        // 开始初始化序列
        self.pic1_command.write(ICW1_INIT);
        io_wait();
        self.pic2_command.write(ICW1_INIT);
        io_wait();

        // 设置中断向量偏移
        self.pic1_data.write(PIC1_OFFSET);
        io_wait();
        self.pic2_data.write(PIC2_OFFSET);
        io_wait();

        // 配置 PIC 链接
        self.pic1_data.write(4);  // 主 PIC 的 IRQ2 连接从 PIC
        io_wait();
        self.pic2_data.write(2);  // 从 PIC 连接到主 PIC 的 IRQ2
        io_wait();

        // 设置 8086 模式
        self.pic1_data.write(ICW4_8086);
        io_wait();
        self.pic2_data.write(ICW4_8086);
        io_wait();

        // 重新禁用所有中断，稍后手动启用需要的
        self.pic1_data.write(0xFF);
        self.pic2_data.write(0xFF);
    }

    /// 启用特定中断
    pub unsafe fn enable_interrupt(&mut self, irq: u8) {
        if irq < 8 {
            let mask = self.pic1_data.read();
            self.pic1_data.write(mask & !(1 << irq));
        } else {
            let mask = self.pic2_data.read();
            self.pic2_data.write(mask & !(1 << (irq - 8)));
        }
    }

    /// 发送 EOI (End of Interrupt) 信号
    pub unsafe fn end_of_interrupt(&mut self, interrupt_id: u8) {
        if interrupt_id >= PIC2_OFFSET {
            // 如果是从 PIC 的中断，两个 PIC 都要发送 EOI
            self.pic2_command.write(0x20);
        }
        // 总是向主 PIC 发送 EOI
        self.pic1_command.write(0x20);
    }
}

/// I/O 延时函数
unsafe fn io_wait() {
    Port::new(0x80).write(0u8);
}

/// 全局 PIC 实例
static PICS: Mutex<Pics> = Mutex::new(Pics::new());

/// 初始化 PIC
pub fn init() {
    unsafe {
        PICS.lock().initialize();
    }
}

/// ✨ 启用定时器中断
pub fn enable_timer() {
    unsafe {
        PICS.lock().enable_interrupt(0); // IRQ0 = 定时器
    }
}

/// 启用键盘中断
pub fn enable_keyboard() {
    unsafe {
        PICS.lock().enable_interrupt(1); // IRQ1 = 键盘
    }
}

/// 发送中断结束信号
pub fn end_of_interrupt(interrupt_id: u8) {
    unsafe {
        PICS.lock().end_of_interrupt(interrupt_id);
    }
}