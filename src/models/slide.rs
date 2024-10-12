use std::time::Duration;

/// 画像スライドの命令書を表す構造体
#[derive(Debug, Clone)]
pub struct SlideImage {
    pub id: i32,
    /// 画像表示の開始時間
    pub start_time: Duration,
    /// 画像表示の終了時間
    pub end_time: Duration,
    /// 画像のファイルパス（またはURL）
    pub image_path: String,
    /// 画像の表示位置やサイズのオプション（オプション）
    pub display_options: Option<ImageDisplayOptions>,
}

/// 画像の表示位置やサイズに関する設定
#[derive(Debug, Clone)]
pub struct ImageDisplayOptions {
    /// 表示するX座標（左上を基準）
    pub x_pos: u32,
    /// 表示するY座標（左上を基準）
    pub y_pos: u32,
    /// 画像の幅
    pub width: u32,
    /// 画像の高さ
    pub height: u32,
}
