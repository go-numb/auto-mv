use std::time::Duration;

use std::fmt;

pub struct Subtitle {
    pub id: i32,
    pub start_time: Duration,
    pub end_time: Duration,
    pub text: String,
    pub style: Option<Style>,
}

/// `Duration` を SRT形式の時間フォーマットに変換する補助関数
pub fn format_duration_as_srt_time(duration: Duration) -> String {
    // 秒数をミリ秒、分、時間に変換してフォーマット
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let millis = duration.subsec_millis();

    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, millis)
}

impl fmt::Display for Subtitle {
    /// `Subtitle` 構造体を SRT形式の文字列に変換する
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{}\n{} --> {}\n{}\n\n",
            self.id,
            format_duration_as_srt_time(self.start_time),
            format_duration_as_srt_time(self.end_time),
            self.text,
        )
    }
}

pub struct Style {
    pub font_family: String,
    pub font_size: u32,
    pub font_color: Color,
    pub font_alpha: u8,

    pub border_style: u32,
    pub border_color: Color,
    pub border_alpha: u8,

    pub outline_style: u32,
    pub shadow_style: u32,

    pub background_color: Color,
    pub background_alpha: u8,

    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

impl Style {
    /// Generate the style string for passing to ffmpeg's force_style option
    pub fn to_style(&self) -> String {
        format!(
            "Fontname={},Fontsize={},PrimaryColour={},BorderStyle={},BorderColour={},Outline={},Shadow={},BackColour={},MarginL={},MarginV={}",
            self.font_family,
            self.font_size,
            self.font_color.to_string(self.font_alpha),
            self.border_style,
            self.border_color.to_string(self.border_alpha),
            self.outline_style,
            self.shadow_style,
            self.background_color.to_string(self.background_alpha),
            // Use x1 for MarginL and y1 for MarginV (these can vary based on your use case)
            self.x1,
            self.y1
        )
    }
}

pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Cyan,
    Magenta,
    White,
    Brown,
    Gray,

    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hex_code = match self {
            Color::Red => "ef9a9a",
            Color::Green => "a5d6a7",
            Color::Blue => "90caf9",
            Color::Yellow => "fff59d",
            Color::Orange => "ffcc80",
            Color::Cyan => "80deea",
            Color::Magenta => "f48fb1",
            Color::White => "ffffff",
            Color::Brown => "bcaaa4",
            Color::Gray => "eeeeee",
            Color::Black => "000000",
        };
        write!(f, "{}", hex_code)
    }
}

impl Color {
    pub fn to_string(&self, alpha: u8) -> String {
        // # replace, &H+alpha+HexCode
        let alpha_hex = format!("{:02x}", 255 - alpha); // fliped alpha
        format!("&H{}{}", alpha_hex, self)
    }
}

#[cfg(test)]
// test color to string
mod tests {
    use super::*;

    #[test]
    fn test_color_to_string() {
        assert_eq!(Color::Red.to_string(00), "&Hffef9a9a");
        assert_eq!(Color::Green.to_string(00), "&Hffa5d6a7");
        assert_eq!(Color::Blue.to_string(00), "&Hff90caf9");
        assert_eq!(Color::Yellow.to_string(00), "&Hfffff59d");
        assert_eq!(Color::Orange.to_string(00), "&Hffffcc80");
        assert_eq!(Color::Cyan.to_string(255), "&H0080deea");
        assert_eq!(Color::Magenta.to_string(255), "&H00f48fb1");
        assert_eq!(Color::White.to_string(255), "&H00ffffff");
        assert_eq!(Color::Brown.to_string(255), "&H00bcaaa4");
        assert_eq!(Color::Gray.to_string(255), "&H00eeeeee");
        assert_eq!(Color::Black.to_string(255), "&H00000000");
    }

    #[test]
    fn test_to_style() {
        let style = Style {
            font_family: "Arial".to_string(),
            font_size: 24,
            font_color: Color::White,
            font_alpha: 255,
            border_style: 1,
            border_color: Color::Black,
            border_alpha: 255,
            outline_style: 1,
            shadow_style: 1,
            background_color: Color::Black,
            background_alpha: 0,
            x1: 10,
            y1: 10,
            x2: 10,
            y2: 10,
        };

        assert_eq!(
            style.to_style(),
            "Fontname=Arial,Fontsize=24,PrimaryColour=&H00ffffff,BorderStyle=1,BorderColour=&H00000000,Outline=1,Shadow=1,BackColour=&Hff000000,MarginL=10,MarginV=10"
        );
    }
}
