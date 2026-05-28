# rSIDlet

[English](README_en.md) | 中文

`figlet` 在 Rust 上的简单实现，支持中文字符渲染（HZK 点阵字库 / TTF/OTF 矢量字体）。提供命令行工具 `sidlet` 和 Rust 库函数。

## 安装

### 方式一：安装脚本（推荐）

直接从 GitHub 下载最新预编译二进制并完成安装，自动配置字体文件和 `PATH`。

#### Windows（PowerShell）

```powershell
iwr -Uri "https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.ps1" -OutFile "$env:TEMP\install.ps1"; & "$env:TEMP\install.ps1"
```

安装到 `%LOCALAPPDATA%\Programs\rsidlet\`，并自动加入用户 `PATH`。若命令未立即生效，请重新打开终端。

#### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.sh | bash
```

安装到 `~/.local/bin/`。若目录不在 `PATH` 中，脚本会提示添加方式。也可指定自定义路径：

```bash
RSIDLET_INSTALL_DIR=/your/path bash install.sh
```

#### 指定版本

将版本号作为参数传入：

```bash
# Linux / macOS
curl -fsSL https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.sh | bash -s v1.1.5

# Windows PowerShell
$ver="v1.1.5"; iwr -Uri "https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.ps1" -OutFile "$env:TEMP\install.ps1"; & "$env:TEMP\install.ps1" -Version $ver
```

### 方式二：Cargo 安装

适合已安装 Rust 的用户。但 Cargo 不会自动安装字体文件，安装后需运行 `sidlet --test` 修复。

```bash
cargo install rsidlet
sidlet --test
```

### 方式三：源码编译安装

适合预编译包不兼容的平台，或需要使用最新开发版本。

