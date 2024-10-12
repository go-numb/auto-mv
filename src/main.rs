use dotenv::dotenv;
use log::{error, info};
use std::{env, fs, io, ops::Add, time::Duration};

use crate::models::{
    ass_subtitle::{create_ass_file, StyleType, Subtitle},
    slide::SlideImage,
};

mod ffmpeg;

mod speech;

mod models;

/// 半自動動画作成手順
/// 0. 脚本べースで動画を作成する際の手順を自動化
/// 1. ffmpeg::command::brank で空白の動画
/// 2. 字幕命令書と画像挿入命令書を生成
/// 3. speech::command::text-to-speech でセリフまたは行ごと音声を生成
/// 4. ffmpeg::command::add_audioで動画に音声を追加
/// 5. 音声の長さと待機時間を[start, end]としタイムスタンプを生成
/// 6. 字幕命令書及び画像挿入命令書にテキスト・画像・タイムスタンプを追加
/// 7. ffmpeg::command::add_subs で字幕を動画に追加
/// 8. ffmpeg::command::add_image_overlay で画像を動画にオーバーレイ
/// 9. ファイナライズ
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // 累積時間の初期化
    let mut total_time = Duration::from_secs_f64(0.0);
    // 字幕付与命令書の生成
    let mut asss = Vec::new();
    // 画像付与命令書の生成
    let mut slides: Vec<SlideImage> = Vec::new();

    let scripts_file = "./source/scripts.txt";
    let voice_output = "./source/voice.wav";
    let subtitle_output = "./source/subtitle.ass";

    // 環境変数から待機時間を取得
    let waiting_sec_after_speaking = {
        let key = env::var("WAITING_SEC_AFTER_SPEAKING")
            .unwrap_or_else(|_| "1".to_string())
            .parse::<u64>()
            .unwrap();
        Duration::from_secs(key)
    };

    // 脚本を定義
    // セリフ、または行ごとに配列で定義
    // 1配列ごとに音声を生成する
    // why: 音声の長さに字幕生成・動画が依存しているため
    let scripts = if fs::exists(scripts_file).unwrap() {
        let script_content = fs::read_to_string(scripts_file).unwrap();
        // file to string
        script_content
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    } else {
        vec![
            String::from("脚本を元に音声を生成"),
            String::from("脚本のセリフまたは行ごとに"),
            String::from("音声を生成"),
            String::from("字幕命令書に追加"),
            String::from("画像付与命令書に追加"),
            String::from("累積時間を更新"),
            String::from("字幕を動画に追加"),
        ]
    };

    // ffmpeg::command::brank で空白の動画を生成
    let (output, resolution, duration, frame_rate) = ("./source/0-brank.mp4", "1280x720", 60, 30);
    ffmpeg::command::brank(output, resolution, duration, frame_rate)?;

    // 脚本を元に音声を生成
    // 脚本のセリフまたは行ごとに
    // 音声を生成
    // 字幕命令書に追加
    // 画像付与命令書に追加
    // 累積時間を更新
    // 字幕を動画に追加
    let update_output = "./source/1-audio-overlay.mp4";
    for (i, script) in scripts.iter().enumerate() {
        // ffmpeg::command::add_audio でテキストから音声を生成
        let code = speech::voice::Code::new();
        let voice = speech::voice::Name::from(code, i as u32).unwrap();
        info!("voice: {}", voice);
        let duration = match speech::local::command(
            script,
            voice_output,
            Some(code.to_string()),
            Some(voice.to_string()),
        ) {
            Ok(duration) => {
                // 音声の長さを取得
                info!("voice time: {:?}", duration);
                asss.push(Subtitle {
                    id: i as i32,
                    start_time: total_time,
                    end_time: total_time + duration,
                    text: script.to_string(),
                    // [TODO] 適宜、スタイルを追加
                    style_name: Some(StyleType::from(i as u32)),
                });

                // [TODO] 適宜、画像付与命令書を追加
                // 画像出力に伴うロジックが必要
                // if i == 0 {
                //     ffmpeg::command::add_image_overlay(
                //         output,
                //         "./source/layer_image.jpg",
                //         0,
                //         20,
                //         output,
                //     )?;
                //     // slides.push(SlideImage {
                //     //     id: i as i32,
                //     //     start_time: Duration::from_secs(0),
                //     //     end_time: Duration::from_secs(20),
                //     //     image_path: "./source/layer_image.jpg".to_string(),
                //     //     display_options: None,
                //     // });
                // }

                duration
            }
            Err(e) => {
                error!("Error: {}", e);
                continue;
            }
        };

        // ffmpeg::command::add_audio で音声を動画に追加
        // 動画ファイルを更新・追記していく
        let (input, audio, update_output) = if i == 0 {
            (output, voice_output, update_output)
        } else {
            (update_output, voice_output, update_output)
        };
        let volume_waight = i + 1;
        ffmpeg::command::add_audio(
            input,
            audio,
            update_output,
            total_time,
            volume_waight as i32,
        )?;

        // 累積時間を更新
        total_time += duration + waiting_sec_after_speaking;

        info!(
            "[{}: {}] 音声ファイルを動画に追加しました。",
            i,
            script // format_duration_as_ass_time(total_time)
        );
    }

    // 音声を追加した動画を更新
    let output = update_output;

    // 音声が追加された動画が完成
    // 画像付与命令書に従い画像を追加

    // [TODO] 画像付与命令書を元に動画に画像を追加
    if !slides.is_empty() {
        for (i, script) in slides.iter().enumerate() {
            // ffmpeg::command::add_image_overlay で画像を動画にオーバーレイ
            let (start, end) = (0, 5);
            let (input, layer_image, output) = (
                update_output,
                "./source/layer_image.jpg",
                "./source/2-image-overlay.mp4",
            );
            ffmpeg::command::add_image_overlay(input, layer_image, start, end, output)?;
        }
    };

    // 音声と画像が追加された動画が完成
    info!("音声とスライドが追加された動画が完成しました。");

    // ass: 字幕命令書を元にass形式の字幕を生成
    if !asss.is_empty() {
        let str_for_ass_file = create_ass_file(&asss);

        // ass形式の字幕ファイルを生成
        std::fs::write(subtitle_output, str_for_ass_file)?;

        info!("字幕ファイルを生成しました。");
    }

    // ffmpeg::command::add_subs で字幕を動画に追加
    let (input, subs_file, output) = (output, subtitle_output, "./source/3-subtitle-overlay.mp4");
    ffmpeg::command::add_subs(input, subs_file, output)?;

    // リザルトファイルを用意する
    let (input, cut_to, output) = (
        output,
        total_time.add(Duration::new(2, 0)),
        "./source/result.mp4",
    );
    ffmpeg::command::cut(input, output, cut_to).unwrap();

    Ok(())
}
