extern crate argparse;
extern crate dirs;

use std::io::BufReader;
use std::env;
use std::fs::File;
use argparse::{ArgumentParser, Store};
use std::process::exit;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut minutes = 25;
    let mut chime_count = 3;
    let mut volume = 0.5;
    configure(&mut minutes, &mut chime_count, &mut volume);
    
    let alarm_file_name = get_alarm_file_name();

    println!("Starting a {} minute session. May you be focused and diligent.", minutes);
    if minutes > 0 {
        sleep(Duration::from_millis(60000 * minutes));
    }
    println!("{} minutes have passed. You should distract yourself.", minutes);
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    sink.set_volume(volume);
    if chime_count > 0 {
        for _x in 0..chime_count {
            let decoder = rodio::Decoder::new(BufReader::new(File::open(&alarm_file_name).unwrap())).unwrap();
            sink.append(decoder);
            sink.sleep_until_end();
            sleep(Duration::from_millis(1000));
        }
    }
}

fn get_alarm_file_name() -> String {
    let file_names = [
        "alarm.ogg",
        "alarm.wav",
        "alarm.mp3"
    ];
    let mut file_dirs = Vec::new();
    match dirs::home_dir() {
        Some(home_dir) => file_dirs.push(home_dir.join(".pomidorka")),
        None => ()
    };
    file_dirs.push(PathBuf::from("/usr/share/pomidorka"));
    match env::current_dir() {
        Ok(current_dir) => file_dirs.push(current_dir),
        Err(_) => ()
    };
    for file_dir in file_dirs.iter() {
        for file_name in file_names.iter() {
            let file_full_path = file_dir.join(file_name);
            let file_result = File::open(&file_full_path);
            match file_result {
                Ok(_) => { return file_full_path.to_str().unwrap().to_string() },
                Err(_) => (),
            }
        }
    }
    println!("ERROR: alarm audio file not found, looked for {:?} in {:?}", file_names, file_dirs);
    exit(-1)
}

fn configure(minutes: &mut u64, chime_count: &mut u32, volume: &mut f32) {
    {
    let mut ap = ArgumentParser::new();
    ap.refer(minutes)
        .add_option(&["-m", "--minutes"], Store, "Length of your pomodoro session in minutes, 25 minutes by default");
    ap.refer(volume)
        .add_option(&["-v", "--volume"], Store, "Chimes volume, values from 0 to 1, 0.5 by default");
    ap.refer(chime_count)
        .add_option(&["-c", "--chimecount"], Store, "Number of times to play the chimes, 3 by default");
    ap.parse_args_or_exit();
    }
    if *volume > 1.0 {
        println!("ERROR: Volume is too big. It should be within range of [0, 1], e.g. 0.5");
        exit(-1)
    } else if *volume < 0.0 {
        println!("ERROR: Volume is too low. It should be within range of [0, 1], e.g. 0.5");
        exit(-1)
    }
}