**前提**：[Rust 工具链](https://rustup.rs/)（stable）、[Git](https://git-scm.com/)。

```bash
# 克隆仓库
git clone https://github.com/SidneyLYZhang/rSIDlet.git
cd rSIDlet

# 编译 release 版本
cargo build --release
```

编译完成后，按以下步骤安装：

**1. 复制可执行文件到系统 PATH 目录：**

| 系统 | 命令 |
|------|------|
| Linux / macOS | `cp target/release/sidlet ~/.local/bin/` |
| Windows (PowerShell) | `copy target\release\sidlet.exe %LOCALAPPDATA%\Programs\rsidlet\` |

**2. 复制 fonts/ 目录到可执行文件同级目录：**程序运行时会在可执行文件同级目录的 `fonts/` 下查找字体文件，因此必须将字体目录复制过去。

| 系统 | 命令 |
|------|------|
| Linux / macOS | `cp -r fonts ~/.local/bin/` |
| Windows (PowerShell) | `xcopy fonts %LOCALAPPDATA%\Programs\rsidlet\fonts\ /E /I` |

> **提示**：`make package` 可将编译产物和 fonts 打包为可分发归档包。

### 验证安装

```bash
sidlet --version            # 输出版本号
sidlet --test               # 检查字体完整性，正常输出彩色 "It's ready"
sidlet "Hello World"        # 英文渲染测试
sidlet "你好世界"            # 中文渲染测试
```

## 命令行使用

### 基本渲染

```bash
sidlet "Hello World"                  # 默认字体（standard）
sidlet -f big.flf "Hello"             # 指定 FIGlet/TOIlet 字体
sidlet "你好世界"                      # 中文自动切换渲染模式
sidlet -f simhei.ttf -s 16 "你好世界"  # 使用系统矢量字体
sidlet -w 40 "长文本自动换行"          # 限制输出宽度
```

### 颜色遮蔽

```bash
sidlet -m rainbow "Rainbow"   # 彩虹渐变
sidlet -m fire "Fire"         # 火焰效果
sidlet -m metal "Metal"       # 金属质感
sidlet -m red "Red"           # 纯色
```

支持 `none`、`rainbow`、`rainbowline`、`metal`、`fire`、`water`、`random`，以及 8 种纯色。详见 [SIDlet 手册](docs/SIDlet-manpage.md)。

### 自定义前景/背景

```bash
sidlet --fore "#" --back "." "Hello"
sidlet --fore "■" "你好"
```

### 字体管理

```bash
sidlet --install big.flf             # 在线安装字体
sidlet --install myfont.flf -d /path # 从本地文件安装
sidlet --info big.flf                # 查看字体信息
sidlet --list installed              # 列出已安装字体
sidlet --list letters                # 列出可在线下载的字体
sidlet --list font                   # 列出系统矢量字体
sidlet --list colormap               # 列出可用颜色遮蔽
```

### 高级选项

```bash
sidlet -C mycontrol.flc -f standard.flf "Hello"   # FIGlet Control 文件
sidlet -d /custom/fonts -f myfont.flf "Hello"     # 额外字体搜索目录
sidlet -s 14 "你好世界"                             # 指定点阵字体尺寸（12/14/16）
```

完整选项参考 [SIDlet 手册](docs/SIDlet-manpage.md)。

## 库函数使用

```bash
cargo add rsidlet
```

### 基础渲染

```rust
use rsidlet::figfont;

let data = figfont::load_font_data("standard.flf")?;
let lines = data.render("Hello World");
for line in &lines { println!("{}", line); }
```

### 中文渲染

```rust
use rsidlet::chilet;

// HZK 点阵字库
if let Some(path) = chilet::find_hzk("HZK16") {
    let lines = chilet::render_hzk("你好", &path)?;
    for line in &lines { println!("{}", line); }
}

// 系统矢量字体
let lines = chilet::render_vector_font("你好", "SimHei", 32.0)?;
for line in &lines { println!("{}", line); }

// 指定字体文件
let lines = chilet::render_with_font_file("你好", &font_path, 24.0)?;
for line in &lines { println!("{}", line); }
```

### 颜色滤镜与拼接

```rust
use rsidlet::utils;

let filter = utils::parse_filter("rainbow").unwrap();
utils::print_colored(&lines, filter);

let combined = utils::hcat(&left_lines, &right_lines, 4);
```

## 字体

### 内置字体

| 字体文件 | 类型 | 说明 |
|---------|------|------|
| `standard.flf` | FIGlet | 标准字体（默认） |
| `big.flf` | FIGlet | 大号字体 |
| `phm-shinonome.flf` | FIGlet | PHM shinonome 字体 |
| `future.tlf` | TOIlet | future 字体 |
| `HZK12` | 点阵 | 12×12 中文字库 |
| `HZK14` | 点阵 | 14×14 中文字库 |
| `HZK16` | 点阵 | 16×16 中文字库 |

### 字体搜索路径

程序运行时按以下优先级搜索字体（详见 [paths.rs](src/paths.rs)）：

1. 可执行文件同级的 `fonts/`
2. 可执行文件上级目录的 `fonts/`
3. 扩展字体目录（用户可写，`--install` 安装位置）
4. 系统级 figlet 目录（`/usr/share/figlet` 等）
5. `-d` 参数指定的额外目录

### 在线安装更多字体

```bash
sidlet --install [font_name]
```

字体源自 [xero/figlet-fonts](https://github.com/xero/figlet-fonts) 和 [PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts)。

### 中文字体

除内置 HZK 点阵字库外，还可使用系统矢量字体（TTF/OTF）。Windows 下 `SimHei`（黑体）在字号 12 时效果较好。

## 文档

- [SIDlet 手册（完整 CLI 参考）](docs/SIDlet-manpage.md)
- [更新日志](CHANGELOG.md)

## 致谢

- [FIGlet](http://www.figlet.org/) — ASCII 艺术字体的开创项目
- [TOIlet](https://github.com/cacalabs/toilet) — FIGlet 增强版，支持颜色滤镜
- [BitmapFont](https://github.com/aguegu/BitmapFont) — 点阵中文字库
- [xero/figlet-fonts](https://github.com/xero/figlet-fonts) — FIGlet 字体合集
- [PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts) — 更多 FIGlet 字体

## 许可证

[MIT License](LICENSE)
