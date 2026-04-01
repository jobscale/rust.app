# rust sound

## Multilingual README
Choose your language:

- [English](#english)
- [日本語](#日本語)

---

# English

<details open>
<summary><strong>English</strong></summary>

## Setup

### Install

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt
cargo install just

sudo apt install -y libasound2-dev pkg-config
```

### Run (Debug)

```
just run --file examples/twinkle.txt --pitch 1.2 --tempo 1.2 --volume 0.6 --soft 0.2 --square 0.2
```

### Build

```
just build
```

### Run (Release)

```
target/release/sound --file examples/twinkle.txt --pitch 1.2 --tempo 1.2 --volume 0.6 --soft 0.2 --square 0.2
```

---

## Project Structure

```
.
├── Cargo.toml
├── README.md
├── examples
│   ├── aimer-fast.txt
│   ├── aimer.txt
│   └── twinkle.txt
├── justfile
└── src
    └── main.rs
```

---

## About This App

This application is a **text‑driven melodic sound synthesizer**.
You write a melody in a simple `.txt` file, and the program generates expressive tones in real time using pure Rust and `cpal`.

---

## Key Features

### 1. Compose with plain text

```
C4 200 vol=0.8 attack=30
D4 200
E4 400
```

Each line represents:

```
NOTE  DURATION(ms)  OPTIONAL_PARAMETERS
```

### 2. Expressive, voice‑like synthesis

The engine supports:

- **Attack** — controls how quickly the sound begins
- **Release** — natural fade‑out
- **Volume** — normalized (1.0 = 15% internal amplitude)
- **Square mix** — blends sine and square wave
- **Soft limiter** — reduces harsh waveform jumps without changing tone

### 3. Soft limiter (waveform smoothing)

`--soft` controls a **delta‑limiter** that reduces sudden waveform jumps.

- `--soft 0.0` → no smoothing
- `--soft 0.1` → mild smoothing
- `--soft 1.0` → strong smoothing

This removes harsh digital edges while keeping the original tone.

### 4. Real‑time synthesis

Audio is generated on the fly using `cpal` — no rendering or exporting required.

</details>

---

# 日本語

<details>
<summary><strong>日本語</strong></summary>

## セットアップ

### インストール

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt
cargo install just

sudo apt install -y libasound2-dev pkg-config
```

### デバッグ実行

```
just run --file examples/twinkle.txt --pitch 1.2 --tempo 1.2 --volume 0.6 --soft 0.2 --square 0.2
```

### ビルド

```
just build
```

### リリース実行

```
target/release/sound --file examples/twinkle.txt --pitch 1.2 --tempo 1.2 --volume 0.6 --soft 0.2 --square 0.2
```

---

## プロジェクト構成

```
.
├── Cargo.toml
├── README.md
├── examples
│   ├── aimer-fast.txt
│   ├── aimer.txt
│   └── twinkle.txt
├── justfile
└── src
    └── main.rs
```

---

## このアプリについて

このアプリは **テキストでメロディを指定して音を生成するシンプルなシンセサイザー** です。
`.txt` ファイルに音階と長さを書くと、Rust と `cpal` によってリアルタイムに音が鳴ります。

---

## 主な特徴

### 1. テキストでメロディを記述

```
C4 200 vol=0.8 attack=30
D4 200
E4 400
```

1 行が 1 音を表します：

```
音階  長さ(ms)  オプションパラメータ
```

### 2. 声のような表現力のある音

- **アタック**（立ち上がり）
- **リリース**（自然な減衰）
- **ボリューム**（1.0 = 内部振幅 15%）
- **スクエアミックス**（サイン波と矩形波のブレンド）
- **ソフトリミッター**（波形の急激な乱れを抑える）

### 3. ソフトリミッター（波形乱れ補正）

`--soft` は **波形の変化量を制限するリミッター** です。

- `--soft 0.0` → 補正なし
- `--soft 0.1` → 軽い補正
- `--soft 1.0` → 強い補正

音色を変えずに、デジタル的な「バリッ」というノイズだけを抑えます。

### 4. リアルタイム合成

`cpal` によるリアルタイム再生で、書き出しは不要です。

</details>
