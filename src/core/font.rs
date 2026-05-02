//! 字体加载。
//!
//! 由于 egui 默认字体不含 CJK 字形，中文会显示为乱码/方框。
//! 本模块按平台搜索系统自带的中文字体并注册到 egui。

/// 加载支持中文的系统字体数据。
pub fn load_cjk_font() -> Option<Vec<u8>> {
    let paths = cjk_font_paths();
    for path in &paths {
        if let Ok(data) = std::fs::read(path) {
            log::info!("已加载 CJK 字体: {path}");
            return Some(data);
        }
    }
    log::warn!("未找到系统 CJK 字体，中文将无法正常显示");
    None
}

/// 按平台返回候选中文字体路径列表。
fn cjk_font_paths() -> Vec<String> {
    let mut paths = Vec::new();

    #[cfg(target_os = "windows")]
    {
        let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".into());
        paths.push(format!("{windir}\\Fonts\\msyh.ttc"));   // 微软雅黑
        paths.push(format!("{windir}\\Fonts\\msyhbd.ttc"));  // 微软雅黑 Bold
        paths.push(format!("{windir}\\Fonts\\simhei.ttf"));  // 黑体
        paths.push(format!("{windir}\\Fonts\\simsun.ttc"));  // 宋体
    }

    #[cfg(target_os = "macos")]
    {
        paths.push("/System/Library/Fonts/PingFang.ttc".into());
        paths.push("/System/Library/Fonts/STHeiti Light.ttc".into());
        paths.push("/System/Library/Fonts/Hiragino Sans GB.ttc".into());
    }

    #[cfg(target_os = "linux")]
    {
        paths.push("/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc".into());
        paths.push("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc".into());
        paths.push("/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc".into());
        paths.push("/usr/share/fonts/wqy-zenhei/wqy-zenhei.ttc".into());
        paths.push("/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc".into());
        paths.push("/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf".into());
    }

    paths
}
