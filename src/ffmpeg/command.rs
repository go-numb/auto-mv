use std::{env, fs, io, process::Command, time::Duration};

pub fn brank(
    output_path: &str,
    resolution: &str,
    duration: u32,
    frame_rate: u32,
) -> Result<(), io::Error> {
    println!("env: {}", env::var("OVERWRITE").unwrap());
    let is_overwrite = match env::var("OVERWRITE") {
        Ok(val) => val == "true",
        Err(_) => false,
    };
    let is_nvidia = match env::var("NVIDIA") {
        Ok(val) => val == "true",
        Err(_) => false,
    };

    let status = Command::new("ffmpeg")
        .args([
            if is_overwrite { "-y" } else { "-n" },
            "-f",
            "lavfi",
            "-i",
            &format!("color=c=white:s={}", resolution),
            if is_nvidia { "-c:v" } else { "" },
            if is_nvidia { "h264_nvenc" } else { "" },
            "-b:v",
            "5M", // ビットレートを 5Mbps に設定
            "-t",
            &duration.to_string(),
            "-r",
            &frame_rate.to_string(),
            output_path,
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

    Ok(())
}

pub fn add_image_overlay(
    input_video: &str,
    image_path: &str,
    start_time: u32,
    end_time: u32,
    output_video: &str,
) -> Result<(), io::Error> {
    let is_overwrite = env::var("OVERWRITE").unwrap() == "true";
    let is_nvidia = env::var("NVIDIA").unwrap() == "true";

    // 一時ファイル名を生成
    let temp_output_video = format!("{}_tmp.mp4", output_video);

    let status = Command::new("ffmpeg")
        .args([
            if is_overwrite { "-y" } else { "-n" },
            "-i",
            input_video,
            "-i",
            image_path,
            "-filter_complex",
            &format!(
                "[0][1]overlay=x=0:y=0:enable='between(t,{},{})'",
                start_time, end_time
            ),
            if is_nvidia { "-c:v" } else { "" },
            if is_nvidia { "h264_nvenc" } else { "" },
            "-c:a",
            "copy", // 音声はそのままコピー
            "-b:v",
            "5M", // ビットレートを 5Mbps に設定
            &temp_output_video,
        ])
        .status()?;

    if !status.success() {
        eprintln!("Error: Failed to add image overlay - status: {:?}", status);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to add image overlay",
        ));
    }

    // 一時ファイルをリネーム
    fs::rename(&temp_output_video, output_video)?;

    Ok(())
}

pub fn add_subs(
    input_video: &str,
    subtitles_file: &str,
    output_video: &str,
) -> Result<(), io::Error> {
    let is_overwrite = env::var("OVERWRITE").unwrap() == "true";
    let is_nvidia = env::var("NVIDIA").unwrap() == "true";

    let status = Command::new("ffmpeg")
        .args([
            if is_overwrite { "-y" } else { "-n" },
            "-i",
            input_video,
            "-vf",
            &format!("subtitles=filename={}", subtitles_file),
            if is_nvidia { "-c:v" } else { "" },
            if is_nvidia { "h264_nvenc" } else { "" },
            "-c:a",
            "copy", // 音声はそのままコピー
            "-b:v",
            "5M", // ビットレートを 5Mbps に設定
            // "-v",
            // "debug",
            output_video,
        ])
        .status()?;

    if !status.success() {
        eprintln!("Error: Failed to add subtitles - status: {:?}", status);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to add subtitles",
        ));
    }

    Ok(())
}

