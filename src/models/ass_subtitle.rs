use std::time::Duration;

use std::fmt;

pub struct Subtitle {
    pub id: i32,
    pub start_time: Duration,
    pub end_time: Duration,
    pub text: String,
    pub style_name: Option<StyleType>,
}

/// `Duration` を SRT形式の時間フォーマットに変換する補助関数
pub fn format_duration_as_time(duration: Duration) -> String {
    // 秒数をミリ秒、分、時間に変換してフォーマット
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let millis = duration.subsec_millis() / 10;

    // 3桁のミリ秒を2桁に変換
    // why:: 3桁ミリ秒では重複が見られる
    format!("{:1}:{:02}:{:02}.{:02}", hours, minutes, seconds, millis)
}

impl fmt::Display for Subtitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Dialogue:0,{},{},{},,0,0,0,,{}",
            format_duration_as_time(self.start_time),
            format_duration_as_time(self.end_time),
            if let Some(t) = self.style_name {
                t.to_string()
            } else {
                StyleType::Default.to_string()
            }, // スタイル名を使用
            escape_ass_text(&self.text)
        )
    }
}

// ASS/SSA形式のテキストエスケープ
fn escape_ass_text(text: &str) -> String {
    // 句点と読点など改行候補になる文字で改行を挿入
    let mut text_with_line_breaks = text.replace("。", "。\n");

    // 改行タグが連続で生じないよう、末尾や不要な箇所の除去（末尾の除去）
    if text_with_line_breaks.ends_with(r"\n") {
        text_with_line_breaks.pop(); // 最後の {\N} を削除する
        text_with_line_breaks.pop();
        text_with_line_breaks.pop();
        text_with_line_breaks.pop();
    }
    text_with_line_breaks
}

pub struct Style {
    pub name: StyleType,
    pub font_family: String,
    pub font_size: u32,
    pub primary_color: Color,
    pub outline_color: Color,
    pub back_color: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike_out: bool,
    pub scale_x: u32,
    pub scale_y: u32,
    pub spacing: u32,
    pub angle: u32,
    pub border_style: u32,
    pub outline: u32,
    pub shadow: bool,
    pub alignment: u32,
    pub margin_l: u32,
    pub margin_r: u32,
    pub margin_v: u32,
    pub encoding: u32,
}

impl Style {
    pub fn default() -> Self {
        Style {
            name: StyleType::Default,
            font_family: "Arial".to_string(),
            font_size: 48,
            primary_color: Color::White,
            outline_color: Color::Black,
            back_color: Color::Black,
            bold: false,
            italic: false,
            underline: false,
            strike_out: false,
            scale_x: 100,
            scale_y: 100,
            spacing: 0,
            angle: 0,
            border_style: 1,
            outline: 2,
            shadow: true,
            alignment: 2,
            margin_l: 10,
            margin_r: 10,
            margin_v: 10,
            encoding: 1,
        }
    }

    pub fn from(n: u32) -> Self {
        Style {
            name: StyleType::from(n),
            font_family: "Arial".to_string(),
            font_size: 48,
            primary_color: Color::White,
            outline_color: Color::Black,
            back_color: Color::Black,
            bold: false,
            italic: false,
            underline: false,
            strike_out: false,
            scale_x: 100,
            scale_y: 100,
            spacing: 0,
            angle: 0,
            border_style: 1,
            outline: 2,
            shadow: true,
            alignment: 2,
            margin_l: 20,
            margin_r: 20,
            margin_v: 20,
            encoding: 1,
        }
    }

    pub fn all() -> Vec<Self> {
        (0..=9).map(Style::from).collect()
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (name, border_color) = match self.name {
            StyleType::Default => ("Default", Color::Black),
            StyleType::Red => ("Red", Color::Red),
            StyleType::Green => ("Green", Color::Green),
            StyleType::Blue => ("Blue", Color::Blue),
            StyleType::Yellow => ("Yellow", Color::Yellow),
            StyleType::Orange => ("Orange", Color::Orange),
            StyleType::Cyan => ("Cyan", Color::Cyan),
            StyleType::Magenta => ("Magenta", Color::Magenta),
            StyleType::Brown => ("Brown", Color::Brown),
            StyleType::Gray => ("Gray", Color::Gray),
        };

        write!(
            f,
            "Style:{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            name,
            self.font_family,
            self.font_size,
            &self.primary_color.to_string(255), // 80% alpha (255 * 0.2 = 0)
            &Color::Black.to_string(220),       // 完全不透明 (通常は使用しない)
            border_color.to_string(220),        // 80% alpha
            &self.back_color.to_string(220),    // 完全不透明
            if self.bold { 1 } else { 0 },
            if self.italic { 1 } else { 0 },
            if self.underline { 1 } else { 0 },
            if self.strike_out { 1 } else { 0 },
            self.scale_x,
            self.scale_y,
            self.spacing,
            self.angle,
            self.border_style,
            self.outline,
            if self.shadow { 1 } else { 0 },
            self.alignment,
            self.margin_l,
            self.margin_r,
            self.margin_v,
            self.encoding
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StyleType {
    Default,
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Cyan,
    Magenta,
    Brown,
    Gray,
}

impl StyleType {
    pub fn new() -> Self {
        StyleType::Default
    }

    // 引数で分岐
    pub fn from(n: u32) -> Self {
        match n {
            1 => StyleType::Red,
            2 => StyleType::Green,
            3 => StyleType::Blue,
            4 => StyleType::Yellow,
            5 => StyleType::Orange,
            6 => StyleType::Cyan,
            7 => StyleType::Magenta,
            8 => StyleType::Brown,
            9 => StyleType::Gray,
            _ => StyleType::Default,
        }
    }
}

impl fmt::Display for StyleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            StyleType::Default => "Default",
            StyleType::Red => "Red",
            StyleType::Green => "Green",
            StyleType::Blue => "Blue",
            StyleType::Yellow => "Yellow",
            StyleType::Orange => "Orange",
            StyleType::Cyan => "Cyan",
            StyleType::Magenta => "Magenta",
            StyleType::Brown => "Brown",
            StyleType::Gray => "Gray",
        };
        write!(f, "{}", name)
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

pub fn create_ass_file(subtitles: &[Subtitle]) -> String {
    // all styles
    let styles = Style::all();

    let mut content = String::new();

    // スクリプト情報（変更なし）
    content.push_str(
        "[Script Info]
; Script generated by Aegisub 3.2.2
; http://www.aegisub.org/
Title: Default Aegisub file
ScriptType: v4.00+
WrapStyle: 0
Collisions: Normal
PlayResX: 1280
PlayResY: 720
Timer: 100.0000
ScaledBorderAndShadow: yes
YCbCr Matrix: None
\n",
    );
    // ...

    // スタイル情報
    content.push_str("[V4+ Styles]\n");
    content.push_str("Format:Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,StrikeOut,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding\n");
    for style in styles {
        content.push_str(&style.to_string());
        content.push('\n');
    }
    content.push_str("\n\n");

    // イベント（字幕）情報
    content.push_str("[Events]\n");
    content.push_str("Format:Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text\n");
    for subtitle in subtitles {
        content.push_str(&subtitle.to_string());
        content.push('\n');
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_ass_text() {
        let text = "これはテストです。もう一つの文章です。";
        let modified_text = escape_ass_text(text);
        println!("{}", modified_text);
    }
}
