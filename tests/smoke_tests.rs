//! 冒烟测试：覆盖 chilet、figfont、paths、utils 模块的关键功能。

use std::path::PathBuf;

use rsidlet::figfont::{self, FigFont, FigletFont, Glyph, ToiletFont};
use rsidlet::paths;
use rsidlet::utils::{self, ColorFilter};

// ============================================================
// helpers
// ============================================================

/// 获取项目 fonts/ 目录路径（测试运行时 cwd 为项目根目录）
fn project_font_dir() -> PathBuf {
    let cwd = std::env::current_dir().unwrap();
    let fonts = cwd.join("fonts");
    assert!(fonts.exists(), "fonts/ 目录不存在，测试无法运行");
    fonts
}

/// 获取 HZK 字体文件路径
fn hzk_path(name: &str) -> PathBuf {
    let p = project_font_dir().join(name);
    assert!(p.exists(), "HZK 文件不存在: {}", name);
    p
}

// ============================================================
// chilet 模块
// ============================================================

mod chilet_tests {
    use super::*;
    use rsidlet::chilet;

    #[test]
    fn find_hzk_returns_some_for_existing_files() {
        assert!(chilet::find_hzk("HZK12").is_some(), "HZK12 应被找到");
        assert!(chilet::find_hzk("HZK14").is_some(), "HZK14 应被找到");
        assert!(chilet::find_hzk("HZK16").is_some(), "HZK16 应被找到");
    }

    #[test]
    fn find_hzk_returns_none_for_missing_file() {
        assert!(chilet::find_hzk("HZK999").is_none());
    }

    #[test]
    fn find_hzk_returns_none_for_non_hzk_name() {
        assert!(chilet::find_hzk("not_a_hzk").is_none());
        assert!(chilet::find_hzk("standard.flf").is_none());
    }

    #[test]
    fn render_hzk_basic_chinese() {
        let path = hzk_path("HZK16");
        let lines = chilet::render_hzk("中文", &path).expect("渲染中文应成功");
        assert_eq!(lines.len(), 16, "HZK16 应输出 16 行");
        for line in &lines {
            assert!(!line.is_empty(), "每行都不应为空");
        }
    }

    #[test]
    fn render_hzk_with_ascii_mixed() {
        let path = hzk_path("HZK16");
        let lines = chilet::render_hzk("Hello世界", &path).expect("渲染混合文本应成功");
        assert_eq!(lines.len(), 16, "HZK16 应输出 16 行");
    }

    #[test]
    fn render_hzk_different_sizes() {
        for (name, expected_height) in &[("HZK12", 12), ("HZK14", 14), ("HZK16", 16)] {
            let path = hzk_path(name);
            let lines = chilet::render_hzk("测试", &path).expect("渲染应成功");
            assert_eq!(
                lines.len(),
                *expected_height,
                "{} 应输出 {} 行",
                name,
                expected_height
            );
        }
    }

    #[test]
    fn render_hzk_empty_string() {
        let path = hzk_path("HZK16");
        let lines = chilet::render_hzk("", &path).expect("渲染空字符串应成功");
        assert_eq!(lines.len(), 16, "空字符串仍应输出正确高度");
        for line in &lines {
            assert!(line.is_empty(), "空字符串每行都应为空");
        }
    }

    #[test]
    fn render_hzk_all_ascii() {
        let path = hzk_path("HZK16");
        let lines = chilet::render_hzk("ABC123", &path).expect("渲染纯 ASCII 应成功");
        assert_eq!(lines.len(), 16);
    }
}

// ============================================================
// figfont 模块
// ============================================================

mod figfont_tests {
    use super::*;

    // --- builtin fonts ---

    #[test]
    fn builtin_figlet_renders_text() {
        let font = FigletFont::builtin();
        let lines = font.render("AB");
        assert_eq!(lines.len(), 7, "内置字体高度应为 7");
        for line in &lines {
            assert!(!line.is_empty(), "每行都不应为空");
        }
    }

    #[test]
    fn builtin_toilet_renders_text() {
        let font = ToiletFont::builtin();
        let lines = font.render("AB");
        assert_eq!(lines.len(), 6, "内置 toilet 字体高度应为 6");
        for line in &lines {
            assert!(!line.is_empty(), "每行都不应为空");
        }
    }

