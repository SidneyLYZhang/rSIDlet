use std::path::{Path, PathBuf};

/// 获取内置字体目录（编译后可执行文件同级的 ../fonts）
///
/// 按优先级依次尝试：
/// 1. exe/../fonts（编译后安装场景）
/// 2. cwd/../fonts（开发场景）
/// 3. cwd/fonts（直接子目录）
pub fn builtin_font_dir() -> Option<PathBuf> {
    // 编译后场景：可执行文件同级
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let path = parent.join("..").join("fonts");
            if path.exists() {
                return Some(path);
            }
        }
    }

    // 开发时回退
    if let Ok(cwd) = std::env::current_dir() {
        let cwd_parent = cwd.join("..").join("fonts");
        if cwd_parent.exists() {
            return Some(cwd_parent);
        }
        let cwd_fonts = cwd.join("fonts");
        if cwd_fonts.exists() {
            return Some(cwd_fonts);
        }
    }

    None
}

/// 获取扩展系统字体目录（figlet 标准目录）
///
/// - Linux/macOS: `/usr/share/figlet`、`/usr/local/share/figlet`
/// - Windows: `%USERPROFILE%\fonts`
pub fn extended_font_dir() -> Option<PathBuf> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let candidates = [
            PathBuf::from("/usr/share/figlet"),
            PathBuf::from("/usr/local/share/figlet"),
        ];
        for p in &candidates {
            if p.exists() {
                return Some(p.clone());
            }
        }
        candidates.first().cloned()
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(home) = std::env::var("USERPROFILE") {
            let p = PathBuf::from(home).join("fonts");
            if p.exists() {
                return Some(p);
            }
        }
        None
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

/// 获取所有字体搜索路径（按优先级排序）
///
/// 返回顺序：
/// 1. 内置 ../fonts 目录
/// 2. 扩展系统目录
/// 3. 用户指定目录（通过 `-d` 参数传入）
pub fn font_search_paths(extra_dir: Option<&Path>) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();

    if let Some(builtin) = builtin_font_dir() {
        paths.push(builtin);
    }

    if let Some(ext) = extended_font_dir() {
        if !paths.contains(&ext) {
            paths.push(ext);
        }
    }

    if let Some(extra) = extra_dir {
        let extra = extra.to_path_buf();
        if !paths.contains(&extra) {
            paths.push(extra);
        }
    }

    paths
}

/// 获取系统 TTF/OTF 字体搜索目录
pub fn system_font_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();

    if let Some(builtin) = builtin_font_dir() {
        dirs.push(builtin);
    }

    #[cfg(target_os = "linux")]
    {
        dirs.push(PathBuf::from("/usr/share/fonts"));
        dirs.push(PathBuf::from("/usr/local/share/fonts"));
        if let Ok(home) = std::env::var("HOME") {
            dirs.push(PathBuf::from(&home).join(".fonts"));
            dirs.push(PathBuf::from(&home).join(".local").join("share").join("fonts"));
        }
    }

    #[cfg(target_os = "macos")]
    {
        dirs.push(PathBuf::from("/System/Library/Fonts"));
        dirs.push(PathBuf::from("/Library/Fonts"));
        if let Ok(home) = std::env::var("HOME") {
            dirs.push(PathBuf::from(&home).join("Library").join("Fonts"));
        }
    }

    #[cfg(target_os = "windows")]
    {
        let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());
        dirs.push(PathBuf::from(windir).join("Fonts"));
        if let Ok(home) = std::env::var("USERPROFILE") {
            dirs.push(PathBuf::from(home).join("fonts"));
        }
    }

    dirs
}

/// 在给定目录列表中按名称查找字体文件（.flf / .tlf / .ttf / .otf / .bdf）
///
/// 支持带或不带扩展名的字体名称。优先匹配精确文件名，其次匹配文件名主干。
pub fn find_font_file(name: &str, dirs: &[PathBuf]) -> Option<PathBuf> {
    let name_lower = name.to_lowercase();
    let has_flf_ext = name_lower.ends_with(".flf") || name_lower.ends_with(".tlf");
    let has_font_ext = name_lower.ends_with(".ttf")
        || name_lower.ends_with(".otf")
        || name_lower.ends_with(".bdf");

    for dir in dirs {
        if !dir.exists() {
            continue;
        }
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let fname = match path.file_name().and_then(|s| s.to_str()) {
                Some(n) => n,
                None => continue,
            };
            let fname_lower = fname.to_lowercase();

            if has_flf_ext || has_font_ext {
                // 精确文件名匹配
                if fname_lower == name_lower {
                    return Some(path);
                }
            } else {
                // 按主干名匹配
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");
                let ext_lower = ext.to_lowercase();
                let valid_ext = matches!(
                    ext_lower.as_str(),
                    "flf" | "tlf" | "ttf" | "otf" | "bdf"
                );
                if valid_ext {
                    let stem = fname_lower
                        .strip_suffix(&format!(".{}", ext_lower))
                        .unwrap_or(&fname_lower);
                    if stem == name_lower {
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}

/// 列出指定目录中所有 flf/tlf 字体文件名
pub fn list_flf_files(dirs: &[PathBuf]) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for dir in dirs {
        if !dir.exists() {
            continue;
        }
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext.eq_ignore_ascii_case("flf") || ext.eq_ignore_ascii_case("tlf") {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        files.push(name.to_string());
                    }
                }
            }
        }
    }
    files.sort();
    files
}

/// 递归列出系统字体目录中所有 TTF/OTF 字体文件名
pub fn list_system_ttf_files() -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for dir in &system_font_dirs() {
        collect_ttf_files(dir, &mut files, 3);
    }
    files.sort();
    files
}

fn collect_ttf_files(dir: &Path, files: &mut Vec<String>, max_depth: usize) {
    if max_depth == 0 || !dir.exists() {
        return;
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_ttf_files(&path, files, max_depth - 1);
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            if ext_lower == "ttf" || ext_lower == "otf" || ext_lower == "ttc" {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    files.push(name.to_string());
                }
            }
        }
    }
}

/// 获取用户主目录
pub fn home_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("USERPROFILE").ok().map(PathBuf::from)
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}
