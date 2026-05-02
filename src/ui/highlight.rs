//! JSON 语法高亮。
//!
//! 将原始 JSON 文本转换为 egui LayoutJob，为不同类型的 token
//! 应用不同的颜色。供 input 和 output 面板共用。
//!
//! 对于超大文本（超过 100 KB），自动跳过语法高亮以避免卡顿。

use egui::{FontId, Stroke, TextFormat};
use crate::app::theme::SyntaxColors;
use crate::core::link::UrlSpan;

/// 超过此字节数则跳过语法高亮，仅做纯文本渲染。
pub const MAX_HIGHLIGHT_BYTES: usize = 100_000;

/// 对 JSON 文本进行语法高亮，追加到 `job`。
///
/// 如果提供了 `url_spans`，其中的 URL 会使用链接专用颜色并添加下划线。
/// 如果 `text` 超过 `MAX_HIGHLIGHT_BYTES`，自动降级为纯文本渲染。
pub fn highlight_json(
    job: &mut egui::text::LayoutJob,
    text: &str,
    colors: &SyntaxColors,
    font_id: FontId,
    url_spans: &[UrlSpan],
) {
    if text.len() > MAX_HIGHLIGHT_BYTES {
        // 大文件：纯文本渲染，仅做 URL 高亮
        append_plain_with_urls(job, text, colors, &font_id, url_spans);
        return;
    }

    let bytes = text.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        let ch = bytes[pos] as char;

        match ch {
            '"' => {
                let start = pos;
                pos += 1;
                while pos < bytes.len() {
                    if bytes[pos] == b'\\' {
                        pos += 1;
                        if pos < bytes.len() {
                            pos += 1;
                        }
                    } else if bytes[pos] == b'"' {
                        pos += 1;
                        break;
                    } else {
                        pos += 1;
                    }
                }
                let end = pos;
                let is_key = bytes[end..]
                    .iter()
                    .skip_while(|b| b.is_ascii_whitespace())
                    .next()
                    == Some(&b':');

                if is_key {
                    job.append(
                        &text[start..end], 0.0,
                        TextFormat::simple(font_id.clone(), colors.key),
                    );
                } else {
                    append_string_with_urls(
                        job, &text[start..end], start, colors, &font_id, url_spans,
                    );
                }
            }

            '{' | '}' | '[' | ']' => {
                job.append(&text[pos..pos + 1], 0.0, TextFormat::simple(font_id.clone(), colors.bracket));
                pos += 1;
            }
            ':' => {
                job.append(&text[pos..pos + 1], 0.0, TextFormat::simple(font_id.clone(), colors.colon));
                pos += 1;
            }
            ',' => {
                job.append(&text[pos..pos + 1], 0.0, TextFormat::simple(font_id.clone(), colors.comma));
                pos += 1;
            }

            't' if text[pos..].starts_with("true") => {
                job.append("true", 0.0, TextFormat::simple(font_id.clone(), colors.boolean));
                pos += 4;
            }
            'f' if text[pos..].starts_with("false") => {
                job.append("false", 0.0, TextFormat::simple(font_id.clone(), colors.boolean));
                pos += 5;
            }
            'n' if text[pos..].starts_with("null") => {
                job.append("null", 0.0, TextFormat::simple(font_id.clone(), colors.null));
                pos += 4;
            }

            '-' | '0'..='9' => {
                let start = pos;
                pos += 1;
                let mut seen_dot = false;
                let mut seen_exp = false;
                while pos < bytes.len() {
                    let c = bytes[pos] as char;
                    match c {
                        '0'..='9' => { pos += 1; }
                        '.' if !seen_dot && !seen_exp => {
                            seen_dot = true;
                            pos += 1;
                            if pos < bytes.len() && (bytes[pos] as char).is_ascii_digit() {
                                pos += 1;
                            }
                        }
                        'e' | 'E' if !seen_exp => {
                            seen_exp = true;
                            pos += 1;
                            if pos < bytes.len() && (bytes[pos] == b'+' || bytes[pos] == b'-') {
                                pos += 1;
                            }
                        }
                        _ => break,
                    }
                }
                job.append(&text[start..pos], 0.0, TextFormat::simple(font_id.clone(), colors.number));
            }

            _ => {
                // 按 UTF-8 字符边界前进，避免多字节字符 panic
                let real_ch = text[pos..].chars().next().unwrap_or('\0');
                let ch_len = real_ch.len_utf8().max(1);
                job.append(
                    &text[pos..pos + ch_len],
                    0.0,
                    TextFormat::simple(font_id.clone(), colors.plain),
                );
                pos += ch_len;
            }
        }
    }
}

/// 对大文件进行纯文本渲染，仅高亮 URL 部分。
fn append_plain_with_urls(
    job: &mut egui::text::LayoutJob,
    text: &str,
    colors: &SyntaxColors,
    font_id: &FontId,
    url_spans: &[UrlSpan],
) {
    if url_spans.is_empty() {
        job.append(text, 0.0, TextFormat::simple(font_id.clone(), colors.plain));
        return;
    }

    let mut cursor = 0;
    for span in url_spans {
        let start = span.start.min(text.len());
        let end = span.end.min(text.len());
        if start < cursor || start >= end {
            continue;
        }

        if start > cursor {
            job.append(&text[cursor..start], 0.0, TextFormat::simple(font_id.clone(), colors.plain));
        }

        let mut fmt = TextFormat::simple(font_id.clone(), colors.url);
        fmt.underline = Stroke::new(1.0, colors.url);
        job.append(&text[start..end], 0.0, fmt);

        cursor = end;
    }
    if cursor < text.len() {
        job.append(&text[cursor..], 0.0, TextFormat::simple(font_id.clone(), colors.plain));
    }
}

/// 追加一个 JSON 字符串 token，对其中的 URL 部分使用链接样式。
fn append_string_with_urls(
    job: &mut egui::text::LayoutJob,
    slice: &str,
    slice_offset: usize,
    colors: &SyntaxColors,
    font_id: &FontId,
    url_spans: &[UrlSpan],
) {
    let slice_end = slice_offset + slice.len();
    let relevant_urls: Vec<&UrlSpan> = url_spans
        .iter()
        .filter(|span| span.start < slice_end && span.end > slice_offset)
        .collect();

    if relevant_urls.is_empty() {
        job.append(slice, 0.0, TextFormat::simple(font_id.clone(), colors.string));
        return;
    }

    let mut cursor = 0;
    for url_span in &relevant_urls {
        let url_start = url_span.start.saturating_sub(slice_offset).min(slice.len());
        let url_end = url_span.end.saturating_sub(slice_offset).min(slice.len());

        if url_start < cursor || url_start >= url_end {
            continue;
        }

        if url_start > cursor {
            job.append(
                &slice[cursor..url_start], 0.0,
                TextFormat::simple(font_id.clone(), colors.string),
            );
        }

        let mut fmt = TextFormat::simple(font_id.clone(), colors.url);
        fmt.underline = Stroke::new(1.0, colors.url);
        job.append(&slice[url_start..url_end], 0.0, fmt);

        cursor = url_end;
    }

    if cursor < slice.len() {
        job.append(
            &slice[cursor..], 0.0,
            TextFormat::simple(font_id.clone(), colors.string),
        );
    }
}
