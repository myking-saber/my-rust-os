// kernel/src/pit.rs
// PIT 8253 可编程间隔定时器驱动

use x86_64::instructions::port::Port;
use spin::Mutex;

/// PIT 端口地址
const PIT_CHANNEL_0: u16 = 0x40;  // 通道0数据端口
const PIT_CHANNEL_1: u16 = 0x41;  // 通道1数据端口 (未使用)
const PIT_CHANNEL_2: u16 = 0x42;  // 通道2数据端口 (未使用)
const PIT_COMMAND: u16 = 0x43;    // 命令寄存器

/// PIT 基础频率 (1.193182 MHz)
const PIT_BASE_FREQUENCY: u32 = 1193182;

/// 目标频率 (100 Hz = 每秒100次中断)
const TARGET_FREQUENCY: u32 = 100;

/// 计算的分频值
const DIVISOR: u16 = (PIT_BASE_FREQUENCY / TARGET_FREQUENCY) as u16;

/// PIT 命令字节
/// 格式: [SC1 SC0 RW1 RW0 M2 M1 M0 BCD]
/// SC1 SC0 = 00 (选择通道0)
/// RW1 RW0 = 11 (读写低字节然后高字节)  
/// M2 M1 M0 = 010 (模式2: 速率发生器)
/// BCD = 0 (二进制模式)
const PIT_COMMAND_BYTE: u8 = 0x34;

/// PIT 控制器结构
pub struct Pit {
    channel_0: Port<u8>,
    command: Port<u8>,
    initialized: bool,
}

impl Pit {
    /// 创建新的 PIT 实例
    pub const fn new() -> Pit {
        Pit {
            channel_0: Port::new(PIT_CHANNEL_0),
            command: Port::new(PIT_COMMAND),
            initialized: false,
        }
    }

    /// 初始化 PIT
    /// 配置通道0为100Hz的定时器
    pub unsafe fn initialize(&mut self) {
        // 发送命令字节
        self.command.write(PIT_COMMAND_BYTE);
        
        // 等待一小段时间确保命令被处理
        io_wait();
        
        // 写入分频值 (先低字节，后高字节)
        let divisor_low = (DIVISOR & 0xFF) as u8;
        let divisor_high = ((DIVISOR >> 8) & 0xFF) as u8;
        
        self.channel_0.write(divisor_low);
        io_wait();
        self.channel_0.write(divisor_high);
        io_wait();
        
        self.initialized = true;
    }

    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// 获取配置的频率
    pub fn get_frequency(&self) -> u32 {
        TARGET_FREQUENCY
    }

    /// 获取每次中断的时间间隔 (毫秒)
    pub fn get_interval_ms(&self) -> u32 {
        1000 / TARGET_FREQUENCY  // 100Hz = 10ms
    }
}

/// I/O 等待函数
unsafe fn io_wait() {
    Port::new(0x80).write(0u8);
}

/// 全局 PIT 实例
static PIT: Mutex<Pit> = Mutex::new(Pit::new());

/// 初始化 PIT 系统
pub fn init() {
    unsafe {
        PIT.lock().initialize();
    }
}

/// 获取 PIT 配置信息
pub fn get_info() -> (u32, u32) {
    let pit = PIT.lock();
    (pit.get_frequency(), pit.get_interval_ms())
}

/// 检查 PIT 是否已初始化
pub fn is_initialized() -> bool {
    PIT.lock().is_initialized()
}