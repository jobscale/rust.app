use clap::Parser;
use std::{
    env,
    f32::consts::PI,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(long, default_value = "1.0")]
    volume: f32,

    #[arg(long, default_value = "1.0")]
    tempo: f32,

    #[arg(long, default_value = "1.0")]
    pitch: f32,

    #[arg(long, default_value = "0.0")]
    soft: f32,

    #[arg(long, default_value = "0.0")]
    square: f32,
}

fn debug_log(msg: &str) {
    if let Ok(level) = env::var("LOG_LEVEL") {
        if level == "debug" {
            println!("[debug] {msg}");
        }
    }
}

fn error_log(msg: &str) {
    eprintln!("[error] {msg}");
}

fn note_to_freq(note: &str) -> Option<f32> {
    let (pitch, octave_str) = note.split_at(note.len() - 1);
    let octave: i32 = octave_str.parse().ok()?;

    let semitone = match pitch {
        "C" => -9,
        "C#" | "Db" => -8,
        "D" => -7,
        "D#" | "Eb" => -6,
        "E" => -5,
        "F" => -4,
        "F#" | "Gb" => -3,
        "G" => -2,
        "G#" | "Ab" => -1,
        "A" => 0,
        "A#" | "Bb" => 1,
        "B" => 2,
        _ => return None,
    };

    let n = semitone + (octave - 4) * 12;
    Some(440.0 * 2f32.powf(n as f32 / 12.0))
}

#[derive(Debug, Clone)]
struct VoiceParams {
    vol: f32,
    attack: f32,
}

#[derive(Debug, Clone)]
struct NoteEvent {
    freq: f32,
    duration_ms: u64,
    params: VoiceParams,
}

fn load_score(path: &PathBuf) -> Vec<NoteEvent> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut events = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let note = parts[0];
        let duration_ms: u64 = parts[1].parse().unwrap();
        let freq = note_to_freq(note).unwrap();

        let mut params = VoiceParams {
            vol: 1.0,
            attack: 20.0,
        };

        for p in &parts[2..] {
            if let Some(v) = p.strip_prefix("vol=") {
                params.vol = v.parse().unwrap();
            } else if let Some(v) = p.strip_prefix("attack=") {
                params.attack = v.parse().unwrap();
            }
        }

        events.push(NoteEvent {
            freq,
            duration_ms,
            params,
        });
    }

    events
}

fn synth_events(events: &[NoteEvent], args: &Args, sample_rate: u32) -> Vec<f32> {
    let mut samples = Vec::new();

    let volume = args.volume * 0.15;
    let soft_strength = args.soft.clamp(0.0, 1.0);

    for ev in events {
        let duration_ms = (ev.duration_ms as f32 / args.tempo) as f32;
        let total_samples = (duration_ms / 1000.0 * sample_rate as f32) as u32;

        let mut t = 0.0f32;
        let dt = 1.0 / sample_rate as f32;
        let freq = ev.freq * args.pitch;
        let mut last_sample = 0.0f32;

        for i in 0..total_samples {
            let sine = (2.0 * PI * freq * t).sin();
            let square_wave = if sine >= 0.0 { 1.0 } else { -1.0 };

            let mut base = sine * (1.0 - args.square) + square_wave * args.square;

            if soft_strength > 0.0 {
                let max_delta = 0.2 * soft_strength;
                let delta = base - last_sample;

                if delta > max_delta {
                    base = last_sample + max_delta;
                } else if delta < -max_delta {
                    base = last_sample - max_delta;
                }
            }

            last_sample = base;

            let attack_gain = (t * 1000.0 / ev.params.attack.max(0.0001)).min(1.0);

            let release_ms = 50.0;
            let time_left_sec = (total_samples - i) as f32 / sample_rate as f32;
            let release_gain = if time_left_sec < release_ms / 1000.0 {
                (time_left_sec * 1000.0 / release_ms).min(1.0)
            } else {
                1.0
            };

            let sample = base * ev.params.vol * volume * attack_gain * release_gain;

            samples.push(sample);
            t += dt;
        }
    }

    samples
}

fn play(events: Vec<NoteEvent>, args: &Args) {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device");

    let default_config = device.default_output_config().unwrap();
    let mut config = default_config.config();

    // Use larger buffer to prevent underruns
    config.buffer_size = cpal::BufferSize::Fixed(1024);

    let sample_rate = config.sample_rate;
    let channels = config.channels as usize;

    debug_log(&format!("sample_rate = {}", sample_rate));
    debug_log(&format!("channels = {}", channels));

    let mono_samples = synth_events(&events, args, sample_rate as u32);

    debug_log(&format!("generated samples = {}", mono_samples.len()));

    let shared = Arc::new(mono_samples);
    let index = Arc::new(AtomicUsize::new(0));

    let err_fn = |err: cpal::StreamError| {
        error_log(&format!("CPAL error: {err}"));
    };

    let shared_clone = shared.clone();
    let index_clone = index.clone();

    debug_log("building stream");

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _| {
                let total = shared_clone.len();
                let mut i = index_clone.load(Ordering::Relaxed);

                for frame in data.chunks_mut(channels) {
                    if i < total {
                        let sample = shared_clone[i];
                        for ch in frame.iter_mut() {
                            *ch = sample;
                        }
                        i += 1;
                    } else {
                        // Pad with silence after audio ends
                        for ch in frame.iter_mut() {
                            *ch = 0.0;
                        }
                    }
                }

                index_clone.store(i, Ordering::Relaxed);
            },
            err_fn,
            None,
        )
        .unwrap();

    debug_log("calling stream.play()");
    stream.play().unwrap();
    debug_log("stream.play() returned");

    let secs = shared.len() as f32 / sample_rate as f32 + 0.5;
    debug_log(&format!("sleeping for {:.2} seconds", secs));
    thread::sleep(Duration::from_secs_f32(secs));
}

fn main() {
    let args = Args::parse();
    let events = load_score(&args.file);
    play(events, &args);
}
