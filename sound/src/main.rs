use clap::Parser;
use rand::{RngExt, rng};
use rodio::{OutputStream, Sink, Source};
use std::f32::consts::PI;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    time::Duration,
};

// -----------------------------
// CLI Args
// -----------------------------
#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(long, default_value = "0.15")]
    volume: f32,

    #[arg(long, default_value = "1.0")]
    tempo: f32,

    #[arg(long, default_value = "1.0")]
    pitch: f32,

    #[arg(long, default_value = "0.0")]
    soft: f32, // 柔らかさ

    #[arg(long, default_value = "0.0")]
    square: f32, // 矩形波の強さ
}

// -----------------------------
// A4=440Hz note_to_freq
// -----------------------------
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

// -----------------------------
// struct VoiceParams
// -----------------------------
#[derive(Debug, Clone)]
struct VoiceParams {
    vol: f32,
    attack: f32,
    noise: f32,
}

// -----------------------------
// struct NoteEvent
// -----------------------------
#[derive(Debug, Clone)]
struct NoteEvent {
    freq: f32,
    duration_ms: u64,
    params: VoiceParams,
}

// -----------------------------
// load txt source
// -----------------------------
fn load_score(path: &PathBuf) -> Vec<NoteEvent> {
    let file = File::open(path).unwrap_or_else(|_| panic!("file not found: {:?}", path));
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

        let freq = note_to_freq(note).unwrap_or_else(|| panic!("invalid note: {}", note));

        let mut params = VoiceParams {
            vol: 1.0,
            attack: 20.0,
            noise: 0.0,
        };

        for p in &parts[2..] {
            if let Some(v) = p.strip_prefix("vol=") {
                params.vol = v.parse().unwrap();
            } else if let Some(v) = p.strip_prefix("attack=") {
                params.attack = v.parse().unwrap();
            } else if let Some(v) = p.strip_prefix("noise=") {
                params.noise = v.parse().unwrap();
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

// -----------------------------
// struct VoiceWave
// -----------------------------
struct VoiceWave {
    freq: f32,
    sample_rate: u32,
    t: f32,
    duration_samples: u32,
    params: VoiceParams,
    soft: f32,
    square: f32,
    last_sample: f32,
}

impl Iterator for VoiceWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.duration_samples == 0 {
            return None;
        }

        let t = self.t;
        let freq_mod = self.freq;
        let sine = (2.0 * PI * freq_mod * t).sin();
        let square_wave = if sine >= 0.0 { 1.0 } else { -1.0 };
        let mut base = sine * (1.0 - self.square) + square_wave * self.square;
        let noise_amount = self.params.noise * (1.0 - self.soft.clamp(0.0, 1.0));
        let mut rng = rng();
        let noise = noise_amount * rng.random_range(-1.0..1.0);
        let attack_scale = 1.0 + self.soft;
        let attack_gain = (t * 1000.0 / (self.params.attack * attack_scale)).min(1.0);
        let release_ms = 50.0;
        let time_left_sec = self.duration_samples as f32 / self.sample_rate as f32;
        let release_gain = if time_left_sec < release_ms / 1000.0 {
            (time_left_sec * 1000.0 / release_ms).min(1.0)
        } else {
            1.0
        };

        if self.soft > 0.0 {
            let alpha = 0.1 * self.soft.clamp(0.0, 1.0);
            base = self.last_sample + alpha * (base - self.last_sample);
            self.last_sample = base;
        }

        let sample = (base + noise) * self.params.vol * attack_gain * release_gain;

        self.t += 1.0 / self.sample_rate as f32;
        self.duration_samples -= 1;

        Some(sample)
    }
}

impl Source for VoiceWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

// -----------------------------
// Play events
// -----------------------------
fn play(events: Vec<NoteEvent>, args: &Args) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for ev in events {
        let duration = (ev.duration_ms as f32 / args.tempo) as f32;
        let samples = (duration / 1000.0 * 44100.0) as u32;

        let wave = VoiceWave {
            freq: ev.freq * args.pitch,
            sample_rate: 44100,
            t: 0.0,
            duration_samples: samples,
            params: VoiceParams {
                vol: ev.params.vol * args.volume,
                attack: ev.params.attack,
                noise: ev.params.noise,
            },
            soft: args.soft,
            square: args.square,
            last_sample: 0.0,
        };

        sink.append(wave);
    }

    sink.sleep_until_end();
}

// -----------------------------
// main
// -----------------------------
fn main() {
    let args = Args::parse();
    let events = load_score(&args.file);
    play(events, &args);
}
