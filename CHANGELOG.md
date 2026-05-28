# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.3] - 2026-05-28

### Added

- 画布滤镜系统（Canvas Filter System）：新增 `Canvas` trait（抽象画布像素读写与几何变换操作）、`CanvasFilter` 枚举（Crop / Rainbow / Metal / Flip / Flop / Rotate180 / RotateLeft / RotateRight / Border 共 9 种滤镜）、`FilterContext` 滤镜管线上下文（支持 `:` 分隔的多滤镜链式组合与动画行计数器）
- `canvas_color` 子模块：映射 libcaca 颜色常量到 ANSI 颜色，提供 `LIGHTBLUE`、`LIGHTGRAY`、`LIGHTMAGENTA` 等 9 种颜色常量
- 滤镜解析：`CanvasFilter::parse()` 支持从字符串名称解析滤镜，`"rotate"` 可作为 `"180"` 的别名以保持向后兼容
- 滤镜实现：`apply_crop()` 裁切空白区域、`apply_rainbow()` 彩虹色渐变、`apply_metal()` 金属色效果、`apply_border()` Unicode 方框字符边框

### Changed

- 内置字体目录查找逻辑增强：新增二进制文件同级 `fonts/` 目录（`exe/fonts/`）作为最高优先级查找路径，共支持 4 级路径回退
- GoReleaser 配置：Windows 构建目标从 `x86_64-pc-windows-msvc` 改为 `x86_64-pc-windows-gnu`，添加 `RING_PREGENERATE_ASM=1` 环境变量
- CI 工作流（`ci.yml`）：push 事件触发限制为 `main` 分支且仅当 `src/**` 路径变更时触发
- 标准输入读取逻辑增强：使用 `IsTerminal` 检测管道/重定向输入，非 TTY 时自动从 stdin 读取全部内容

### Fixed

- Makefile `package` 目标修复 tar 归档路径，使用 `-C` 切换目录确保内部路径结构正确

## [1.1.2] - 2026-05-28

### Added

- 支持从标准输入（stdin）管道读取文本消息，允许通过管道输入文本（如 `echo "Hello" | sidlet`）

### Changed

- CI 工作流（`ci.yml`）优化：push 事件触发限制为 `main` 分支且仅当 `src/**` 路径变更时触发
- GoReleaser 配置（`.goreleaser.yaml`）添加显式 `binary: sidlet` 字段

### Removed

- Release 工作流（`release.yml`）移除独立的 Arch Linux 原生构建 job，统一由 GoReleaser 管理跨平台构建

## [1.1.0] - 2026-05-27

### Added

- GoReleaser 配置（`.goreleaser.yaml`），使用 `cargo-zigbuild` 实现跨平台交叉编译（Linux x86_64/musl、macOS x86_64/aarch64、Windows x64）
- 跨平台 Makefile，提供 `build` / `test` / `lint` / `run` / `doc` / `package` / `install` / `uninstall` / `clean` 等统一构建目标
- 一键安装脚本：`install.sh`（Linux/macOS）和 `install.ps1`（Windows），自动检测平台、下载对应版本的二进制文件并安装
- CI Release 工作流新增 Arch Linux 原生构建 job（在 Arch 容器内编译），确保 ABI 兼容性
- 字体下载新增 HTTP 代理支持：`ureq` 启用 `proxy-from-env` feature，自动从 `HTTP_PROXY` / `HTTPS_PROXY` / `NO_PROXY` 环境变量读取代理配置
- 下载模块使用全局懒加载 `ureq::Agent` 统一管理 HTTP 连接复用与代理设置

### Changed

- CI Release 工作流重构：改用 GoReleaser 统一管理跨平台构建、打包和 GitHub Release 发布
- `--test` 命令增强：在检查字体目录存在性的基础上，新增基础字体文件（big.flf、future.tlf、standard.flf、phm-shinonome.flf、HZK12/14/16）完整性校验
- 文档版本号更新为 `v1.1.0+`

