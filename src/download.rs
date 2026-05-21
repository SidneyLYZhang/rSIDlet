use std::io;
use std::path::{Path, PathBuf};

/// GitHub 字体仓库配置
const FONT_REPOS: &[FontRepo] = &[
    FontRepo {
        owner: "xero",
        repo: "figlet-fonts",
        branch: "master",
        subdir: "",
    },
    FontRepo {
        owner: "PhMajerus",
        repo: "FIGfonts",
        branch: "main",
        subdir: "fonts",
    },
];

struct FontRepo {
    owner: &'static str,
    repo: &'static str,
    branch: &'static str,
    subdir: &'static str,
}

/// 下载结果
#[derive(Debug)]
pub struct DownloadResult {
    pub file_path: PathBuf,
    pub source_url: String,
}

/// 从 GitHub 仓库下载字体文件到目标目录
///
/// 依次尝试两个 GitHub 仓库，找到后下载到 `dest_dir`。
pub fn download_font(name: &str, dest_dir: &Path) -> io::Result<DownloadResult> {
    // 确保字体名有扩展名
    let candidates = resolve_candidates(name);

    for repo in FONT_REPOS {
        for candidate in &candidates {
            let url = format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                repo.owner, repo.repo, repo.branch,
                if repo.subdir.is_empty() {
                    candidate.to_string()
                } else {
                    format!("{}/{}", repo.subdir, candidate)
                }
            );

            match try_download(&url, candidate, dest_dir) {
                Ok(result) => return Ok(result),
                Err(_) => continue,
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("无法从任何 GitHub 仓库下载字体: {}", name),
    ))
}

/// 下载字体文件的原始数据
pub fn download_font_data(name: &str) -> io::Result<(Vec<u8>, String)> {
    let candidates = resolve_candidates(name);

    for repo in FONT_REPOS {
        for candidate in &candidates {
            let url = format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                repo.owner, repo.repo, repo.branch,
                if repo.subdir.is_empty() {
                    candidate.to_string()
                } else {
                    format!("{}/{}", repo.subdir, candidate)
                }
            );

            match try_download_data(&url) {
                Ok(data) => return Ok((data, candidate.clone())),
                Err(_) => continue,
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("无法从任何 GitHub 仓库下载字体: {}", name),
    ))
}

/// 获取两个 GitHub 仓库中可用的字体文件列表
pub fn list_available_online() -> io::Result<Vec<String>> {
    let mut all_files: Vec<String> = Vec::new();

    for repo in FONT_REPOS {
        let api_url = if repo.subdir.is_empty() {
            format!(
                "https://api.github.com/repos/{}/{}/contents/",
                repo.owner, repo.repo
            )
        } else {
            format!(
                "https://api.github.com/repos/{}/{}/contents/{}",
                repo.owner, repo.repo, repo.subdir
            )
        };

        if let Ok(files) = fetch_repo_file_list(&api_url) {
            for f in files {
                let lower = f.to_lowercase();
                if lower.ends_with(".flf") || lower.ends_with(".tlf") {
                    if !all_files.contains(&f) {
                        all_files.push(f);
                    }
                }
            }
        }
    }

    if all_files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "无法获取在线字体列表（GitHub API 可能限流）",
        ));
    }

    all_files.sort();
    Ok(all_files)
}

/// 尝试从 URL 下载并保存到目标目录
fn try_download(url: &str, filename: &str, dest_dir: &Path) -> io::Result<DownloadResult> {
    let data = try_download_data(url)?;

    std::fs::create_dir_all(dest_dir)?;
    let file_path = dest_dir.join(filename);
    std::fs::write(&file_path, &data)?;

    Ok(DownloadResult {
        file_path,
        source_url: url.to_string(),
    })
}

/// 尝试从 URL 下载数据
fn try_download_data(url: &str) -> io::Result<Vec<u8>> {
    let resp = ureq::get(url)
        .set("User-Agent", "rsidlet/0.1.0")
        .call()
        .map_err(|e| {
            io::Error::new(io::ErrorKind::ConnectionRefused, format!("下载失败: {}", e))
        })?;

    let mut data = Vec::new();
    resp.into_reader().read_to_end(&mut data)?;
    Ok(data)
}

/// 获取 GitHub 仓库目录中的文件列表
fn fetch_repo_file_list(api_url: &str) -> io::Result<Vec<String>> {
    let resp = ureq::get(api_url)
        .set("User-Agent", "rsidlet/0.1.0")
        .set("Accept", "application/vnd.github.v3+json")
        .call()
        .map_err(|e| {
            io::Error::new(io::ErrorKind::ConnectionRefused, format!("API 请求失败: {}", e))
        })?;

    let body = resp.into_string().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("读取响应失败: {}", e))
    })?;

    // GitHub API 可能返回 prettified 或 minified JSON
    // 需要在整个 body 中搜索所有 "name" 字段，而非逐行查找第一个
    let mut names = Vec::new();
    let mut pos = 0usize;
    while pos < body.len() {
        let haystack = &body[pos..];
        let start = match haystack.find("\"name\"") {
            Some(s) => s,
            None => break,
        };
        let abs = pos + start;
        // 找到 ": " 后面的引号
        let after_key = match body[abs..].find(':') {
            Some(c) => abs + c + 1,
            None => {
                pos = abs + 6;
                continue;
            }
        };
        let val = &body[after_key..];
        let q1 = match val.find('"') {
            Some(q) => q,
            None => {
                pos = abs + 6;
                continue;
            }
        };
        let q2 = match val[q1 + 1..].find('"') {
            Some(q) => q,
            None => {
                pos = abs + 6;
                continue;
            }
        };
        let name = &val[q1 + 1..q1 + 1 + q2];
        names.push(name.to_string());
        pos = after_key + q1 + 1 + q2 + 1;
    }

    Ok(names)
}

/// 根据字体名生成候选文件名列表
fn resolve_candidates(name: &str) -> Vec<String> {
    let name_lower = name.to_lowercase();

    // 如果已经带扩展名，直接使用
    if name_lower.ends_with(".flf") || name_lower.ends_with(".tlf") {
        return vec![name.to_string(), name_lower];
    }

    // 尝试添加常见扩展名
    vec![
        format!("{}.flf", name),
        format!("{}.tlf", name),
        format!("{}.FLF", name),
        format!("{}.TLF", name),
    ]
}
