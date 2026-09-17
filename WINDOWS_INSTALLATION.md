# Windows 安装指南

## 如何选择正确的安装包

本应用提供两种 Windows 架构的安装包：

### 1. x64 版本（适用于大多数用户）
- **文件名**: `YDL.GUI_x.x.x_x64-setup.exe` 或 `YDL.GUI_x.x.x_x64_en-US.msi`
- **适用于**: Intel 或 AMD 处理器的 Windows 电脑
- **如何确认**: 大多数台式机和笔记本电脑都使用 x64 架构

### 2. ARM64 版本（适用于 ARM 处理器设备）
- **文件名**: `YDL.GUI_x.x.x_arm64-setup.exe` 或 `YDL.GUI_x.x.x_arm64_en-US.msi`
- **适用于**: 使用 ARM 处理器的 Windows 设备（如部分 Surface 设备）
- **如何确认**: 查看"系统信息"中的"处理器"字段，如果包含"ARM"字样，则需要此版本

## 如何查看你的系统架构

1. 按 `Win + R` 打开运行对话框
2. 输入 `msinfo32` 并回车
3. 在"系统摘要"中查看"系统类型"：
   - **x64-based PC**: 使用 x64 版本
   - **ARM-based PC** 或 **ARM64-based PC**: 使用 ARM64 版本

## 常见问题

### Q: 我收到"不兼容程序"错误
**A**: 这可能是因为：
1. 下载了错误架构的版本（见上方如何选择正确版本）
2. 使用了旧版本的安装包（请确保使用 v0.8.1 或更高版本，包含 Windows 11 兼容性修复）

### Q: .exe 和 .msi 有什么区别？
**A**: 
- **`.exe` (NSIS 安装程序)**: 更简单的安装向导，推荐大多数用户使用
- **`.msi` (MSI 安装程序)**: 支持企业部署和 GPO（组策略）管理

两者功能相同，选择你喜欢的即可。

### Q: 需要管理员权限吗？
**A**: 不需要。应用以标准用户权限运行（`asInvoker`），但安装程序可能需要管理员权限来安装到 Program Files 目录。

## 系统要求

- **操作系统**: Windows 7 SP1 或更高版本（推荐 Windows 10/11）
- **WebView2**: 自动检测并安装（如果缺失）
- **架构**: x64 或 ARM64

## 重新构建

如果你想从源码构建：

```bash
# 安装依赖
pnpm install

# 构建 Windows 版本
pnpm tauri build
```

生成的安装包位于 `src-tauri/target/release/bundle/` 目录。