    #[test]
    fn builtin_figlet_handles_unknown_chars() {
        let font = FigletFont::builtin();
        // 中文字符不在内置字体中，应回退到占位符
        let lines = font.render("你好");
        assert_eq!(lines.len(), 7);
    }

    #[test]
    fn builtin_figlet_handles_empty_string() {
        let font = FigletFont::builtin();
        let lines = font.render("");
        assert_eq!(lines.len(), 7);
        for line in &lines {
            assert!(line.is_empty());
        }
    }

    #[test]
    fn builtin_figlet_render_colored() {
        let font = FigletFont::builtin();
        let lines = font.render_colored("Hi", ColorFilter::Rainbow);
        assert_eq!(lines.len(), 7);
        // Rainbow 模式下每行应包含 ANSI 转义码
        assert!(lines[0].contains("\x1b["));
    }

    #[test]
    fn builtin_toilet_render_colored() {
        let font = ToiletFont::builtin();
        let lines = font.render_colored("Hi", ColorFilter::Metal);
        assert_eq!(lines.len(), 6);
        assert!(lines[0].contains("\x1b["));
    }

    // --- loading from font files ---

    #[test]
    fn load_figlet_from_file() {
        let path = project_font_dir().join("standard.flf");
        assert!(path.exists());
        let font = FigletFont::load(&path).expect("加载 standard.flf 应成功");
        let lines = font.render("Test");
        assert!(!lines.is_empty());
    }

    #[test]
    fn load_toilet_from_file() {
        let path = project_font_dir().join("future.tlf");
        assert!(path.exists());
        let font = ToiletFont::load(&path).expect("加载 future.tlf 应成功");
        // has_color_tags 取决于具体字体文件，不做断言
        let _ = font.has_color_tags;
        let lines = font.render("Test");
        assert!(!lines.is_empty());
    }

    #[test]
    fn load_font_data_auto_detect() {
        let path = project_font_dir().join("standard.flf");
        let data = figfont::load_font_data(&path).expect("加载字体数据应成功");
        assert!(data.header.signature.starts_with("flf"));
        let lines = data.render("OK");
        assert!(!lines.is_empty());
    }

    #[test]
    fn load_font_data_toilet() {
        let path = project_font_dir().join("future.tlf");
        let data = figfont::load_font_data(&path).expect("加载 toilet 数据应成功");
        assert!(data.header.signature.starts_with("tlf"));
    }

    // --- FontData ---

    #[test]
    fn font_data_get_glyph() {
        let font = FigletFont::builtin();
        let glyph = font.get_glyph('A').expect("应能找到字符 A 的字形");
        assert_eq!(glyph.codepoint, 65);
        assert!(glyph.is_valid());
    }

    #[test]
    fn font_data_get_glyph_missing() {
        let font = FigletFont::builtin();
        // 中文字符不应在内置字体中
        assert!(font.get_glyph('中').is_none());
    }

    #[test]
    fn font_data_char_width() {
        let font = FigletFont::builtin();
        let width = font.data().char_width('A');
        assert!(width > 0, "字母 A 的宽度应大于 0");
    }

    #[test]
    fn font_data_header_after_load() {
        let path = project_font_dir().join("standard.flf");
        let font = FigletFont::load(&path).expect("加载字体应成功");
        let header = font.header();
        assert_eq!(header.signature, "flf2a");
        assert!(header.height > 0);
    }

    #[test]
    fn font_data_comment_after_load() {
        let path = project_font_dir().join("big.flf");
        let data = figfont::load_font_data(&path).expect("加载 big.flf 应成功");
        let comment = data.comment.content();
        // big.flf 通常包含作者信息
        assert!(!comment.is_empty(), "big.flf 应有评论信息");
    }

    // --- Glyph ---

    #[test]
    fn glyph_placeholder() {
        let g = Glyph::placeholder(65, 5);
        assert_eq!(g.codepoint, 65);
        assert_eq!(g.height, 5);
        assert_eq!(g.width, 3);
    }

