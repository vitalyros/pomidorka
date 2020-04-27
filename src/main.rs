use std::io::BufReader;
use std::thread;
use std::env;
use std::fs::File;

fn main() {
    let args_vec: Vec<String> = env::args().collect();  
    let minutes = getMinutes(&args_vec);
    println!("Starting a {} minute session. May you be focused and diligent.", minutes);
    if (minutes > 0) {
        for current_minite in 0..minutes {
            thread::sleep_ms(60000)
        }
    }
    println!("{} minutes have passed. You should distract yourself.", minutes);
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    sink.set_volume(0.05);
    for x in 0..3 {
        let file = std::fs::File::open("music_box.mp3").unwrap();
        let reader = BufReader::new(file);
        let decoder = rodio::Decoder::new(reader).unwrap();
        sink.append(decoder);
    }
    sink.sleep_until_end();
}

fn getMinutes(args: &Vec<String>) -> u8 {
    let args_len = args.len();
    return if (args_len > 1) {
        args[args_len - 1].parse().unwrap()
    } else {
        25
    }
}