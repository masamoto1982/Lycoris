// 画面の組み立てとイベント処理。
//
// - 状態はすべて Rust コア（WASM）にあり、ここは「コマンドを送る」「スナップショットを描く」だけ。
// - 描き直しは領域ごとに、内容が変わったときだけ行う（問題1：無駄な再描画で操作が重くならない）。
//   入力中の値とフォーカスは描き直しの前後で引き継ぐ。
// - 確認ダイアログは使わない。結果はその画面に数秒だけ出るお知らせで返す（問題4）。

import init, { PosSimulator } from "./wasm/generated/lycoris_pos.js";
import type { Command, Snapshot } from "./types";
import {
  auditPanel,
  cashPanel,
  customerScreen,
  esc,
  matPanel,
  ownerPanel,
  scannerPanel,
  sideOf,
  simPanel,
  staffCalls,
  staffScreen,
} from "./view";

declare global {
  interface Window {
    /** 単一 HTML 版でだけ埋め込まれる WASM 本体（base64） */
    __LYCORIS_WASM_B64__?: string;
  }
}

const app = document.getElementById("app")!;
let sim: PosSimulator;
let t0 = performance.now();
let snap: Snapshot;
const cache = new WeakMap<Element, string>();
const toastTimers = new Map<string, number>();

function bytesFromB64(b64: string): Uint8Array {
  const bin = atob(b64);
  const out = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
  return out;
}

async function boot(): Promise<void> {
  const b64 = window.__LYCORIS_WASM_B64__;
  await init(b64 ? { module_or_path: bytesFromB64(b64) } : undefined);
  reset(2, 1);
  window.setInterval(frame, 100);
}

function reset(registers: number, mats: number): void {
  sim?.free();
  sim = new PosSimulator(Date.now() & 0xffff, registers, mats);
  t0 = performance.now();
  snap = JSON.parse(sim.snapshot()) as Snapshot;
  buildSkeleton(registers, mats);
  render();
}

// ---------------------------------------------------------------------------
// 骨組み（一度だけ作る）
// ---------------------------------------------------------------------------