    #[test]
    fn glyph_is_valid_true() {
        let g = Glyph {
            codepoint: 65,
            lines: vec![" ## ".into(), "#  #".into()],
            width: 4,
            height: 2,
        };
        assert!(g.is_valid());
    }

    #[test]
    fn glyph_is_valid_false_for_all_whitespace() {
        let g = Glyph {
            codepoint: 32,
            lines: vec!["   ".into(), "   ".into()],
            width: 3,
            height: 2,
        };
        assert!(!g.is_valid());
    }

    #[test]
    fn glyph_line_bounds() {
        let g = Glyph {
            codepoint: 66,
            lines: vec!["a".into(), "b".into()],
            width: 1,
            height: 2,
        };
        assert_eq!(g.line(0), "a");
        assert_eq!(g.line(1), "b");
        assert_eq!(g.line(99), ""); // 越界返回空
    }

    #[test]
    fn glyph_render_with_hardblank() {
        let g = Glyph {
            codepoint: 67,
            lines: vec!["a$b".into()],
            width: 3,
            height: 1,
        };
        let rendered = g.render_with_hardblank('$');
        assert_eq!(rendered[0], "a b");
    }

    // --- FontHeader (via public API) ---

    #[test]
    fn font_header_parsed_correctly_from_file() {
        // 通过加载字体文件间接验证头部解析
        let font = FigletFont::builtin();
        let header = font.header();
        assert_eq!(header.signature, "flf2a");
        assert_eq!(header.hard_blank, '$');
        assert_eq!(header.height, 7);
        assert_eq!(header.baseline, 6);

        // 验证 toilet 字体头
        let tfont = ToiletFont::builtin();
        let theader = tfont.header();
        assert_eq!(theader.signature, "tlf2a");
        assert_eq!(theader.height, 6);
    }

    #[test]
    fn loaded_font_header_signature() {
        let path = project_font_dir().join("standard.flf");
        let font = FigletFont::load(&path).expect("加载 standard.flf 应成功");
        assert_eq!(font.header().signature, "flf2a");
        assert!(font.header().hard_blank != '\0');

        let tpath = project_font_dir().join("future.tlf");
        let tfont = ToiletFont::load(&tpath).expect("加载 future.tlf 应成功");
        assert_eq!(tfont.header().signature, "tlf2a");
    }

    // --- ToiletFont specifics ---

    #[test]
    fn toilet_font_has_unicode_detection() {
        let font = ToiletFont::builtin();
        // 内置字体 codetag_count 为 0
        assert!(!font.has_unicode);
    }

    #[test]
    fn toilet_font_supports_range() {
        let font = ToiletFont::builtin();
        // 内置字体支持 A-Z
        assert!(font.supports_range(65, 66));
        // 不支持中文字符范围
        assert!(!font.supports_range(0x4E00, 0x4E01));
    }

    #[test]
    fn toilet_font_color_tag_and_layout() {
        let font = ToiletFont::builtin();
        assert_eq!(font.color_tag_count(), 0);
        assert_eq!(font.full_layout_mode(), 1);
    }

    // --- FigFont trait methods ---

    #[test]
    fn figfont_trait_type_and_extensions() {
        assert_eq!(FigletFont::font_type(), "figlet");
        assert_eq!(FigletFont::extensions(), &["flf"]);
        assert_eq!(ToiletFont::font_type(), "toilet");
        assert_eq!(ToiletFont::extensions(), &["tlf"]);
    }

    #[test]
    fn figfont_parse_from_data() {
        // 用 minimal flf 数据测试 parse
        let data = b"flf2a$ 7 6 11 0 1 0 0 0\nBuiltin standard font\n \n \n \n \n \n \n \n";
        let font = FigletFont::parse(data).expect("解析最小 flf 数据应成功");
        assert_eq!(font.header().height, 7);
    }
}

// ============================================================
// paths 模块
// ============================================================

mod paths_tests {
    use super::*;

    #[test]
    fn builtin_font_dir_finds_fonts() {
        let dir = paths::builtin_font_dir();
        assert!(dir.is_some(), "应能找到 fonts/ 目录");
        let dir = dir.unwrap();
        assert!(dir.exists());
        assert!(dir.join("standard.flf").exists());
    }

