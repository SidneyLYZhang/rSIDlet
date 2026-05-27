# rSIDlet

中文 | [English](README_en.md)

`figlet` 在 Rust 上的简单实现，并添加了中文支持。中文支持通过点阵字库（HZK）和矢量字体（TTF/OTF）两种方式实现。

本项目既提供命令行工具 `sidlet`，也提供 Rust 库函数，方便集成到其他项目中。

## 安装

### 方式一：安装脚本（推荐）

直接从 GitHub 执行安装脚本，自动下载最新的预编译二进制文件并完成安装。脚本会自动处理字体文件、路径配置等。

#### Windows（PowerShell）

在 PowerShell 中执行以下命令：

```powershell
iwr -Uri "https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.ps1" -OutFile "$env:TEMP\install.ps1"; & "$env:TEMP\install.ps1"
```

安装脚本会自动：
- 下载对应平台的最新发布版本
- 将 `sidlet.exe` 安装到 `%LOCALAPPDATA%\Programs\rsidlet\`
- 复制字体文件到安装目录
- 将安装目录添加到用户 `PATH` 环境变量

> **提示**：如果安装完成后命令未立即生效，请重新打开终端。

#### Linux / macOS（bash）

在终端中执行以下命令：

```bash
curl -fsSL https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.sh | bash
```

安装脚本会自动：
- 检测操作系统和架构
- 下载对应的最新发布版本
- 将 `sidlet` 安装到 `~/.local/bin/`
- 复制字体文件到安装目录

> **注意**：如果 `~/.local/bin` 不在你的 `PATH` 中，脚本会提示你添加。你也可以通过环境变量自定义安装目录：
> ```bash
> RSIDLET_INSTALL_DIR=/your/custom/path bash install.sh
> ```

#### 指定版本安装

如需安装特定版本，将版本号作为参数传递：

```bash
# Linux / macOS
curl -fsSL https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.sh | bash -s v1.1.0

# Windows PowerShell
$ver="v1.1.0"; iwr -Uri "https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.ps1" -OutFile "$env:TEMP\install.ps1"; & "$env:TEMP\install.ps1" -Version $ver
```

### 方式二：源码编译安装

如果预编译包不适用于你的平台，或者你需要使用最新开发版本，可以从源码编译安装。

#### 前提条件

- [Rust 工具链](https://rustup.rs/)（stable 版本）
- [Git](https://git-scm.com/)

#### 步骤

**1. 克隆仓库**

```bash
git clone https://github.com/SidneyLYZhang/rSIDlet.git
cd rSIDlet
```

**2. 编译并安装**

```bash
# 编译 release 版本
cargo build --release

# 安装到系统
make install
```

`make install` 会根据操作系统自动：
- 将编译好的二进制文件复制到安装目录
- 复制 `fonts/` 字体文件夹

> **手动安装说明**：如果不使用 `make install`，你需要手动将 `target/release/sidlet`（Windows 下为 `target\release\sidlet.exe`）复制到 PATH 中的某个目录，同时确保 `fonts/` 文件夹位于可执行文件同级目录或上一级目录。

**3. 字体文件夹特殊说明**

字体文件夹（`fonts/`）包含以下必要文件：

| 文件 | 说明 |
|------|------|
| `standard.flf` | 默认 FIGlet 字体 |
| `big.flf` | 大号 FIGlet 字体 |
| `phm-shinonome.flf` | PHM shinonome 字体 |
| `future.tlf` | TOIlet future 字体 |
| `HZK12` / `HZK14` / `HZK16` | 点阵中文字库 |

程序运行时会从以下位置按优先级搜索字体：
1. 可执行文件同级或上级目录的 `fonts/` 文件夹
2. 用户扩展字体目录（详见 [字体搜索路径](docs/SIDlet-manpage.md)）
3. 系统级 figlet 目录（`/usr/share/figlet` 等）

如果你将可执行文件单独移动到其他位置，请确保同目录下存在 `fonts/` 文件夹，或运行 `sidlet --test` 自动修复字体配置。

#### 各操作系统编译说明

<details>
<summary><b>Linux</b></summary>

```bash
# 安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 安装编译依赖（以 Ubuntu/Debian 为例）
sudo apt install build-essential pkg-config

# 克隆并编译
git clone https://github.com/SidneyLYZhang/rSIDlet.git
cd rSIDlet
cargo build --release
make install
```
</details>

<details>
<summary><b>macOS</b></summary>

```bash
# 安装 Xcode Command Line Tools（如未安装）
xcode-select --install

# 安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 克隆并编译
git clone https://github.com/SidneyLYZhang/rSIDlet.git
cd rSIDlet
cargo build --release
make install
```
</details>

<details>
<summary><b>Windows</b></summary>

```powershell
# 安装 Rust 工具链（下载 rustup-init.exe 从 https://rustup.rs/）

# 克隆仓库
git clone https://github.com/SidneyLYZhang/rSIDlet.git
cd rSIDlet

# 编译
cargo build --release

# 安装
make install
```

> 在 Windows 上也可以直接通过 Cargo 安装，然后运行 `sidlet --test` 修复字体：
> ```powershell
> cargo install rsidlet
> sidlet --test
> ```
</details>

### 验证安装

安装完成后，运行以下命令验证安装是否成功：

```bash
sidlet --version
```

如果安装正确，会输出当前版本号。

运行自检命令，确保字体文件完整：

```bash
sidlet --test
```

如果一切正常，会显示彩虹色的 `It's ready`。