function buildSkeleton(registers: number, mats: number): void {
  const regOpts = Array.from({ length: 10 }, (_, i) => i + 1)
    .map((n) => `<option value="${n}" ${n === registers ? "selected" : ""}>レジ${n}台</option>`)
    .join("");
  const matOpts = Array.from({ length: 5 }, (_, i) => i + 1)
    .map((n) => `<option value="${n}" ${n === mats ? "selected" : ""}>MAT${n}台</option>`)
    .join("");

  const regs = snap.registers
    .map(
      (r) => `<article class="register">
      <h2>レジ${r.id} <span data-region="mode-${r.id}"></span></h2>
      <div class="screens">
        <section class="screen staff-screen">
          <div class="screen-title">店員画面</div>
          <div data-region="calls-${r.id}"></div>
          <div data-region="staff-${r.id}"></div>
          <div class="toast" id="toast-${r.id}-staff"></div>
        </section>
        <section class="screen customer-screen">
          <div class="screen-title">お客様画面</div>
          <div data-region="cust-${r.id}"></div>
          <div class="toast" id="toast-${r.id}-customer"></div>
        </section>
      </div>
      <div class="hw">
        <div class="hw-block"><h3>店員側スキャナー</h3>${scannerPanel(r.id, "staff", snap)}</div>
        <div class="hw-block"><h3>お客様側スキャナー</h3>${scannerPanel(r.id, "customer", snap)}</div>
        <div class="hw-block"><h3>紙幣・硬貨の投入口／電子マネー端末（お客様側）</h3>${cashPanel(r.id)}</div>
      </div>
    </article>`,
    )
    .join("");

  const pw = snap.accounts.map((a) => `${esc(a.name)}（名札 ${esc(a.barcode)}）`).join("、");
  app.innerHTML = `
    <header class="top">
      <div>
        <h1>Lycoris セミセルフPOS シミュレーター <span class="muted">叩き台</span></h1>
        <p class="muted">レジ1台につき状態は1つ。店員画面とお客様画面は同じ取引の見え方違いです。</p>
      </div>
      <div class="row">
        <select id="cfg-reg">${regOpts}</select><select id="cfg-mat">${matOpts}</select>
        <button id="cfg-apply" class="ghost">この構成で作り直す</button>
      </div>
    </header>
    <div class="layout">
      <div class="registers">${regs}</div>
      <aside class="side">
        <section class="panel"><h3>MAT（呼び鈴）</h3><div class="mats" data-region="mats"></div>
          <div class="toast" id="toast-store"></div></section>
        <section class="panel"><h3>試し方</h3>
          <ol class="help">
            <li>レジ1（セミセルフ）：店員側スキャナーで商品を読み、店員画面の「会計」。お客様画面で支払方法を選ぶか現金を投入。</li>
            <li>レジ2（セルフ）：お客様側で「缶ビール」→「お会計」。年確の呼び出しで MAT が鳴ります。店員側で名札を読むと、お客様画面の「20歳以上です」が押せます。</li>
            <li>FF：レジ2のお客様画面で「FF を注文」→ 名札 → FF 登録 →「お渡し」→ お客様画面で「受け取りました」。</li>
            <li>会計画面で何かを読むと、登録画面に戻って追加されます（投入済みの現金はそのまま）。</li>
            <li>出金パスワードは <b>7777</b>（ログインとは別）。オーナーは 9001 / <b>9999</b>。</li>
          </ol>
          <p class="muted">店員：${pw}</p>
        </section>
        <section class="panel"><h3>障害を起こす（シミュレーター用）</h3><div data-region="sim"></div></section>
        <details class="panel"><summary><h3>年確ルール（オーナー設定）</h3></summary><div data-region="owner"></div></details>
        <details class="panel"><summary><h3>出金の記録</h3></summary><div data-region="audit"></div></details>
      </aside>
    </div>`;
}

// ---------------------------------------------------------------------------
// 描画
// ---------------------------------------------------------------------------

function setRegion(name: string, html: string): void {
  const el = app.querySelector(`[data-region="${name}"]`);
  if (!el || cache.get(el) === html) return;
  cache.set(el, html);
  // 入力中の値とフォーカスを引き継ぐ
  const values = new Map<string, string>();
  el.querySelectorAll<HTMLInputElement | HTMLSelectElement>("[data-key]").forEach((i) => values.set(i.dataset.key!, i.value));
  const active = document.activeElement as HTMLInputElement | null;
  const activeKey = active && el.contains(active) ? active.dataset.key : undefined;
  const sel = activeKey && "selectionStart" in active! ? active!.selectionStart : null;
  const open = new Set<number>();
  el.querySelectorAll("details").forEach((d, i) => d.open && open.add(i));
  const hadDetails = el.querySelectorAll("details").length > 0;

  el.innerHTML = html;

  el.querySelectorAll<HTMLInputElement | HTMLSelectElement>("[data-key]").forEach((i) => {
    const v = values.get(i.dataset.key!);
    if (v !== undefined) i.value = v;
  });
  if (hadDetails) el.querySelectorAll("details").forEach((d, i) => (d.open = open.has(i)));
  if (activeKey) {
    const next = el.querySelector<HTMLInputElement>(`[data-key="${activeKey}"]`);
    if (next) {
      next.focus();
      if (sel !== null && next.type !== "number") next.setSelectionRange(sel, sel);
    }
  }
}

function render(): void {
  for (const r of snap.registers) {
    setRegion(`mode-${r.id}`, `<span class="mode mode-${r.mode}">${esc(r.mode_label)}</span>`);
    setRegion(`calls-${r.id}`, staffCalls(r, snap.now));
    setRegion(`staff-${r.id}`, staffScreen(r, snap));
    setRegion(`cust-${r.id}`, customerScreen(r));
  }
  setRegion("mats", matPanel(snap));
  setRegion("sim", simPanel(snap));
  setRegion("owner", ownerPanel(snap));
  setRegion("audit", auditPanel(snap));
}

