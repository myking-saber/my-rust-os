// kernel/src/time.rs
// 系统时间管理

use spin::Mutex;

/// 时间管理器
pub struct TimeManager {
    /// 系统启动以来的毫秒数
    system_ticks: u64,
    /// 每个tick的毫秒数 (由PIT决定)
    ms_per_tick: u32,
    /// 是否已初始化
    initialized: bool,
}

impl TimeManager {
    /// 创建新的时间管理器
    pub const fn new() -> TimeManager {
        TimeManager {
            system_ticks: 0,
            ms_per_tick: 10, // 默认10ms (100Hz)
            initialized: false,
        }
    }

    /// 初始化时间管理器
    pub fn initialize(&mut self, ms_per_tick: u32) {
        self.ms_per_tick = ms_per_tick;
        self.system_ticks = 0;
        self.initialized = true;
    }

    /// 系统tick中断时调用 (暂时手动调用用于测试)
    pub fn tick(&mut self) {
        if self.initialized {
            self.system_ticks += 1;
        }
    }

    /// 获取系统运行的总毫秒数
    pub fn get_uptime_ms(&self) -> u64 {
        if self.initialized {
            self.system_ticks * (self.ms_per_tick as u64)
        } else {
            0
        }
    }

    /// 获取系统运行的秒数
    pub fn get_uptime_seconds(&self) -> u64 {
        self.get_uptime_ms() / 1000
    }

    /// 获取格式化的运行时间 (天:小时:分钟:秒)
    pub fn get_uptime_formatted(&self) -> UptimeInfo {
        let total_seconds = self.get_uptime_seconds();
        
        let days = total_seconds / 86400;
        let hours = (total_seconds % 86400) / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        let milliseconds = (self.get_uptime_ms() % 1000) as u16;

        UptimeInfo {
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
            total_ms: self.get_uptime_ms(),
            total_seconds: self.get_uptime_seconds(), // ✨ 新增字段
        }
    }

    /// 获取tick计数
    pub fn get_tick_count(&self) -> u64 {
        self.system_ticks
    }

    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// 格式化的运行时间信息
#[derive(Debug, Clone, Copy)]
pub struct UptimeInfo {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub milliseconds: u16,
    pub total_ms: u64,
    pub total_seconds: u64, // ✨ 新增字段
}

impl UptimeInfo {
    /// 格式化为字符串显示
    pub fn format_detailed(&self) -> FormattedUptime {
        FormattedUptime {
            uptime_info: *self,
        }
    }

    /// 直接获取总秒数
    pub fn get_uptime_seconds(&self) -> u64 {
        self.total_seconds
    }
}

/// 用于格式化显示的包装器
pub struct FormattedUptime {
    uptime_info: UptimeInfo,
}

impl FormattedUptime {
    /// 获取简短格式 (HH:MM:SS)
    pub fn short_format(&self) -> (u64, u64, u64) {
        (
            self.uptime_info.hours + self.uptime_info.days * 24,
            self.uptime_info.minutes,
            self.uptime_info.seconds,
        )
    }

    /// 获取详细格式的各个组件
    pub fn detailed_format(&self) -> (u64, u64, u64, u64, u16) {
        (
            self.uptime_info.days,
            self.uptime_info.hours,
            self.uptime_info.minutes,
            self.uptime_info.seconds,
            self.uptime_info.milliseconds,
        )
    }

    /// 获取总毫秒数
    pub fn total_milliseconds(&self) -> u64 {
        self.uptime_info.total_ms
    }
}

/// 全局时间管理器
static TIME_MANAGER: Mutex<TimeManager> = Mutex::new(TimeManager::new());

/// 初始化时间系统
pub fn init(ms_per_tick: u32) {
    TIME_MANAGER.lock().initialize(ms_per_tick);
}

/// 系统tick (目前手动调用用于测试)
pub fn tick() {
    TIME_MANAGER.lock().tick();
}

/// 获取系统运行时间
pub fn get_uptime() -> UptimeInfo {
    TIME_MANAGER.lock().get_uptime_formatted()
}

/// 获取系统运行的毫秒数
pub fn get_uptime_ms() -> u64 {
    TIME_MANAGER.lock().get_uptime_ms()
}

/// 获取tick计数
pub fn get_tick_count() -> u64 {
    TIME_MANAGER.lock().get_tick_count()
}

/// 检查时间系统是否已初始化
pub fn is_initialized() -> bool {
    TIME_MANAGER.lock().is_initialized()
}

/// 模拟时间流逝 (用于测试)
/// 这个函数会模拟指定数量的tick
pub fn simulate_time_passage(ticks: u64) {
    let mut manager = TIME_MANAGER.lock();
    for _ in 0..ticks {
        manager.tick();
    }
}