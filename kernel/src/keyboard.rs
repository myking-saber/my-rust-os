// kernel/src/keyboard.rs

/// 鍵盤狀態 - 跟蹤修飾鍵狀態
pub struct KeyboardState {
    pub shift_pressed: bool,
    pub ctrl_pressed: bool,  // 為將來擴展預留
    pub alt_pressed: bool,   // 為將來擴展預留
    pub caps_lock: bool,     // Caps Lock 狀態
}

impl KeyboardState {
    pub const fn new() -> KeyboardState {
        KeyboardState {
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
            caps_lock: false,
        }
    }
}

/// 處理修飾鍵的按下和釋放
pub fn handle_modifier_key(state: &mut KeyboardState, scancode: u8) -> bool {
    match scancode {
        // Shift 鍵按下
        0x2A | 0x36 => { // 左 Shift (0x2A) 或右 Shift (0x36)
            state.shift_pressed = true;
            true // 表示這是修飾鍵
        },
        // Shift 鍵釋放
        0xAA | 0xB6 => { // 左 Shift 釋放 (0x2A + 0x80) 或右 Shift 釋放 (0x36 + 0x80)
            state.shift_pressed = false;
            true
        },
        // Caps Lock 按下（切換狀態）
        0x3A => { // Caps Lock 鍵
            state.caps_lock = !state.caps_lock; // 切換 Caps Lock 狀態
            true
        },
        _ => false // 不是修飾鍵
    }
}

/// 將掃描碼轉換為字符（考慮 Shift 和 Caps Lock 狀態）
pub fn scancode_to_char(scancode: u8, shift_pressed: bool, caps_lock: bool) -> Option<char> {
    match scancode {
        // 數字行 - 不受 Caps Lock 影響，只受 Shift 影響
        0x02 => Some(if shift_pressed { '!' } else { '1' }),
        0x03 => Some(if shift_pressed { '@' } else { '2' }),
        0x04 => Some(if shift_pressed { '#' } else { '3' }),
        0x05 => Some(if shift_pressed { '$' } else { '4' }),
        0x06 => Some(if shift_pressed { get_percent_char() } else { '5' }),
        0x07 => Some(if shift_pressed { '^' } else { '6' }),
        0x08 => Some(if shift_pressed { '&' } else { '7' }),
        0x09 => Some(if shift_pressed { '*' } else { '8' }),
        0x0A => Some(if shift_pressed { '(' } else { '9' }),
        0x0B => Some(if shift_pressed { ')' } else { '0' }),
        
        // QWERTY 行 - 受 Caps Lock 和 Shift 影響
        0x10 => Some(letter_case('q', 'Q', shift_pressed, caps_lock)),
        0x11 => Some(letter_case('w', 'W', shift_pressed, caps_lock)),
        0x12 => Some(letter_case('e', 'E', shift_pressed, caps_lock)),
        0x13 => Some(letter_case('r', 'R', shift_pressed, caps_lock)),
        0x14 => Some(letter_case('t', 'T', shift_pressed, caps_lock)),
        0x15 => Some(letter_case('y', 'Y', shift_pressed, caps_lock)),
        0x16 => Some(letter_case('u', 'U', shift_pressed, caps_lock)),
        0x17 => Some(letter_case('i', 'I', shift_pressed, caps_lock)),
        0x18 => Some(letter_case('o', 'O', shift_pressed, caps_lock)),
        0x19 => Some(letter_case('p', 'P', shift_pressed, caps_lock)),
        
        // ASDF 行 - 受 Caps Lock 和 Shift 影響
        0x1E => Some(letter_case('a', 'A', shift_pressed, caps_lock)),
        0x1F => Some(letter_case('s', 'S', shift_pressed, caps_lock)),
        0x20 => Some(letter_case('d', 'D', shift_pressed, caps_lock)),
        0x21 => Some(letter_case('f', 'F', shift_pressed, caps_lock)),
        0x22 => Some(letter_case('g', 'G', shift_pressed, caps_lock)),
        0x23 => Some(letter_case('h', 'H', shift_pressed, caps_lock)),
        0x24 => Some(letter_case('j', 'J', shift_pressed, caps_lock)),
        0x25 => Some(letter_case('k', 'K', shift_pressed, caps_lock)),
        0x26 => Some(letter_case('l', 'L', shift_pressed, caps_lock)),
        
        // ZXCV 行 - 受 Caps Lock 和 Shift 影響
        0x2C => Some(letter_case('z', 'Z', shift_pressed, caps_lock)),
        0x2D => Some(letter_case('x', 'X', shift_pressed, caps_lock)),
        0x2E => Some(letter_case('c', 'C', shift_pressed, caps_lock)),
        0x2F => Some(letter_case('v', 'V', shift_pressed, caps_lock)),
        0x30 => Some(letter_case('b', 'B', shift_pressed, caps_lock)),
        0x31 => Some(letter_case('n', 'N', shift_pressed, caps_lock)),
        0x32 => Some(letter_case('m', 'M', shift_pressed, caps_lock)),
        
        // 特殊鍵
        0x39 => Some(' '),  // 空格鍵
        0x1C => Some('\n'), // 回車鍵
        0x0E => Some('\x08'), // 退格鍵
        0x0F => Some('\t'), // Tab 鍵
        
        // 標點符號 - 不受 Caps Lock 影響，只受 Shift 影響
        0x0C => Some(if shift_pressed { '_' } else { '-' }),
        0x0D => Some(if shift_pressed { '+' } else { '=' }),
        0x1A => Some(if shift_pressed { '{' } else { '[' }),
        0x1B => Some(if shift_pressed { '}' } else { ']' }),
        0x27 => Some(if shift_pressed { ':' } else { ';' }),
        0x28 => Some(if shift_pressed { get_quote_char() } else { get_apostrophe_char() }),
        0x29 => Some(if shift_pressed { '~' } else { '`' }),
        0x2B => Some(if shift_pressed { get_pipe_char() } else { get_backslash_char() }),
        0x33 => Some(if shift_pressed { '<' } else { ',' }),
        0x34 => Some(if shift_pressed { '>' } else { '.' }),
        0x35 => Some(if shift_pressed { '?' } else { '/' }),
        
        _ => None, // 未知或不支持的鍵
    }
}

/// 處理字母大小寫邏輯
/// Caps Lock XOR Shift = 大寫
fn letter_case(lowercase: char, uppercase: char, shift_pressed: bool, caps_lock: bool) -> char {
    // XOR 邏輯：當且僅當 Caps Lock 和 Shift 中有一個（但不是兩個）為 true 時，返回大寫
    if caps_lock ^ shift_pressed {
        uppercase
    } else {
        lowercase
    }
}

/// 返回百分號字符
fn get_percent_char() -> char {
    '%'
}

/// 返回雙引號字符
fn get_quote_char() -> char {
    '"'
}

/// 返回單引號字符
fn get_apostrophe_char() -> char {
    '\''
}

/// 返回豎線字符
fn get_pipe_char() -> char {
    '|'
}

/// 返回反斜杠字符
fn get_backslash_char() -> char {
    '\\'
}