## [1.0.6] - 2026-05-22

### Changed

- Linux 扩展字体目录（目录B）从系统路径（`/usr/share/figlet`、`/usr/local/share/figlet`）改为用户可写路径（`$XDG_DATA_HOME/figlet` 或 `~/.local/share/figlet`）
- macOS 扩展字体目录（目录B）从系统路径改为 `~/Library/Application Support/figlet`
- 字体搜索路径新增系统级 figlet 目录（`/usr/share/figlet`、`/usr/local/share/figlet`）作为只读的第三优先级搜索源
- CI 发布流程优化：`cargo publish` 从 ubuntu 构建 job 中拆分为独立的 `publish` job，依赖 `build` job 全部完成后执行

### Fixed

- CI release workflow 增加 `permissions: contents: write`，修复 GitHub Release 上传权限问题

## [1.0.5] - 2026-05-22

### Added

- 字体下载增加重试机制（默认 3 次，间隔 500ms），提升网络不稳定环境下的安装成功率
- 扩展字体目录（目录B）支持自动创建，不存在时无需手动创建
- Windows 平台家目录检测增加 `HOME` 环境变量回退
- 修复模式（`--test`）增强：自动下载所有必要字体到目录B（big.flf、future.tlf、standard.flf、phm-shinonome.flf、HZK12/14/16）
- 新增 772 行冒烟测试，覆盖 chilet、figfont、paths、utils 模块的关键功能

### Changed

- Windows 平台扩展字体目录（`%USERPROFILE%/fonts`）不再检查目录是否存在，改为由自动创建逻辑统一处理
- 测试安装状态（`--test`）的目录显示标签优化为"目录A（内置）"和"目录B（扩展）"
- README 中英文文档增加 `cargo install` 后需运行 `sidlet --test` 修复字体的说明

### Fixed

- 清理 install.rs 中残留的注释代码
- 修复 paths.rs 中 `home_dir()` 与 `extended_font_dir()` 的逻辑一致性问题

## [1.0.0] - 2026-05-21

### Added

- 首个正式版本发布
- **核心功能**：figlet/toilet 风格的 ASCII 艺术字渲染引擎
- **中文支持**：通过 HZK 点阵字库（12/14/16px）和矢量字体（TTF/OTF）两种方式渲染中文
- **颜色滤镜**：支持 rainbow（逐字符/逐行彩虹）、metal（金属）、fire（火焰）、water（水色）、random（随机）及 8 种纯色
- **字体管理**：
  - `--install`：在线安装 figlet/toilet 字体
  - `--info` / `-i`：查看字体头部信息
  - `--list font|colormap|installed|letters`：列出资源
  - `--test`：检查并修复字体目录安装状态
- **在线字体下载**：从 xero/figlet-fonts、PhMajerus/FIGfonts、aguegu/BitmapFont 三个 GitHub 仓库下载字体
- **命令行选项**：
  - `-f` / `--font`：指定字体
  - `-w` / `--width`：最大输出宽度，超宽自动换行
  - `-m` / `--maskcolor`：颜色遮蔽
  - `-s` / `--size`：点阵字体大小
  - `--fore` / `--back`：自定义前景/背景字符
  - `-C` / `--control`：figlet control 文件
  - `-d` / `--directory`：额外字体搜索目录
- **figlet control 文件支持**：支持 `.flc` 格式的布局控制文件
- **Rust 库函数**：提供 `figfont`、`chilet`、`utils` 等公共模块，方便集成到其他 Rust 项目
- **跨平台**：支持 Windows（x64）、macOS（universal）、Ubuntu Linux（x64）、Arch Linux（x64）
- **内置字体**：standard.flf、big.flf、phm-shinonome.flf、future.tlf、HZK12、HZK14、HZK16
- **发布到 crates.io**：可通过 `cargo install rsidlet` 安装
