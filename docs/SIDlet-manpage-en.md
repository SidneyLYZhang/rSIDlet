# SIDLET(1) -- Convert text to ASCII art

## NAME

**sidlet** -- Convert text to ASCII art, supporting traditional FIGlet/TOIlet English fonts as well as Chinese bitmap and vector fonts.

## SYNOPSIS

```
sidlet [options...] [message]
```

## DESCRIPTION

**sidlet** reads user input text and converts it into large characters made up of ordinary ASCII characters (such as `#`, `█`, etc.), outputting the result. Its output style is reminiscent of FIGlet and TOIlet.

**sidlet** supports multiple font formats, including traditional FIGlet fonts (`.flf`), TOIlet fonts (`.tlf`), as well as HZK bitmap fonts and system vector fonts (`.ttf`, `.otf`) for Chinese rendering. When the input text contains Chinese characters, **sidlet** automatically switches to Chinese rendering mode (Chilet mode).

**sidlet** also supports color filters (Color Filter), which add ANSI color effects to ASCII art output, enabling visual effects such as rainbow, fire, and metal gradients.

## OPTIONS

**sidlet** reads command line options from left to right, and only the last option that affects a parameter has any effect.

### Font Options

**-f** *fontfile*, **--font** *fontfile*
:   Select the font file or font name. The file extension may be omitted; **sidlet** will automatically try `.flf`, `.tlf`, `.ttf`, `.otf` and other formats. Defaults to `standard.flf`.

**-d** *fontdirectory*, **--directory** *fontdirectory*
:   Specify an additional local font search directory. This directory has lower search priority than the built-in font directory, the extended font directory, and system-level figlet directories.

**-s** *size*, **--size** *size*
:   Specify the font size (in pixels) for Chilet mode. Any floating-point value is accepted. When the value is 12, 14, or 16, HZK bitmap fonts (HZK12/HZK14/HZK16) are used preferentially; other values will automatically fall back to system vector font rendering. Defaults to `12`.

### Output Control Options

**-w** *outputwidth*, **--width** *outputwidth*
:   Set the maximum output width (number of columns). When the rendered result exceeds this width, **sidlet** automatically performs binary search to segment the input text, renders each segment separately, and concatenates them vertically. By default there is no width limit (render as-is).

**-m** *maskcolor*, **--maskcolor** *maskcolor*
:   Specify a color filter (Color Filter) to add ANSI color effects to the ASCII art output. Available values include `none`, `rainbow`, `rainbowline`, `metal`, `fire`, `water`, `random`, as well as `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, `black`.

**-C** *controlfile*, **--control** *controlfile*
:   Specify a FIGlet Control file (`.flc`) for character mapping conversion of the input text before rendering. The Control file format is compatible with FIGlet.

**--fore** *foreground*
:   Specify the foreground character for Chilet mode. Defaults to `█`.

**--back** *background*
:   Specify the background character for Chilet mode. Defaults to ` ` (space).

### Information and Management Commands

**--install** [*fontfile*]
:   Download and install a specified font file from online font repositories (GitHub). If *fontfile* is omitted, it should be used in combination with other parameters. Font files are installed to the extended font directory (`%USERPROFILE%\fonts` on Windows, `$XDG_DATA_HOME/figlet` or `~/.local/share/figlet` on Linux, `~/Library/Application Support/figlet` on macOS). Supports `.flf`, `.tlf`, and HZK Chinese font file downloads.

    Online font sources:
    - **FIGlet fonts**: from [xero/figlet-fonts](https://github.com/xero/figlet-fonts) and [PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts)
    - **HZK Chinese fonts**: from [aguegu/BitmapFont](https://github.com/aguegu/BitmapFont)

**-i** [*fontfile*], **--info** [*fontfile*]
:   Display font information. If a font name is specified, outputs the font's header parameters (signature, height, baseline, max width, layout mode, etc.), comment information, and an ASCII art rendering of the font name using the font itself. If the font name is omitted, lists all font search directories and their statuses.

**--list** *type*
:   List resources of a specific type. *type* can be one of the following:

    - `font`: List available vector fonts (TTF/OTF) in the system
    - `colormap`: List all available color filter names
    - `installed`: List font files installed in the current font search paths
    - `letters`: List font file names available for download online but not yet installed locally (separated by `;`)

**--test**
:   Check font installation status. If the built-in font directory (Directory A) or extended font directory (Directory B) is missing, or required base font files are incomplete, prompt the user whether to repair. The repair process automatically creates the extended font directory and downloads missing essential font files (standard.flf, big.flf, future.tlf, phm-shinonome.flf, and HZK12/HZK14/HZK16). After repair is complete, outputs `It's ready` in rainbow color.

