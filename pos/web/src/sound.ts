// 実機の音を Web Audio で鳴らす（音源ファイルは使わず、その場で合成する）。
// ブラウザは操作があるまで音を出せないので、最初のタッチで準備する。
// 呼び出しの音（MAT・レジ）は聞き逃しを防ぐため、消す手段を設けない。

let ctx: AudioContext | null = null;

export function unlockAudio(): void {
  if (!ctx) {
    const AC = window.AudioContext ?? (window as unknown as { webkitAudioContext?: typeof AudioContext }).webkitAudioContext;
    if (!AC) return;
    ctx = new AC();
  }
  if (ctx.state === "suspended") void ctx.resume();
}

function tone(freq: number, start: number, dur: number, type: OscillatorType, gain: number, out?: AudioNode): void {
  if (!ctx) return;
  const t = ctx.currentTime + start;
  const osc = ctx.createOscillator();
  const g = ctx.createGain();
  osc.type = type;
  osc.frequency.value = freq;
  // クリック音が出ないよう、立ち上がりと減衰をつける
  g.gain.setValueAtTime(0, t);
  g.gain.linearRampToValueAtTime(gain, t + 0.005);
  g.gain.exponentialRampToValueAtTime(0.0001, t + dur);
  osc.connect(g).connect(out ?? ctx.destination);
  osc.start(t);
  osc.stop(t + dur + 0.02);
}

function ready(): boolean {
  return ctx !== null && ctx.state === "running";
}

/** 商品を登録したときの「ピッ」 */
export function beepScan(): void {
  if (!ready()) return;
  tone(2400, 0, 0.09, "square", 0.08);
}

/** 読めなかった・受け付けられなかったときの「ブッブー」 */
export function beepError(): void {
  if (!ready()) return;
  tone(220, 0, 0.16, "square", 0.07);
  tone(220, 0.2, 0.22, "square", 0.07);
}

/**
 * 呼び出し中のレジ本体の鳴動（約6秒。コアの REG_BUZZ_MS に合わせる）。
 * MAT とは別の音色。店員が来たら途中でも止められるよう、止める関数を返す
 */
export function ringRegister(): () => void {
  if (!ready() || !ctx) return () => {};
  const bus = ctx.createGain();
  bus.connect(ctx.destination);
  for (let i = 0; i < 20; i++) {
    tone(i % 2 ? 784 : 1047, i * 0.3, 0.22, "square", 0.06, bus);
  }
  return () => bus.disconnect();
}

/** MAT の鳴動（約3秒の呼び出し音。MAT の鳴動時間に合わせる） */
export function ringMat(): void {
  if (!ready()) return;
  for (let i = 0; i < 6; i++) {
    tone(1320, i * 0.5, 0.18, "triangle", 0.14);
    tone(990, i * 0.5 + 0.2, 0.25, "triangle", 0.14);
  }
}