    #[test]
    fn extended_font_dir_returns_path() {
        let dir = paths::extended_font_dir();
        assert!(dir.is_some(), "extended_font_dir 应返回路径");
    }

    #[test]
    fn font_search_paths_ordering() {
        let paths = paths::font_search_paths(None);
        assert!(!paths.is_empty(), "至少应有一个搜索路径");

        // 第一个路径应为内置 fonts/ 目录
        let first = &paths[0];
        assert!(
            first.to_string_lossy().contains("fonts"),
            "第一个路径应为 fonts 目录"
        );
    }

    #[test]
    fn font_search_paths_includes_extra_dir() {
        let extra = PathBuf::from("/tmp/test_fonts");
        let paths = paths::font_search_paths(Some(&extra));
        assert!(paths.contains(&extra), "应包含额外指定的目录");
    }

    #[test]
    fn find_font_file_exact_name() {
        let dirs = paths::font_search_paths(None);
        let found = paths::find_font_file("standard.flf", &dirs);
        assert!(found.is_some(), "应能找到 standard.flf");
    }

    #[test]
    fn find_font_file_stem_only() {
        let dirs = paths::font_search_paths(None);
        let found = paths::find_font_file("standard", &dirs);
        assert!(found.is_some(), "通过主干名应能找到 standard");
    }

    #[test]
    fn find_font_file_case_insensitive() {
        let dirs = paths::font_search_paths(None);
        let found = paths::find_font_file("STANDARD.FLF", &dirs);
        assert!(found.is_some(), "大小写不敏感匹配应成功");
    }

    #[test]
    fn find_font_file_missing() {
        let dirs = paths::font_search_paths(None);
        let found = paths::find_font_file("nonexistent_font_xyz.flf", &dirs);
        assert!(found.is_none());
    }

    #[test]
    fn list_flf_files_finds_expected() {
        let dirs = paths::font_search_paths(None);
        let files = paths::list_flf_files(&dirs);
        assert!(!files.is_empty(), "应找到至少一个 .flf/.tlf 文件");
        // 项目 fonts 目录中应包含 standard.flf
        let has_standard = files.iter().any(|f| f == "standard.flf");
        assert!(has_standard, "应包含 standard.flf");
    }

    #[test]
    fn system_font_dirs_not_empty() {
        let dirs = paths::system_font_dirs();
        assert!(!dirs.is_empty(), "系统字体目录列表不应为空");
    }

    #[test]
    fn list_system_ttf_files_does_not_panic() {
        // 不应 panic，无论是否找到字体
        let _files = paths::list_system_ttf_files();
    }

    #[test]
    fn home_dir_returns_some() {
        let home = paths::home_dir();
        assert!(home.is_some(), "应能找到用户主目录");
    }

    #[test]
    fn ensure_extended_font_dir_creates_and_returns_dir() {
        let result = paths::ensure_extended_font_dir();
        assert!(result.is_ok(), "应成功确保扩展字体目录存在");
        let dir = result.unwrap();
        assert!(dir.exists(), "返回的目录应存在");
    }
}

// ============================================================
// utils 模块
// ============================================================

mod utils_tests {
    use super::*;

    // --- contains_chinese ---

    #[test]
    fn contains_chinese_true() {
        assert!(utils::contains_chinese("你好"));
        assert!(utils::contains_chinese("Hello世界"));
        assert!(utils::contains_chinese("中文test"));
    }

    #[test]
    fn contains_chinese_false() {
        assert!(!utils::contains_chinese("Hello World"));
        assert!(!utils::contains_chinese("ABC123"));
        assert!(!utils::contains_chinese(""));
    }

    #[test]
    fn contains_chinese_rare_cjk() {
        // CJK Extension A 字符
        assert!(utils::contains_chinese("㐀"));
        // CJK Compatibility Ideographs
        assert!(utils::contains_chinese("豈"));
    }

    #[test]
    fn contains_chinese_japanese_not_chinese() {
        // 日文假名不是中文字符
        assert!(!utils::contains_chinese("あいうえお"));
        assert!(!utils::contains_chinese("カタカナ"));
    }

    // --- ColorFilter ---

