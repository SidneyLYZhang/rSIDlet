# rSIDlet

中文 | [English](README_en.md)

`figlet` 在 Rust 上的简单实现，并添加了中文支持。中文支持通过点阵字库（HZK）和矢量字体（TTF/OTF）两种方式实现。

本项目既提供命令行工具 `sidlet`，也提供 Rust 库函数，方便集成到其他项目中。

## 安装

### 命令行工具

```bash
cargo install rsidlet
```

### 库函数

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
