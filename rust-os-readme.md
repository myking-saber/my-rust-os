# My Rust OS

一個使用 Rust 編程語言和 bootloader 0.11+ 構建的簡單操作系統項目。

## 項目概述

這是一個最小化的 64 位 x86 操作系統，展示了：
- 使用現代 bootloader 0.11+ 架構
- 幀緩衝區圖形輸出
- 基本的內存安全操作系統內核

## 功能特性

- ✅ **現代 Bootloader**：使用 bootloader 0.11+ 支持 BIOS 和 UEFI
- ✅ **圖形顯示**：通過幀緩衝區進行圖形輸出
- ✅ **內存安全**：100% Rust 編寫的內核代碼
- ✅ **跨平台構建**：支持在 Linux/macOS/Windows 上構建

## 項目結構

```
my-kernel/
├── Cargo.toml          # 工作空間配置
├── build.rs            # 構建腳本，生成磁盤映像
├── .cargo/
│   └── config.toml     # Cargo 配置
├── kernel/             # 內核源代碼
│   ├── Cargo.toml
│   └── src/
│       └── main.rs     # 內核主程序
└── README.md           # 項目說明
```

## 系統要求

### 軟件要求
- **Rust Nightly**：需要最新的 nightly 工具鏈
- **QEMU**：用於虛擬機測試（推薦）
- **llvm-tools-preview**：Rust 組件

### 支持的平台
- **目標平台**：x86_64
- **開發平台**：Linux、macOS、Windows

## 安裝與設置

### 1. 安裝 Rust Nightly
```bash
# 安裝 nightly 工具鏈
rustup install nightly
rustup override set nightly

# 安裝必要組件
rustup component add llvm-tools-preview
rustup target add x86_64-unknown-none
```

### 2. 安裝 QEMU（可選，用於測試）
```bash
# Ubuntu/Debian
sudo apt install qemu-system-x86

# macOS
brew install qemu

# Windows
# 從 https://www.qemu.org/ 下載安裝
```

### 3. 克隆並構建項目
```bash
git clone <your-repo-url>
cd my-kernel
cargo build
```

## 運行操作系統

### 使用 cargo run（推薦）
```bash
cargo run
```

### 手動運行 QEMU
```bash
# 找到生成的磁盤映像
find target -name "bios.img" -exec qemu-system-x86_64 -drive format=raw,file={} \;
```

### 預期輸出
運行成功後，你應該看到：
- QEMU 窗口打開
- 藍色背景的屏幕
- 屏幕中央有一個白色矩形

## 技術細節

### Bootloader 架構
本項目使用 bootloader 0.11+ 架構：
- **分離的 API**：`bootloader_api` 用於內核接口
- **artifact dependencies**：自動構建和鏈接內核
- **運行時加載**：支持 FAT 文件系統上的內核加載
- **雙模式支持**：同時生成 BIOS 和 UEFI 磁盤映像

### 關鍵配置文件

#### .cargo/config.toml
```toml
[unstable]
bindeps = true                    # 啟用 artifact dependencies

[build]
target = "x86_64-unknown-none"    # 裸機目標平台

[target.x86_64-unknown-none]
runner = "qemu-system-x86_64 -drive format=raw,file={}"
```

#### Cargo.toml（根目錄）
```toml
[workspace]
members = ["kernel"]

[build-dependencies]
bootloader = "0.11"
kernel = { path = "kernel", artifact = "bin", target = "x86_64-unknown-none" }
```

### 內核實現
內核主要功能：
- **入口點**：使用 `bootloader_api::entry_point!` 宏
- **幀緩衝區訪問**：通過 `BootInfo` 獲取顯示信息
- **像素操作**：直接操作幀緩衝區進行圖形繪製
- **無標準庫**：`#![no_std]` 環境下運行

## 開發指南

### 添加新功能
1. 修改 `kernel/src/main.rs` 中的 `kernel_main` 函數
2. 運行 `cargo build` 重新構建
3. 使用 `cargo run` 測試

### 調試技巧
- 使用 `cargo build` 檢查編譯錯誤
- 在 QEMU 中按 `Ctrl+Alt+2` 切換到 QEMU 監視器
- 使用 `cargo clean` 清理構建緩存

### 常見問題

#### 編譯錯誤：`bindeps` 功能未啟用
**解決方案**：確保 `.cargo/config.toml` 中有 `bindeps = true`

#### 找不到 `CARGO_BIN_FILE_KERNEL_kernel` 環境變量
**解決方案**：檢查根目錄 `Cargo.toml` 是否包含 artifact dependency 配置

#### QEMU 窗口是黑屏
**可能原因**：
- 內核 panic 或無限循環
- 幀緩衝區訪問錯誤
- 檢查終端輸出中的錯誤信息

## 擴展功能建議

### 短期目標
- [ ] 實現文字渲染系統
- [ ] 添加鍵盤輸入處理
- [ ] 實現簡單的命令行界面

### 長期目標
- [ ] 內存管理和堆分配器
- [ ] 進程和線程支持
- [ ] 文件系統實現
- [ ] 網絡協議棧

## 相關資源

- [Writing an OS in Rust](https://os.phil-opp.com/) - 優秀的 Rust 操作系統開發教程
- [bootloader 官方文檔](https://docs.rs/bootloader/) - bootloader 包文檔
- [OSDev Wiki](https://wiki.osdev.org/) - 操作系統開發資源
- [Rust Embedded Book](https://rust-embedded.github.io/book/) - Rust 嵌入式開發指南

## 許可證

本項目采用 MIT 許可證 - 查看 [LICENSE](LICENSE) 文件了解詳情。

## 貢獻

歡迎提交 Issue 和 Pull Request！

## 致謝

感謝 Rust 操作系統開發社區和 bootloader 項目的貢獻者們。