function frame(): void {
  sim.tick(performance.now() - t0);
  snap = JSON.parse(sim.snapshot()) as Snapshot;
  render();
}

function toast(where: string, text: string, error: boolean): void {
  const el = document.getElementById(where);
  if (!el) return;
  el.textContent = text;
  el.className = `toast show ${error ? "error" : ""}`;
  window.clearTimeout(toastTimers.get(where));
  toastTimers.set(
    where,
    window.setTimeout(() => (el.className = "toast"), error ? 4000 : 2500),
  );
}

function send(cmd: Command): void {
  sim.tick(performance.now() - t0);
  const res = JSON.parse(sim.dispatch(JSON.stringify(cmd))) as { ok: boolean; message?: string; error?: string };
  const side = sideOf(cmd);
  const where = side === "store" ? "toast-store" : `toast-${cmd.register}-${side}`;
  if (res.ok) {
    if (res.message) toast(where, res.message, false);
  } else {
    toast(where, res.error ?? "操作できません", true);
  }
  frame();
}

// ---------------------------------------------------------------------------
// イベント（委譲。描き直しでボタンが入れ替わっても取りこぼさない）
// ---------------------------------------------------------------------------

const val = (key: string): string => app.querySelector<HTMLInputElement>(`[data-key="${key}"]`)?.value.trim() ?? "";
const clear = (key: string): void => {
  const el = app.querySelector<HTMLInputElement>(`[data-key="${key}"]`);
  if (el) el.value = "";
};

function form(b: HTMLElement): void {
  const reg = Number(b.dataset.reg);
  switch (b.dataset.form) {
    case "scan": {
      const key = `bc-${reg}-${b.dataset.side}`;
      const code = val(key);
      if (code) send({ type: "scan", register: reg, side: b.dataset.side, code });
      clear(key);
      break;
    }
    case "service":
      send({ type: "complete_service", register: reg, line_id: Number(b.dataset.line), amount: Number(val(`svc-${reg}-${b.dataset.line}`) || 0) });
      break;
    case "cashout":
      send({
        type: "cash_out",
        register: reg,
        kind: val(`co-kind-${reg}`),
        amount: Number(val(`co-amt-${reg}`) || 0),
        password: val(`co-pw-${reg}`),
      });
      clear(`co-pw-${reg}`);
      break;
    case "add-rule": {
      const kind = val("ar-kind");
      const target = kind === "product" ? { kind, code: val("ar-code") } : { kind, category: val("ar-cat") };
      send({ type: "add_age_rule", owner_id: val("ow-id"), password: val("ow-pw"), target, min_age: Number(val("ar-age")) });
      clear("ow-pw");
      break;
    }
    case "remove-rule":
      send({ type: "remove_age_rule", owner_id: val("ow-id"), password: val("ow-pw"), rule_id: Number(b.dataset.rule) });
      clear("ow-pw");
      break;
  }
}

app.addEventListener("click", (e) => {
  const b = (e.target as Element).closest("button");
  if (!b || b.disabled) return;
  if (b.id === "cfg-apply") {
    const r = Number((document.getElementById("cfg-reg") as HTMLSelectElement).value);
    const m = Number((document.getElementById("cfg-mat") as HTMLSelectElement).value);
    reset(r, m);
    return;
  }
  if (b.dataset.cmd) send(JSON.parse(b.dataset.cmd) as Command);
  else if (b.dataset.form) form(b);
});

app.addEventListener("keydown", (e) => {
  const t = e.target as HTMLInputElement;
  if (e.key !== "Enter" || !t.dataset.scan) return;
  const [reg, side] = t.dataset.scan.split(":");
  const code = t.value.trim();
  if (code) send({ type: "scan", register: Number(reg), side, code });
  t.value = "";
});

boot().catch((e) => {
  app.innerHTML = `<p class="error">読み込みに失敗しました：${esc(e)}</p>`;
});