pub fn add_audio(
    input_video: &str,
    audio_file: &str,
    output_video: &str,
    start: Duration,
    weight: i32,
) -> Result<(), io::Error> {
    let is_overwrite = env::var("OVERWRITE").unwrap() == "true";
    let is_nvidia = env::var("NVIDIA").unwrap() == "true";

    // startのチェック
    if start.as_secs() == 0 {
        // startが0秒の場合は、直接音声を追加する（itsoffsetは不要）
        let status = Command::new("ffmpeg")
            .args([
                if is_overwrite { "-y" } else { "-n" },
                "-i",
                input_video,
                "-i",
                audio_file,
                "-c:v",
                if is_nvidia { "h264_nvenc" } else { "copy" }, // 映像エンコーディング
                "-c:a",
                "aac", // 音声をAACでエンコード
                "-b:v",
                "5M", // ビットレートを5Mbpsに設定
                output_video,
            ])
            .status()?;

        // エラーチェック
        if !status.success() {
            eprintln!(
                "Error: Failed to add audio (start = 0s) - status: {:?}",
                status
            );
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to add audio (start=0s)",
            ));
        }
    } else {
        // startが0秒ではない場合、音声にディレイを加える
        let start_milliseconds = start.as_millis(); // start時間を秒に変換

        // 一時ファイル名を生成
        let temp_output_video = format!("{}_tmp.mp4", output_video);

        let status = Command::new("ffmpeg")
            .args([
                if is_overwrite { "-y" } else { "-n" },
                "-i",
                input_video,
                "-i",
                audio_file,
                "-c:v",
                if is_nvidia { "h264_nvenc" } else { "copy" }, // 映像エンコーディング
                "-c:a",
                "aac", // 音声をAACでエンコード
                "-b:v",
                "5M", // 映像ビットレート
                "-filter_complex",
                // `adelay` フィルタを使用して音声ファイルを遅延させる
                // `loudnorm` フィルタを使用して音声を正規化する
                &format!(
                    "[1:a]adelay={}|{},volume=1,loudnorm=I=-16:TP=-1.5:LRA=11.5[delayed_audio];[0:a][delayed_audio]amix=inputs=2:normalize=1:weights=1 {:.2}",
                    start_milliseconds, start_milliseconds,
                    1.0/weight as f32
                ),
                &temp_output_video,
            ])
            .status()?;

        // エラーチェック
        if !status.success() {
            eprintln!(
                "Error: Failed to add audio (start delay) - status: {:?}",
                status
            );
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to add audio (start delay)",
            ));
        }

        // 一時ファイルをリネーム
        fs::rename(&temp_output_video, output_video)?;
    }

    Ok(())
}

/// 指定Durationで動画を分割し、前方を保存する
pub fn cut(input_video: &str, output_video: &str, duration: Duration) -> Result<(), io::Error> {
    let is_overwrite = env::var("OVERWRITE").unwrap() == "true";
    let is_nvidia = env::var("NVIDIA").unwrap() == "true";

    let status = Command::new("ffmpeg")
        .args([
            if is_overwrite { "-y" } else { "-n" },
            "-i",
            input_video,
            "-c:v",
            if is_nvidia { "h264_nvenc" } else { "copy" }, // 映像エンコーディング
            "-c:a",
            "aac", // 音声をAACでエンコード
            "-b:v",
            "5M", // 映像ビットレート
            "-t",
            &format!("{}", duration.as_secs_f32()),
            output_video,
        ])
        .status()?;

    if !status.success() {
        eprintln!("Error: Failed to split video - status: {:?}", status);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to split video",
        ));
    }

    Ok(())
}

/// 指定Duration[start to end]で動画を切り抜き保存する
pub fn crop(
    input_video: &str,
    output_video: &str,
    start: Duration,
    end: Duration,
) -> Result<(), io::Error> {
    let is_overwrite = env::var("OVERWRITE").unwrap() == "true";
    let is_nvidia = env::var("NVIDIA").unwrap() == "true";

    let status = Command::new("ffmpeg")
        .args([
            if is_overwrite { "-y" } else { "-n" },
            "-i",
            input_video,
            "-c:v",
            if is_nvidia { "h264_nvenc" } else { "copy" }, // 映像エンコーディング
            "-c:a",
            "aac", // 音声をAACでエンコード
            "-b:v",
            "5M",  // 映像ビットレート
            "-ss", // 開始時間
            &format!("{}", start.as_secs_f32()),
            "-t", // 終了時間
            &format!("{}", end.as_secs_f32()),
            output_video,
        ])
        .status()?;

    if !status.success() {
        eprintln!("Error: Failed to split video - status: {:?}", status);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to split video",
        ));
    }

    Ok(())
}
