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

// ---------------------------------------------------------------------------
// 呼び出し音の鳴らし分け
//
// - 同時に鳴らさない：MAT やレジが同時に鳴ると、ずれた音が重なって曲のように聞こえ、
//   どれが鳴っているのか分からなくなる。呼び出し音は 1 本の「鳴動チャンネル」に
//   順番に並べ、前の音が終わってから次を鳴らす（待っている間に解決したものは鳴らさない）。
// - 放置ほど大きく：音量はコアの ring_volume（呼び出しの放置時間で決まる）に従う。
//   先に呼んだレジほど大きく鳴るので、どのレジが最初か音で分かる。
// ---------------------------------------------------------------------------

export type RingSource = {
  /** "mat" または "reg-1" など */
  id: string;
  kind: "mat" | "register";
  /** コアが鳴動中としている（立ち上がりで鳴動を予約する） */
  ringing: boolean;
  /** 0〜1。0 なら呼び出しは片付いている */
  volume: number;
};

const queue: string[] = [];
const sources = new Map<string, RingSource>();
const wasRinging = new Map<string, boolean>();
let playing: { id: string; bus: GainNode; timer: number } | null = null;

function pattern(kind: RingSource["kind"], bus: AudioNode): number {
  if (kind === "register") {
    // レジ本体：約6秒（コアの REG_BUZZ_MS）
    for (let i = 0; i < 20; i++) tone(i % 2 ? 784 : 1047, i * 0.3, 0.22, "square", 0.12, bus);
    return 6000;
  }
  // MAT：約3秒（MAT の鳴動時間）
  for (let i = 0; i < 6; i++) {
    tone(1320, i * 0.5, 0.18, "triangle", 0.28, bus);
    tone(990, i * 0.5 + 0.2, 0.25, "triangle", 0.28, bus);
  }
  return 3000;
}

function playNext(): void {
  if (playing || !ready() || !ctx) return;
  while (queue.length) {
    const id = queue.shift()!;
    const src = sources.get(id);
    if (!src || src.volume <= 0) continue; // 待っている間に片付いた
    const bus = ctx.createGain();
    bus.gain.value = src.volume;
    bus.connect(ctx.destination);
    const ms = pattern(src.kind, bus);
    playing = { id, bus, timer: window.setTimeout(finish, ms + 150) };
    return;
  }
}

function finish(): void {
  if (!playing) return;
  window.clearTimeout(playing.timer);
  playing.bus.disconnect();
  playing = null;
  playNext();
}

/** 毎フレーム、いまの鳴動元を渡す。立ち上がりで予約し、片付いたら鳴っていても止める */
export function updateRings(list: RingSource[]): void {
  for (const s of list) {
    sources.set(s.id, s);
    if (s.ringing && !wasRinging.get(s.id) && playing?.id !== s.id && !queue.includes(s.id)) queue.push(s.id);
    wasRinging.set(s.id, s.ringing);
  }
  if (playing && (sources.get(playing.id)?.volume ?? 0) <= 0) finish();
  playNext();
}
