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

### Run on Debug

```
just run --file examples/twinkle.txt --pitch 1.2 --tempo 1.2 --volume 0.05 --soft 2 --square 0.05
```

### Build

```
just build
```

### Run (Release)

```
target/release/sound --file examples/twinkle.txt --pitch 1.2 --tempo 1.2 --volume 0.05 --soft 2 --square 0.05
```

### Project Structure

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

### About This App

This app is a **text‑driven singing sound generator**.  
You write a melody in a plain `.txt` file, and the app turns it into an expressive, human‑like voice in real time.

### Key Features

#### 1. Compose with plain text

```
C4 200 vol=0.8 attack=30 noise=0.1
D4 200
E4 400
```

#### 2. Expressive, voice‑like sound

- Attack (how the sound begins)  
- Release (natural fade‑out)  
- Noise (breathiness)  
- Softness (warm tone)  
- Square tone (emotional edge)  
- Simple low‑pass smoothing  

#### 3. Adjustable emotional tone

```
--soft 1.5
--square 0.7
--soft 1.2 --square 0.3
```

#### 4. Real‑time synthesis  
Audio is generated instantly — no exporting required.

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
just run --file examples/twinkle.txt --pitch 1.2 --tempo 1.2 --volume 0.05 --soft 2 --square 0.05
```

### ビルド

```
just build
```

### リリース実行

```
target/release/sound --file examples/twinkle.txt --pitch 1.2 --tempo 1.2 --volume 0.05 --soft 2 --square 0.05
```

### プロジェクト構成

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

### このアプリについて

このアプリは **テキストでメロディを指定して歌わせるサウンドジェネレーター** です。  
`.txt` ファイルにメロディを書くと、リアルタイムに人間味のある声のような音を生成します。

### 主な特徴

#### 1. テキストでメロディを指定

```
C4 200 vol=0.8 attack=30 noise=0.1
D4 200
E4 400
```

#### 2. 声のような表現力のある音

- アタック（立ち上がり）  
- リリース（自然な消え際）  
- ノイズ（息っぽさ）  
- Soft（柔らかいトーン）  
- Square（切なく芯のあるトーン）  
- 簡易ローパスで角を丸める  

#### 3. 感情表現をパラメータで調整

```
--soft 1.5
--square 0.7
--soft 1.2 --square 0.3
```

#### 4. リアルタイム合成  
書き出し不要で即時再生できます。

</details>
