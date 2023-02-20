#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,screen_shot,start_stream,start_recording,end_stream,save_stream1,display_stream,screenshot_stream])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn screen_shot() -> () {
    //https://doc.rust-lang.org/std/process/struct.Command.html
    let mut screenshotcmd = Command::new("ffmpeg");
    screenshotcmd.arg("-f");
    screenshotcmd.arg("gdigrab");
    screenshotcmd.arg("-i");
    screenshotcmd.arg("desktop");
    screenshotcmd.arg("-frames:v");
    screenshotcmd.arg("1");
    screenshotcmd.arg("-strftime");
    screenshotcmd.arg("1");
    //screenshotcmd.arg("F:\\School\\Capstone\\output%H%M%S.png");
    screenshotcmd.arg("E:\\sers\\ams676\\FFMPEGPROJECTCACHE\\output%H%M%S.png");
    screenshotcmd
        .status()
        .expect("DID NTO WORK LOSER");
}

#[tauri::command]
fn start_stream() -> () {
    let mut startstreamcmd = Command::new("ffmpeg");
    startstreamcmd.arg("-f");
    startstreamcmd.arg("gdigrab");
    startstreamcmd.arg("-framerate");
    startstreamcmd.arg("60");
    startstreamcmd.arg("-i");
    startstreamcmd.arg("desktop");
    startstreamcmd.arg("-vcodec");
    startstreamcmd.arg("mpeg4");
    startstreamcmd.arg("-q");
    startstreamcmd.arg("12");
    startstreamcmd.arg("-f");
    startstreamcmd.arg("mpegts");
    startstreamcmd.arg("udp://127.0.0.1:3000");
    //must add framerate and port number config to settings
    //add right intputs for desktop audio and microphone audio
    startstreamcmd
        .status()
        .expect("did NOT WORK STARTING STREAM");
}

#[tauri::command]
fn display_stream() -> () {
    let mut playstreamcmd = Command::new("ffplay");
    playstreamcmd.arg("udp://127.0.0.1:3000");
    playstreamcmd.status().expect("dID NOT PLAY STREAM");
}
#[tauri::command]
fn save_stream1() -> () {
    let mut savestreamcmd = Command::new("ffmpeg");
    savestreamcmd.arg("-i");
    savestreamcmd.arg("udp://127.0.0.1:3000");
    savestreamcmd.arg("-f");
    savestreamcmd.arg("segment");
    savestreamcmd.arg("-segment_time");
    savestreamcmd.arg("15"); //should be 1.5 times as long as the action replay
    savestreamcmd.arg("-segment_wrap");
    savestreamcmd.arg("2");
    savestreamcmd.arg("-vcodec");
    savestreamcmd.arg("mpeg4");
    savestreamcmd.arg("-strftime");
    savestreamcmd.arg("1");
    savestreamcmd.arg("F:\\School\\output%H%M%S.mp4");
    savestreamcmd
        .status()
        .expect("save stream DID NTO WORK LOSER");
}

#[tauri::command]
fn start_recording() -> () {
    let mut startrecording = Command::new("ffmpeg");
    startrecording.arg("-rtbufsize");
    startrecording.arg("1500M");
    startrecording.arg("-thread_queue_size");
    startrecording.arg("512");
    startrecording.arg("gdigrab");
    startrecording.arg("-video_size"); //should be 1.5 times as long as the action replay
    startrecording.arg("2560x1440");
    startrecording.arg("-i");
    startrecording.arg("desktop");
    startrecording.arg("-f");
    startrecording.arg("dshow");
    startrecording.arg("-i");
  //  startrecording.arg("audio=\"Microphone (Arctis Nova Pro Wireless)\"");
    startrecording.arg("-crf");
    startrecording.arg("0");
    startrecording.arg("-filter:a");
    startrecording.arg("\"volume=1.5\"");
    startrecording.arg("-vcodec");
    startrecording.arg("libx264");
    startrecording.arg("-strftime");
    startrecording.arg("1");
    startrecording.arg("F:\\School\\output%H%M%S.mp4");
    startrecording.status().expect("DID NTO WORK LOSER");
}

#[tauri::command]
fn end_stream() -> () {
    let mut end_stream = Command::new("q");
    end_stream.status().expect("NOOO! stream end");
}
#[tauri::command]
fn screenshot_stream() -> () {
    let mut screenshot_stream = Command::new("ffmpeg");
    screenshot_stream.arg("-i");
    screenshot_stream.arg("udp://127.0.0.1:3000");
    screenshot_stream.arg("-strftime");
    screenshot_stream.arg("1");
    screenshot_stream.arg("F:\\School\\output%H%M%S.mp4");
    screenshot_stream
        .status()
        .expect("no screenshot stream DID NTO WORK LOSER");
}

// fn alt_start_end_stream() -> (){
//     let mut startstreamcmd = Command::new("ffmpeg")
//     .arg("-f")
//     .arg("gdigrab")
//     .arg("-framerate")
//     .arg("60")
//     .arg("-i")
//     .arg("desktop")
//     .arg("-vcodec")
//     .arg("mpeg4")
//     .arg("-q")
//     .arg("12")
//     .arg("-f")
//     .arg("mpegts")
//     .arg("udp://127.0.0.1:3000")
//     .stdin(Stdio::piped())
//     .spawn();

//     let mut stdin = startstreamcmd.stdin.take().unwrap();

//     let mut kill = Command::new("kill")
//         .arg(startstreamcmd.id().to_string())
//         .spawn()?;
//     kill.wait();
// }

// fn alt_start_endstream() -> Result<Receiver<()>, ctrlc::Error> {
//     let (sender, receiver) = bounded(100);
//     ctrlc::set_handler(move || {
//         let _ = sender.send(());
//     })?;

//     Ok(receiver)
// }
