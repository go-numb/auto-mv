use std::{fs::File, process::Command, time::Duration};

use std::io::{self, Read};

// local binary:: speech.exe
/// テキストを音声に変換する
// -text string:    text line for say something (default "Hello World")
// -output string:  uotput path & filename (default "./speech-voice.wav")
// -lang string:    language code (default "ja-JP")
// -voice string:   voice name (default "jp-JP-Standard-A")
pub fn command(
    text: &str,
    output: &str,
    lang: Option<String>,
    voice: Option<String>,
) -> Result<Duration, io::Error> {
    let lang = if let Some(l) = lang {
        l
    } else {
        "ja-JP".to_string()
    };
    let voice = if let Some(v) = voice {
        v
    } else {
        "jp-JP-Standard-A".to_string()
    };

    let status = Command::new("speech")
        .args([
            "-text", text, "-output", output, "-lang", &lang, "-voice", &voice,
        ])
        .status()?;

    if !status.success() {
        eprintln!(
            "Error: Failed to generate blank video - status: {:?}",
            status
        );
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to generate blank video",
        ));
    }

    // 音声ファイルの再生時間を取得
    let duration = get_wav_duration(output)?;

    Ok(duration)
}
fn get_wav_duration(file_path: &str) -> Result<Duration, io::Error> {
    let mut file = File::open(file_path)?;

    let mut header = [0u8; 44]; // WAVヘッダのサイズは44バイト。
    file.read_exact(&mut header)?;

    // サンプリングレートを取得
    let sample_rate = u32::from_le_bytes(header[24..28].try_into().unwrap());
    // チャンネル数を取得
    let num_channels = u16::from_le_bytes(header[22..24].try_into().unwrap());
    // データサイズを取得
    let data_size = u32::from_le_bytes(header[40..44].try_into().unwrap());

    // 再生時間（秒）を計算
    let f = data_size as f32 / (sample_rate as f32 * num_channels as f32 * 2.0); // 16ビット=2バイト

    let duration = Duration::from_secs_f32(f);

    Ok(duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let text = "おはよう世界！";
        let output = "./source/voice.wav";

        match command(text, output, None, None) {
            Ok(duration) => {
                println!("Success! {:?}", duration);
            }
            Err(e) => panic!("Error: {:?}", e),
        }
    }
}