### General Options

**--help**
:   Display help information and exit.

**--version**
:   Display version information and exit.

## USAGE

Simply type `sidlet` on the command line followed by the text to render. You can also pipe input, or place input on the command line after the options. See the examples below.

## COLOR FILTERS

**sidlet** supports the following color filters (specified via the `-m`/`--maskcolor` flag):

| Filter Name | Description |
|-------------|-------------|
| `none` | No color (default) |
| `rainbow` | Per-character rainbow gradient |
| `rainbowline` | Per-line rainbow gradient |
| `metal` | Metal gradient (black to bright white) |
| `fire` | Fire gradient (red to bright yellow) |
| `water` | Water gradient (blue to bright cyan) |
| `random` | Random color (hash-based per line) |
| `red` | Solid red |
| `green` | Solid green |
| `yellow` | Solid yellow |
| `blue` | Solid blue |
| `magenta` | Solid magenta |
| `cyan` | Solid cyan |
| `white` | Solid white |
| `black` | Solid black |

## FONTS

### Built-in Fonts

| Font File | Type | Description |
|-----------|------|-------------|
| `standard.flf` | FIGlet | Standard font (default), FIGlet classic |
| `big.flf` | FIGlet | Large display font |
| `phm-shinonome.flf` | FIGlet | PHM shinonome font |
| `future.tlf` | TOIlet | Future font (TOIlet classic) |
| `HZK12` | BitmapFont | 12×12 bitmap Chinese font |
| `HZK14` | BitmapFont | 14×14 bitmap Chinese font |
| `HZK16` | BitmapFont | 16×16 bitmap Chinese font |

### Installing More Fonts Online

```
sidlet --install [font_name.flf]
```

**sidlet** can download online fonts from the following GitHub repositories:

- **[xero/figlet-fonts](https://github.com/xero/figlet-fonts)**: A large collection of community-contributed FIGlet fonts.
- **[PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts)**: FIGlet font collection.

Downloaded font files are stored in the extended font directory:

- **Windows**: `%USERPROFILE%\fonts`
- **Linux**: `$XDG_DATA_HOME/figlet` or `~/.local/share/figlet`
- **macOS**: `~/Library/Application Support/figlet`

### Chinese Rendering Support

**sidlet** automatically enables Chinese rendering (Chilet mode) in the following two situations:

1. **Input text contains Chinese characters**: automatically uses HZK bitmap fonts for rendering.
2. **A vector font is specified** (`.ttf`/`.otf`): uses the fontdue engine to rasterize vector glyphs into a bitmap and output.

Chinese rendering supports custom foreground and background characters (via the `--fore` and `--back` flags).

## FONT SEARCH PATHS

**sidlet** searches for font files in the following priority order:

1. **Built-in font directory (Directory A)**: The `../fonts` subdirectory relative to the executable, or `../fonts`/`fonts` relative to the current working directory.
2. **Extended font directory (Directory B)**:
   - Windows: `%USERPROFILE%\fonts`
   - Linux: `$XDG_DATA_HOME/figlet` or `~/.local/share/figlet`
   - macOS: `~/Library/Application Support/figlet`
3. **System-level figlet directories** (read-only, for fonts installed by package managers):
   - Linux/macOS: `/usr/share/figlet`, `/usr/local/share/figlet`
4. **User-specified directory**: An additional directory specified via the `-d`/`--directory` flag.

## FILE FORMATS

**sidlet** supports the following font file formats:

| Extension | Format | Description |
|-----------|--------|-------------|
| `.flf` | FIGlet Font | FIGlet font file, signature `flf2a` |
| `.tlf` | TOIlet Font | TOIlet font file, signature `tlf2a`, supports Unicode and color tags |
| `.flc` | FIGlet Control | FIGlet Control file for character mapping |
| `.ttf` / `.otf` | TrueType/OpenType | Vector font for Chinese/Unicode character rendering |
| `.bdf` | BDF Bitmap | Bitmap font (Glyph Bitmap Distribution Format), usable for Chinese/Unicode character rendering |
| `HZK*` | HZK Bitmap | Chinese bitmap font (no extension), GB2312 encoding |

## EXAMPLES

### Basic English Rendering

```
# Render with the default font
sidlet "Hello World"

# Specify a FIGlet font
sidlet -f big.flf "Hello"

# Use a TOIlet font
sidlet -f future.tlf "Hello"
```

### Chinese Rendering

```
# Auto-detect Chinese and render with bitmap font
sidlet "你好世界"

# Use 16-dot bitmap font
sidlet -s 16 "你好世界"

# Render Chinese using a system vector font
sidlet -f simhei.ttf -s 24 "你好世界"
```

### Color Effects

```
# Rainbow effect
sidlet -m rainbow "Rainbow Text"

# Fire effect
sidlet -m fire "Fire Text"

# Solid blue effect
sidlet -m blue "Blue Text"
```

### Width Control

```
# Limit output width to 40 columns
sidlet -w 40 "This is a very long text that will be wrapped"
```

### Custom Foreground/Background

```
# Use # as foreground, . as background
sidlet --fore "#" --back "." "Hello"

# Also works with Chinese
sidlet --fore "■" "你好"
```

### Font Management

```
# Install a font online
sidlet --install big.flf

# View font information
sidlet --info big.flf

# List installed fonts
sidlet --list installed

# List fonts available for download
sidlet --list letters

# List available color filters
sidlet --list colormap

# Check and repair installation status
sidlet --test
```

### Input from File or Pipe

```
# Read and render from a file
sidlet < myfile.txt

# Pipe input
echo "Hello" | sidlet -f standard.flf

# Combine options
sidlet -f big.flf -m rainbow -w 60 "Hello World"
```

### Using a Control File

```
sidlet -C 8859-8.flc -f standard.flf "Shalom"
```

## LIBRARY USAGE

**sidlet** is also available as the Rust library `rsidlet`, which can be integrated into other Rust projects.

### Basic Rendering

```rust
use rsidlet::figfont::{FigFont, FigletFont, ToiletFont};

// Load a FIGlet font
let font = FigletFont::load("fonts/standard.flf")?;
let lines = font.render("Hello World");
for line in &lines {
    println!("{}", line);
}

// Load a TOIlet font
let toilet_font = ToiletFont::load("fonts/future.tlf")?;
let lines = toilet_font.render("Hello");
for line in &lines {
    println!("{}", line);
}
```

### Chinese Rendering

```rust
use rsidlet::chilet::{self, RenderOptions};

// Use the unified rendering entry point (auto-selects HZK or vector font)
let options = RenderOptions::default()
    .with_size(16)
    .chars('█', ' ');

let lines = chilet::render("你好世界", &options)?;
for line in &lines {
    println!("{}", line);
}

// Use HZK bitmap font directly (requires specifying foreground/background chars)
if let Some(path) = chilet::find_hzk("HZK16") {
    let lines = chilet::render_hzk("你好", &path, '█', ' ')?;
    for line in &lines {
        println!("{}", line);
    }
}

// Use a system vector font
let options = RenderOptions::default()
    .with_size(32)
    .font("SimHei")
    .chars('█', ' ');

let lines = chilet::render("你好", &options)?;
for line in &lines {
    println!("{}", line);
}
```

### Color Filters

```rust
use rsidlet::utils;

let filter = utils::parse_filter("rainbow").unwrap();
let lines = vec!["Hello".to_string()];
utils::print_colored(&lines, filter);
```

### Horizontal Concatenation

```rust
use rsidlet::utils;

let combined = utils::hcat(&left_lines, &right_lines, 4);
for line in &combined {
    println!("{}", line);
}
```

## ENVIRONMENT

**sidlet** depends on the following environment variables:

`USERPROFILE` (Windows) or `HOME` (Linux/macOS)
:   Used to determine the user's home directory, and subsequently the extended font directory.

`XDG_DATA_HOME` (Linux)
:   If set, overrides `~/.local/share` as the base path for the extended font directory. If not set, defaults to `~/.local/share/figlet`.

`WINDIR` (Windows)
:   Used to locate the system font directory (`%WINDIR%\Fonts`).

## EXIT STATUS

| Status | Meaning |
|--------|---------|
| `0` | Success |
| `1` | Error occurred (font not found, missing input, etc.) |

## BUGS

- Rendering may be suboptimal for some text containing full-width characters or complex Unicode combinations.
- External font download functionality that relies on online GitHub repositories may fail due to network fluctuations or API rate limiting.
- HZK bitmap fonts only support Chinese characters within the GB2312 encoding range.
- In Chinese rendering mode, FIGlet smushing and kerning features are not available.

Please submit bug reports to the project repository:
<https://github.com/SidneyLYZhang/rSIDlet/issues>


---

v1.1.0+, 2026 -- SIDLET(1)