    #[test]
    fn color_filter_none_preserves_lines() {
        let lines = vec!["hello".to_string(), "world".to_string()];
        let result = ColorFilter::None.apply(&lines);
        assert_eq!(result, lines);
    }

    #[test]
    fn color_filter_none_empty_lines() {
        let result = ColorFilter::None.apply(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn color_filter_foreground_applies_color() {
        let lines = vec!["test".to_string()];
        let result = ColorFilter::Foreground(utils::AnsiColor::Red).apply(&lines);
        assert!(result[0].starts_with("\x1b[31m"));
        assert!(result[0].ends_with("\x1b[0m"));
    }

    #[test]
    fn color_filter_rainbow_output_contains_ansi() {
        let lines = vec!["ABC".to_string()];
        let result = ColorFilter::Rainbow.apply(&lines);
        assert!(result[0].contains("\x1b["));
    }

    #[test]
    fn color_filter_rainbow_line_multiple_lines() {
        let lines = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = ColorFilter::RainbowLine.apply(&lines);
        assert_eq!(result.len(), 3);
        for line in &result {
            assert!(line.contains("\x1b["), "每行都应包含 ANSI 转义码");
        }
    }

    #[test]
    fn color_filter_rainbow_line_single_line() {
        let lines = vec!["solo".to_string()];
        let result = ColorFilter::RainbowLine.apply(&lines);
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("\x1b["));
    }

    #[test]
    fn color_filter_metal_fire_water() {
        let lines = vec!["X".to_string()];
        for filter in &[ColorFilter::Metal, ColorFilter::Fire, ColorFilter::Water] {
            let result = filter.apply(&lines);
            assert!(result[0].contains("\x1b["), "{:?} 应输出 ANSI 码", filter);
        }
    }

    #[test]
    fn color_filter_random() {
        let lines = vec!["test".to_string()];
        let result = ColorFilter::Random.apply(&lines);
        assert!(result[0].contains("\x1b["));
    }

    #[test]
    fn color_filter_gradient() {
        let lines = vec!["a".to_string(), "b".to_string()];
        let filter = ColorFilter::Gradient {
            start: utils::AnsiColor::Red,
            end: utils::AnsiColor::Blue,
        };
        let result = filter.apply(&lines);
        assert_eq!(result.len(), 2);
        assert!(result[0].contains("\x1b["));
        assert!(result[1].contains("\x1b["));
    }

    #[test]
    fn color_filter_apply_to_line_basics() {
        let line = "test";
        // None
        assert_eq!(ColorFilter::None.apply_to_line(line, 0, 1), line);
        // Foreground
        let colored = ColorFilter::Foreground(utils::AnsiColor::Green).apply_to_line(line, 0, 1);
        assert!(colored.starts_with("\x1b[32m"));
        // Rainbow
        let rb = ColorFilter::Rainbow.apply_to_line("abc", 0, 1);
        assert!(rb.contains("\x1b["));
    }

    #[test]
    fn color_filter_apply_to_line_empty() {
        let result = ColorFilter::Rainbow.apply_to_line("", 0, 1);
        assert_eq!(result, "");
    }

    // --- parse_filter ---

    #[test]
    fn parse_filter_valid_names() {
        let cases = [
            ("none", ColorFilter::None),
            ("rainbow", ColorFilter::Rainbow),
            ("rainbowline", ColorFilter::RainbowLine),
            ("metal", ColorFilter::Metal),
            ("fire", ColorFilter::Fire),
            ("water", ColorFilter::Water),
            ("random", ColorFilter::Random),
        ];
        for (name, expected_variant) in &cases {
            let parsed = utils::parse_filter(name);
            assert!(parsed.is_some(), "应能解析 '{}'", name);
            // 比较 variant 类型（忽略内部参数差异）
            assert_eq!(
                std::mem::discriminant(&parsed.unwrap()),
                std::mem::discriminant(expected_variant),
                "解析 '{}' 应为对应 variant",
                name
            );
        }
    }

    #[test]
    fn parse_filter_color_names() {
        for name in &["red", "green", "blue", "yellow", "cyan", "magenta", "white", "black"] {
            let parsed = utils::parse_filter(name);
            assert!(parsed.is_some(), "应能解析颜色名 '{}'", name);
        }
    }

    #[test]
    fn parse_filter_case_insensitive() {
        assert!(utils::parse_filter("RAINBOW").is_some());
        assert!(utils::parse_filter("RainBow").is_some());
        assert!(utils::parse_filter("Red").is_some());
    }

    #[test]
    fn parse_filter_invalid() {
        assert!(utils::parse_filter("").is_some()); // "" maps to None
        assert!(utils::parse_filter("bogus_filter_xyz").is_none());
    }

    // --- available_filters ---

    #[test]
    fn available_filters_has_expected() {
        let filters = utils::available_filters();
        assert!(filters.contains(&"rainbow"));
        assert!(filters.contains(&"metal"));
        assert!(filters.contains(&"fire"));
        assert!(filters.contains(&"water"));
        assert!(filters.contains(&"random"));
        assert!(filters.contains(&"red"));
        assert!(filters.contains(&"none"));
    }

    // --- hcat ---

    #[test]
    fn hcat_same_height() {
        let left = vec!["AAA".to_string(), "BBB".to_string()];
        let right = vec!["XXX".to_string(), "YYY".to_string()];
        let result = utils::hcat(&left, &right, 2);
        assert_eq!(result, vec!["AAA  XXX", "BBB  YYY"]);
    }

    #[test]
    fn hcat_different_heights() {
        let left = vec!["AAA".to_string(), "BBB".to_string(), "CCC".to_string()];
        let right = vec!["XXX".to_string()];
        let result = utils::hcat(&left, &right, 1);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "AAA XXX");
        assert_eq!(result[1], "BBB ");
        assert_eq!(result[2], "CCC ");
    }

