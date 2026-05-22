# SIDLET(1) -- 将文本转换为 ASCII 艺术字

## 名称 (NAME)

**sidlet** -- 将文本转换为 ASCII 艺术字，支持传统 FIGlet/TOIlet 英文字体及中文点阵/矢量字体。

## 概要 (SYNOPSIS)

```
sidlet [选项...] [message]
```

## 描述 (DESCRIPTION)

**sidlet** 读取用户输入的文本，将其转换为由普通 ASCII 字符（如 `#`、`█` 等）组成的大号字形并输出。其输出风格类似于 FIGlet 和 TOIlet。

**sidlet** 支持多种字体格式，包括传统的 FIGlet 字体（`.flf`）、TOIlet 字体（`.tlf`），以及用于中文渲染的 HZK 点阵字库和系统矢量字体（`.ttf`、`.otf`）。当输入文本包含中文字符时，**sidlet** 将自动切换到中文渲染模式（Chilet 模式）。

**sidlet** 还支持颜色遮蔽（Color Filter），能够为 ASCII 艺术输出添加 ANSI 颜色效果，实现彩虹色、火焰色、金属色等多种视觉效果。

## 选项 (OPTIONS)

**sidlet** 从左到右解析命令行选项，只有最后一个影响某参数的选项才会生效。

### 字体选项

**-f** *fontfile*, **--font** *fontfile*
:   指定字体文件或字体名称。音译扩展名可省略，**sidlet** 将自动尝试 `.flf`、`.tlf`、`.ttf`、`.otf` 等格式。默认使用 `standard.flf`。

**-d** *fontdirectory*, **--directory** *fontdirectory*
:   指定额外的本地字体搜索目录。该目录的搜索优先级低于内置字体目录和系统字体目录。

**-s** *size*, **--size** *size*
:   指定 Chilet 模式下点阵字体的大小。支持 12、14、16（对应 HZK12/HZK14/HZK16）。默认值为 `12`。仅对中文点阵渲染和矢量字体渲染生效。

### 输出控制选项

**-w** *outputwidth*, **--width** *outputwidth*
:   设置最大输出宽度（列数）。当渲染结果超出此宽度时，**sidlet** 会自动按二分查找将输入文本分段，分别渲染后纵向拼接。默认无宽度限制（按图渲染）。

**-m** *maskcolor*, **--maskcolor** *maskcolor*
:   指定颜色遮蔽（Color Filter），为 ASCII 艺术输出添加 ANSI 颜色效果。可用值包括 `none`、`rainbow`、`rainbowline`、`metal`、`fire`、`water`、`random`，以及 `red`、`green`、`yellow`、`blue`、`magenta`、`cyan`、`white`、`black`。

**-C** *controlfile*, **--control** *controlfile*
:   指定 FIGlet Control 文件（`.flc`），用于在渲染前对输入文本进行字符映射转换。Control 文件的格式与 FIGlet 兼容。

**--fore** *foreground*
:   指定 Chilet 模式下的前景字符。默认值为 `█`。

**--back** *background*
:   指定 Chilet 模式下的背景字符。默认值为 ` `（空格）。

### 信息与管理命令