测试渲染功能：

```bash
# 英文渲染
sidlet "Hello World"

# 中文渲染
sidlet "你好世界"
```

### 库函数

在 Rust 项目中使用，将 `rsidlet` 添加为依赖：

```bash
cargo add rsidlet
```

## 命令行使用

### 基本用法

```bash
# 使用默认字体（standard）渲染英文文本
sidlet "Hello World"

# 指定 figlet/toilet 字体
sidlet -f big.flf "Hello"

# 渲染中文文本（自动使用点阵字体或矢量字体）
sidlet "你好世界"

# 使用系统矢量字体渲染中文
sidlet -f simhei.ttf -s 16 "你好世界"

# 限制输出宽度，超宽自动换行
sidlet -w 40 "This is a long text"
```

### 颜色遮蔽

```bash
# 彩虹色（逐字符渐变）
sidlet -m rainbow "Rainbow Text"

# 火焰色
sidlet -m fire "Fire Text"

# 金属色
sidlet -m metal "Metal Text"

# 纯色（red/green/yellow/blue/magenta/cyan/white/black）
sidlet -m red "Red Text"
```

支持的颜色遮蔽：`none`、`rainbow`、`rainbowline`、`metal`、`fire`、`water`、`random`，以及 `red`、`green`、`yellow`、`blue`、`magenta`、`cyan`、`white`、`black`。

### 自定义前景/背景字符

```bash
# 使用 # 作为前景字符
sidlet --fore "#" --back "." "Hello"

# 适用于中文渲染
sidlet --fore "■" "你好"
```

### 字体管理

```bash
# 在线安装 figlet/toilet 字体
sidlet --install standard.flf
sidlet --install big.flf

# 从本地文件安装字体
sidlet --install myfont.flf -d /path/to/fonts

# 查看字体信息
sidlet --info big.flf
sidlet -i

# 列出已安装字体
sidlet --list installed

# 列出可在线下载的字体
sidlet --list letters

# 列出系统矢量字体
sidlet --list font

# 列出可用的颜色遮蔽
sidlet --list colormap

# 检测安装状态
sidlet --test
```

### 高级选项

```bash
# 使用 figlet control 文件
sidlet -C mycontrol.flc -f standard.flf "Hello"

# 指定额外字体搜索目录
sidlet -d /path/to/custom/fonts -f myfont.flf "Hello"

# 指定点阵字体大小（12/14/16）
sidlet -s 16 "你好世界"
```

## 库函数使用

### 基础渲染

```rust
use rsidlet::figfont;

// 加载并渲染 figlet/toilet 字体
let data = figfont::load_font_data("standard.flf")?;
let lines = data.render("Hello World");
for line in &lines {
    println!("{}", line);
}
```

### 中文渲染

```rust
use rsidlet::chilet;

// 使用 HZK 点阵字库渲染中文
if let Some(path) = chilet::find_hzk("HZK16") {
    let lines = chilet::render_hzk("你好", &path)?;
    for line in &lines { println!("{}", line); }
}

// 使用矢量字体渲染
let lines = chilet::render_vector_font("你好", "SimHei", 32.0)?;
for line in &lines { println!("{}", line); }

// 直接指定矢量字体文件路径
let lines = chilet::render_with_font_file("你好", &font_path, 24.0)?;
for line in &lines { println!("{}", line); }
```

### 颜色滤镜

```rust
use rsidlet::utils;

let filter = utils::parse_filter("rainbow").unwrap();
let lines = vec!["Hello".to_string()];
utils::print_colored(&lines, filter);
```

### 横向拼接

```rust
use rsidlet::utils;

let combined = utils::hcat(&left_lines, &right_lines, 4);
for line in &combined {
    println!("{}", line);
}
```

## 字体

### 自带字体

| 字体文件 | 类型 | 说明 |
|---------|------|------|
| `standard.flf` | FIGlet | 标准字体（默认） |
| `big.flf` | FIGlet | 大号字体 |
| `phm-shinonome.flf` | FIGlet | PHM shinonome 字体 |
| `future.tlf` | TOIlet | future 字体 |
| `HZK12` | BitmapFont | 12x12 点阵中文字库 |
| `HZK14` | BitmapFont | 14x14 点阵中文字库 |
| `HZK16` | BitmapFont | 16x16 点阵中文字库 |

### 在线安装更多字体

```bash
sidlet --install [font_name]
```

字体文件从以下 GitHub 仓库下载：[xero/figlet-fonts](https://github.com/xero/figlet-fonts) 和 [PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts)。

### 中文字体

除了自带的 HZK 点阵字库，也可以使用系统安装的矢量字体（TTF/OTF）进行渲染，具体效果因字体而异。经测试，Windows 下使用 `SimHei`（黑体）字体，在字号 12 时显示效果较好。

## Thanks

- [FIGlet](http://www.figlet.org/) — ASCII 艺术字体的开创项目
- [TOIlet](https://github.com/cacalabs/toilet) — FIGlet 的增强版本，支持颜色滤镜
- [BitmapFont](https://github.com/aguegu/BitmapFont) — 点阵中文字库
- [xero/figlet-fonts](https://github.com/xero/figlet-fonts) — FIGlet 字体合集
- [PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts) — 更多 FIGlet 字体

## License

[MIT License](LICENSE)