    #[test]
    fn hcat_empty() {
        let result = utils::hcat(&[], &[], 0);
        assert!(result.is_empty());
    }

    #[test]
    fn hcat_zero_padding() {
        let left = vec!["A".to_string()];
        let right = vec!["B".to_string()];
        let result = utils::hcat(&left, &right, 0);
        assert_eq!(result, vec!["AB"]);
    }

    // --- print_colored (does not panic) ---

    #[test]
    fn print_colored_does_not_panic() {
        let lines = vec!["test".to_string()];
        utils::print_colored(&lines, ColorFilter::Rainbow);
        utils::print_colored(&lines, ColorFilter::None);
        utils::print_colored(&[], ColorFilter::Metal);
    }

    // --- AnsiColor ---

    #[test]
    fn ansi_color_values() {
        assert_eq!(utils::AnsiColor::Red as u8, 31);
        assert_eq!(utils::AnsiColor::Blue as u8, 34);
        assert_eq!(utils::AnsiColor::Reset as u8, 0);
        assert_eq!(utils::AnsiColor::BrightGreen as u8, 92);
    }
}

// ============================================================
// 集成测试：端到端渲染管线
// ============================================================

#[test]
fn end_to_end_standard_flf_render() {
    let dirs = paths::font_search_paths(None);
    let path = paths::find_font_file("standard", &dirs).expect("应找到 standard 字体");
    let data = figfont::load_font_data(&path).expect("应成功加载字体");
    let lines = data.render("It works!");
    assert!(!lines.is_empty());
    // 应用颜色滤镜不应 panic
    let colored = ColorFilter::Rainbow.apply(&lines);
    assert_eq!(colored.len(), lines.len());
}

#[test]
fn end_to_end_hzk_render_with_color() {
    let path = hzk_path("HZK16");
    let lines = rsidlet::chilet::render_hzk("你好", &path).expect("HZK 渲染应成功");
    let colored = ColorFilter::Fire.apply(&lines);
    assert_eq!(colored.len(), lines.len());
}

#[test]
fn builtin_fallback_works_for_all_printable_ascii() {
    let font = FigletFont::builtin();
    let text: String = (32u8..=126).map(char::from).collect();
    let lines = font.render(&text);
    assert_eq!(lines.len(), 7);
    // 所有行都不应为空
    for line in &lines {
        assert!(!line.is_empty(), "内置字体应渲染所有可打印 ASCII");
    }
}