**--install** [*fontfile*]
:   从在线字体仓库（GitHub）下载并安装指定的字体文件。若省略 fontfile，则需配合其他参数使用。字体文件将安装到扩展字体目录（Windows 下为 `%USERPROFILE%\fonts`，Linux/macOS 下为 `/usr/share/figlet`）。支持 `.flf`、`.tlf` 以及 HZK 中文字库文件的下载。
    
    在线字体来源：
    - **FIGlet 字体**：来自 [xero/figlet-fonts](https://github.com/xero/figlet-fonts) 和 [PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts)
    - **HZK 中文字库**：来自 [aguegu/BitmapFont](https://github.com/aguegu/BitmapFont)

**-i** [*fontfile*], **--info** [*fontfile*]
:   显示字体信息。若指定字体名，则输出该字体的头部参数（签名、高度、基线、最大宽度、布局模式等）、评论信息和用该字体渲染的字体名 ASCII 图形。若省略字体名，则列出所有字体搜索目录及其状态。

**--list** *type*
:   列出特定类型的资源。*type* 可以是以下值：
    
    - `font`：列出系统中可用的矢量字体（TTF/OTF）
    - `colormap`：列出所有可用的颜色遮蔽名称
    - `installed`：列出当前字体搜索路径中已安装的字体文件
    - `letters`：列出在线可下载但本地尚未安装的字体文件名

**--test**
:   检查字体目录安装状态。若内置字体目录（目录A）或扩展字体目录（目录B）缺失，将提示用户是否进行修复。修复过程会自动下载缺失的必要字体文件（standard.flf、big.flf、future.tlf、phm-shinonome.flf 以及 HZK12/HZK14/HZK16）。修复完成后，以彩虹色输出 `It's ready`。

### 通用选项

**--help**
:   显示帮助信息并退出。

**--version**
:   显示版本信息并退出。

## 用法 (USAGE)

直接在命令行输入 `sidlet` 后跟要渲染的文本。也可以通过管道输入，或将输入放在命令行参数的选项之后。参见下方的示例。

## 颜色遮蔽 (COLOR FILTERS)

**sidlet** 支持以下颜色遮蔽（通过 `-m`/`--maskcolor` 参数指定）：

| 遮蔽名称 | 说明 |
|----------|------|
| `none` | 无颜色（默认） |
| `rainbow` | 逐字符彩虹色渐变 |
| `rainbowline` | 逐行彩虹色渐变 |
| `metal` | 金属色渐变（黑 → 亮白） |
| `fire` | 火焰色渐变（红 → 亮黄） |
| `water` | 水色渐变（蓝 → 亮青） |
| `random` | 随机颜色（基于行内容的哈希） |
| `red` | 纯红色 |
| `green` | 纯绿色 |
| `yellow` | 纯黄色 |
| `blue` | 纯蓝色 |
| `magenta` | 纯品红色 |
| `cyan` | 纯青色 |
| `white` | 纯白色 |
| `black` | 纯黑色 |

## 字体 (FONTS)

### 内置字体

| 字体文件 | 类型 | 说明 |
|----------|------|------|
| `standard.flf` | FIGlet | 标准字体（默认），FIGlet 经典字体 |
| `big.flf` | FIGlet | 大号显示字体 |
| `phm-shinonome.flf` | FIGlet | PHM 东云字体 |
| `future.tlf` | TOIlet | Future 字体（TOIlet 经典字体） |
| `HZK12` | BitmapFont | 12×12 点阵中文字库 |
| `HZK14` | BitmapFont | 14×14 点阵中文字库 |
| `HZK16` | BitmapFont | 16×16 点阵中文字库 |

### 在线安装更多字体

```
sidlet --install [font_name.flf]
```

**sidlet** 可以从以下 GitHub 仓库下载在线字体：

- **[xero/figlet-fonts](https://github.com/xero/figlet-fonts)**：收录了大量社区贡献的 FIGlet 字体。
- **[PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts)**：FIGlet 字体合集。

下载后的字体文件存放在扩展字体目录中：

- **Windows**：`%USERPROFILE%\fonts`
- **Linux/macOS**：`/usr/share/figlet`

### 中文渲染支持

**sidlet** 在以下两种情形下自动启用中文渲染（Chilet 模式）：

1. **输入文本包含中文字符**：自动使用 HZK 点阵字库渲染。
2. **指定了矢量字体**（`.ttf`/`.otf`）：使用 fontdue 引擎将矢量字形光栅化为点阵后输出。

中文渲染支持自定义前景和背景字符（通过 `--fore` 和 `--back` 参数）。

## 字体搜索路径

**sidlet** 按以下优先级搜索字体文件：

1. **内置字体目录（目录A）**：可执行文件所在目录的 `../fonts` 子目录，或当前工作目录的 `../fonts`、`fonts` 子目录。
2. **扩展字体目录（目录B）**：
   - Windows：`%USERPROFILE%\fonts`
   - Linux/macOS：`/usr/share/figlet` 或 `/usr/local/share/figlet`
3. **用户指定目录**：通过 `-d`/`--directory` 参数指定的额外目录。

## 文件格式 (FILE FORMATS)

**sidlet** 支持以下字体文件格式：

| 扩展名 | 格式 | 说明 |
|--------|------|------|
| `.flf` | FIGlet Font | FIGlet 字体文件，签名 `flf2a` |
| `.tlf` | TOIlet Font | TOIlet 字体文件，签名 `tlf2a`，支持 Unicode 和颜色标记 |
| `.flc` | FIGlet Control | FIGlet Control 文件，用于字符映射 |
| `.ttf` / `.otf` | TrueType/OpenType | 矢量字体，用于中文/统码字符的渲染 |
| `HZK*` | HZK Bitmap | 中文点阵字库（无扩展名），GB2312 编码 |

## 示例 (EXAMPLES)

### 基本英文渲染

```
# 使用默认字体渲染
sidlet "Hello World"

# 指定 FIGlet 字体
sidlet -f big.flf "Hello"

# 使用 TOIlet 字体
sidlet -f future.tlf "Hello"
```

### 中文渲染

```
# 自动检测中文并使用点阵字体渲染
sidlet "你好世界"

# 使用 16 号点阵字体
sidlet -s 16 "你好世界"

# 使用系统矢量字体渲染中文
sidlet -f simhei.ttf -s 24 "你好世界"
```

### 颜色效果

```
# 彩虹色效果
sidlet -m rainbow "Rainbow Text"

# 火焰色效果
sidlet -m fire "Fire Text"

# 纯蓝色效果
sidlet -m blue "Blue Text"
```

### 宽度控制

```
# 限制输出宽度为 40 列
sidlet -w 40 "This is a very long text that will be wrapped"
```

### 自定义前景/背景

```
# 使用 # 作为前景，. 作为背景
sidlet --fore "#" --back "." "Hello"

# 中文也可自定义
sidlet --fore "■" "你好"
```

### 字体管理

```
# 在线安装字体
sidlet --install big.flf

# 查看字体信息
sidlet --info big.flf

# 列出已安装字体
sidlet --list installed

# 列出可下载的字体
sidlet --list letters

# 列出可用的颜色遮蔽
sidlet --list colormap

# 检查并修复安装状态
sidlet --test
```

### 从文件或管道输入

```
# 从文件读取并渲染
sidlet < myfile.txt

# 管道输入
echo "Hello" | sidlet -f standard.flf

# 组合选项
sidlet -f big.flf -m rainbow -w 60 "Hello World"
```

### 使用 Control 文件

```
sidlet -C 8859-8.flc -f standard.flf "Shalom"
```

## 作为 Rust 库使用 (LIBRARY USAGE)

**sidlet** 同时也是 Rust 库 `rsidlet`，可以集成到其他 Rust 项目中。

### 基础渲染

```rust
use rsidlet::figfont;

let data = figfont::load_font_data("standard.flf")?;
let lines = data.render("Hello World");
for line in &lines {
    println!("{}", line);
}
```

### 中文渲染

```rust
use rsidlet::chilet;

// HZK 点阵字库
if let Some(path) = chilet::find_hzk("HZK16") {
    let lines = chilet::render_hzk("你好", &path)?;
    for line in &lines { println!("{}", line); }
}

// 矢量字体
let lines = chilet::render_vector_font("你好", "SimHei", 32.0)?;
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

## 环境 (ENVIRONMENT)

**sidlet** 依赖以下环境变量：

`USERPROFILE`（Windows）或 `HOME`（Linux/macOS）
:   用于确定用户主目录，进而定位扩展字体目录。

`WINDIR`（Windows）
:   用于定位系统字体目录（`%WINDIR%\Fonts`）。

## 退出状态 (EXIT STATUS)

| 状态码 | 含义 |
|--------|------|
| `0` | 成功 |
| `1` | 发生错误（字体未找到、输入缺失等） |

## 已知问题 (BUGS)

- 对于某些包含全角字符或复杂 Unicode 组合的文本，渲染效果可能不够理想。
- 依赖于在线 GitHub 仓库的外部字体下载功能可能因网络波动或 API 限流而失败。
- HZK 点阵字库仅支持 GB2312 编码范围内的中文字符。
- 在中文渲染模式下，FIGlet 的 smushing（挤压）和 kerning（字距调整）特性不可用。

请将错误报告提交至项目仓库：
<https://github.com/SidneyLYZhang/rSIDlet/issues>



---

v1.0.5, 2025 -- SIDLET(1)
