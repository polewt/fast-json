//! URL / 文件路径检测与打开。

/// 提取文本中的 URL。
pub fn extract_urls(text: &str) -> Vec<UrlSpan> {
    let mut results = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if let Some(len) = starts_with_scheme(&bytes[i..]) {
            let end = i + len;
            results.push(UrlSpan {
                url: text[i..end].to_string(),
                start: i,
                end,
            });
            i = end;
        } else {
            i += 1;
        }
    }
    results
}

fn starts_with_scheme(bytes: &[u8]) -> Option<usize> {
    for prefix in [b"http://" as &[u8], b"https://", b"ftp://", b"file://"] {
        if bytes.starts_with(prefix) {
            let rest = &bytes[prefix.len()..];
            return Some(prefix.len() + consume_url_chars(rest));
        }
    }
    None
}

fn consume_url_chars(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .take_while(|&&b| b > 32 && b < 127 && !b"\"'<>{}|\\^`".contains(&b))
        .count()
}

/// URL 在文本中的位置。
#[derive(Debug, Clone)]
pub struct UrlSpan {
    pub url: String,
    pub start: usize,
    pub end: usize,
}

/// 使用系统默认程序打开 URL。
#[cfg(feature = "platform-link")]
pub fn open_url(url: &str) {
    let _ = open::that(url);
}

#[cfg(not(feature = "platform-link"))]
pub fn open_url(url: &str) {
    log::warn!("link opening disabled (enable platform-link feature): {url}");
}

/// 判断字符串是否看起来像一个 URL。
pub fn is_url(text: &str) -> bool {
    text.starts_with("http://")
        || text.starts_with("https://")
        || text.starts_with("ftp://")
        || text.starts_with("file://")
}
