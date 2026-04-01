# rust sound

## Setup

<!-- language switcher (no JS, CSS-only control via :has) -->
<style>
.lang-content { display: none; }
:root:has(#language option[value="ja"]:checked) .ja,
:root:has(#language option[value="en"]:checked) .en {
  display: initial;
}
</style>

<div class="field-box">
  <label class="right" for="language">
    <span class="lang-content en">Language</span>
    <span class="lang-content ja">表示言語</span>
  </label>
  <select id="language" name="language">
    <option value="en" selected>English</option>
    <option value="ja">日本語</option>
  </select>
</div>

---

## <span class="lang-content en">Install</span><span class="lang-content ja">インストール</span>

<pre><code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt
cargo install just

sudo apt install -y libasound2-dev pkg-config
</code></pre>

---

## <span class="lang-content en">Run on Debug</span><span class="lang-content ja">デバッグ実行</span>

<pre><code>just run --file examples/twinkle.txt --pitch 1.2 --tempo 1.2 --volume 0.05 --soft 2 --square 0.05
</code></pre>

---

## <span class="lang-content en">Build</span><span class="lang-content ja">ビルド</span>

<pre><code>just build
</code></pre>

---

## <span class="lang-content en">Run (Release)</span><span class="lang-content ja">リリース実行</span>

<pre><code>target/release/sound --file examples/twinkle.txt --pitch 1.2 --tempo 1.2 --volume 0.05 --soft 2 --square 0.05
</code></pre>

---

## <span class="lang-content en">Project Structure</span><span class="lang-content ja">プロジェクト構成</span>

<pre><code>.
├── Cargo.toml
├── README.md
├── examples
│   ├── aimer-fast.txt
│   ├── aimer.txt
│   └── twinkle.txt
├── justfile
└── src
    └── main.rs
</code></pre>

---

## <span class="lang-content en">About This App</span><span class="lang-content ja">このアプリについて</span>

<p class="lang-content en">
This app is a <strong>text‑driven singing sound generator</strong>.  
You write a melody in a plain <code>.txt</code> file, and the app turns it into an expressive, human‑like voice in real time.
</p>

<p class="lang-content ja">
このアプリは、<strong>テキストでメロディを指定して歌わせるサウンドジェネレーター</strong>です。  
シンプルな <code>.txt</code> ファイルでメロディを記述すると、リアルタイムに人間味のある声のような音を生成します。
</p>

---

## <span class="lang-content en">Key Features</span><span class="lang-content ja">主な特徴</span>

### <span class="lang-content en">1. Compose with plain text</span><span class="lang-content ja">1. テキストでメロディを指定</span>

<p class="lang-content en">You can describe a melody using simple lines like:</p>
<p class="lang-content ja">次のようなシンプルな行でメロディを記述できます:</p>

<pre><code>C4 200 vol=0.8 attack=30 noise=0.1
D4 200
E4 400
</code></pre>

---

### <span class="lang-content en">2. Expressive, voice‑like sound</span><span class="lang-content ja">2. 声のような表現力のある音</span>

<ul class="lang-content en">
  <li>Attack (how the sound begins)</li>
  <li>Release (how it fades out)</li>
  <li>Noise (breathiness)</li>
  <li>Softness (warm, rounded tone)</li>
  <li>Square tone (a slightly aching, determined edge)</li>
  <li>Simple low‑pass smoothing for gentle voices</li>
</ul>

<ul class="lang-content ja">
  <li>アタック（音の立ち上がり）</li>
  <li>リリース（音の消え際の自然なフェード）</li>
  <li>ノイズ（息っぽさ）</li>
  <li>Soft（柔らかく丸いトーン）</li>
  <li>Square（少し切なく芯のあるトーン）</li>
  <li>簡易ローパスによる角の取れた柔らかい音</li>
</ul>

---

### <span class="lang-content en">3. Adjustable emotional tone</span><span class="lang-content ja">3. 感情表現をパラメータで調整</span>

<p class="lang-content en">You can shape the tone with simple flags:</p>
<p class="lang-content ja">コマンドラインのフラグで、感情的なニュアンスを調整できます:</p>

<ul class="lang-content en">
  <li><code>--soft 1.5</code> – warm, gentle, Aimer‑like tone</li>
  <li><code>--square 0.7</code> – slightly aching, determined square tone</li>
  <li><code>--soft 1.2 --square 0.3</code> – blended character</li>
</ul>

<ul class="lang-content ja">
  <li><code>--soft 1.5</code> – 柔らかく包み込むようなトーン（Aimer 風）</li>
  <li><code>--square 0.7</code> – 少し切なく、決意のこもった矩形波トーン</li>
  <li><code>--soft 1.2 --square 0.3</code> – 両者をブレンドしたニュアンス</li>
</ul>
