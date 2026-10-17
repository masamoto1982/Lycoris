// 画面の組み立てとイベント処理。
//
// - 状態はすべて Rust コア（WASM）にあり、ここは「コマンドを送る」「スナップショットを描く」だけ。
// - 描き直しは領域ごとに、内容が変わったときだけ行う（問題1：無駄な再描画で操作が重くならない）。
//   入力中の値とフォーカスは描き直しの前後で引き継ぐ。
// - 確認ダイアログは使わない。結果はその画面に数秒だけ出るお知らせで返す（問題4）。
// - 買い物は、右の売り場から商品をレジの画面へドラッグ＆ドロップして再現する。
//   店員画面に落とせば店員側スキャナー、お客様画面に落とせばお客様側スキャナーで読んだことになる。

import init, { PosSimulator } from "./wasm/generated/janus_pos.js";
import type { Command, Snapshot } from "./types";
import {
  auditPanel,
  customerScreen,
  esc,
  matPanel,
  ownerPanel,
  shelfPanel,
  sideOf,
  simPanel,
  staffCalls,
  staffScreen,
} from "./view";

declare global {
  interface Window {
    /** 単一 HTML 版でだけ埋め込まれる WASM 本体（base64） */
    __JANUS_WASM_B64__?: string;
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
  const b64 = window.__JANUS_WASM_B64__;
  await init(b64 ? { module_or_path: bytesFromB64(b64) } : undefined);
  reset(2, 1);
  setupDrag();
  window.setInterval(frame, 100);
}

function reset(registers: number, mats: number): void {
  sim?.free();
  sim = new PosSimulator(Date.now() & 0xffff, registers, mats);
  t0 = performance.now();
  snap = JSON.parse(sim.snapshot()) as Snapshot;
  buildSkeleton();
  // 閉じたら（Esc でも）オーナーのパスワードを消す
  rulesDlg().addEventListener("close", () => {
    const pw = rulesDlg().querySelector<HTMLInputElement>(`[data-owner="pw"]`);
    if (pw) pw.value = "";
  });
  render();
}

// ---------------------------------------------------------------------------
// 骨組み（一度だけ作る）
// ---------------------------------------------------------------------------

function buildSkeleton(): void {
  const regs = snap.registers
    .map(
      (r) => `<section class="register" aria-label="レジ${r.id}">
      <h2>レジ${r.id} <span data-region="mode-${r.id}"></span>
        <label class="login-sel">店員
          <select data-login="${r.id}" aria-label="レジ${r.id}の店員ログイン">
            <option value="">未ログイン</option>
            ${snap.accounts.map((a) => `<option value="${esc(a.barcode)}">${esc(a.name)}</option>`).join("")}
          </select>
        </label>
      </h2>
      <div class="screens">
        <section class="screen staff-screen" data-drop="${r.id}:staff">
          <div class="screen-title">店員画面<span class="drop-hint">ここに落とす＝店員側スキャナーで読む</span></div>
          <div data-region="calls-${r.id}"></div>
          <div data-region="staff-${r.id}"></div>
          <div class="toast" id="toast-${r.id}-staff"></div>
        </section>
        <section class="screen customer-screen" data-drop="${r.id}:customer">
          <div class="screen-title">お客様画面<span class="drop-hint">ここに落とす＝お客様側スキャナーで読む</span></div>
          <button type="button" class="gear" data-gear="${r.id}" aria-label="レジ${r.id}の店舗メニュー" title="店舗メニュー（有人モードの店員用）" hidden>⚙</button>
          <div data-region="cust-${r.id}"></div>
          <div class="toast" id="toast-${r.id}-customer"></div>
        </section>
      </div>
      <div class="money-io" aria-label="レジ${r.id}の投入口と排出口（お客様側だけにあります）">
        <div class="money-io-cust">
        <div class="inlet"><span class="hw-label">投入口</span>
          <input type="number" inputmode="numeric" min="1" placeholder="金額" data-key="in-${r.id}" aria-label="レジ${r.id}の投入口に入れる金額" />
          <button data-form="insert" data-reg="${r.id}">入れる</button>
        </div>
        <button type="button" class="outlet" data-outlet="${r.id}" aria-label="レジ${r.id}の排出口（タッチで取り出す）"><span class="hw-label">排出口</span><span data-region="outlet-${r.id}"></span></button>
        </div>
      </div>
    </section>`,
    )
    .join("");

  const staffList = snap.accounts.map((a) => `${esc(a.name)}`).join("・");
  app.innerHTML = `
    <header class="top">
      <h1>Janus POS</h1>
      <p class="muted">右の売り場から商品をつかんで、レジの店員画面かお客様画面に落とすと、その側のスキャナーで読んだことになります。</p>
    </header>
    <article class="floor">
      <div class="left">
        ${regs}
        <section class="mat-area" aria-label="携帯情報端末">
          <h2>携帯情報端末（MAT）</h2>
          <div class="mat-row"><div class="mats" data-region="mats"></div><div class="mat-sim" data-region="sim"></div></div>
          <div class="toast" id="toast-store"></div>
        </section>
      </div>
      <aside class="right" aria-label="売り場">
        <h2>売り場 <button class="small ghost rules-open" data-open-rules aria-haspopup="dialog">年確ルール設定</button></h2>
        <div data-region="shelf"></div>
      </aside>
    </article>
    <footer class="bottom">
      <section>
        <h3>試し方</h3>
        <ol class="help">
          <li>運用モードは名札で決まります。名札なし＝セミセルフ（お客様が自分で登録・会計）、名札あり＝有人。セルフは将来の拡張用で、いまは OFF です。</li>
          <li>セミセルフ：商品をお客様画面に落として「お会計」。缶ビールなら年確の呼び出しで MAT が鳴ります。レジ名の右の「店員」でログインすると有人モードになり、「20歳以上です」が押せます。</li>
          <li>有人：「店員」で名前を選んでログインし、商品を店員画面に落として「会計」します。</li>
          <li>FF：セミセルフのお客様画面で「FF を注文」→ ログイン（有人）→ FF 登録 →「お渡し」→ お客様画面で「受け取りました」。</li>
          <li>宅急便・代行収納をお客様画面に落とすと係員の呼び出しになり、店員が金額を入れて受け付けます。</li>
          <li>有人モードは、取引のない状態で5分間操作がないとログアウトしてセミセルフに戻ります。残り時間はレジ名の横に出ます。</li>
          <li>お金は各レジ最下段の投入口に金額を入れて投入します。おつりや出金したお金は排出口に出て、タッチすると取り出せます。店舗メニュー：有人モード（ログイン中）だとお客様画面の右上に ⚙ が出ます。返品返金・緊急出金・経費支出は店舗パスワード <b>7777</b>（ログインとは別）、取引外入金と自動釣銭機の両替は投入口を使い、パスワード不要です。年確ルールは売り場の「年確ルール設定」から、オーナー 9001 / <b>9999</b> で商品ごとに変更できます。</li>
        </ol>
        <p class="muted">店員：${staffList}（各レジのレジ名の右にある「店員」で選ぶとログインします。「未ログイン」でログアウト）</p>
      </section>
      <details><summary>出金の記録</summary><div data-region="audit"></div></details>
      <p class="copyright"><a href="https://github.com/masamoto1982" target="_blank" rel="noopener">© 2026 masamoto yamashiro</a></p>
    </footer>
    <dialog class="rules-dlg" id="rules-dlg" aria-labelledby="rules-title">
      <div class="dlg-head"><h2 id="rules-title">年確ルール設定</h2><button class="small ghost" data-close-rules>閉じる</button></div>
      <p class="muted">変えられるのはオーナーだけです。変更のたびにここのパスワードで認証し、閉じるとパスワードは消えます。酒・たばこは法令どおり 20歳以上で固定され、変更できません。年確は会計ボタンを押した時点で、カゴの中で最も厳しい年齢で1回だけ行います。</p>
      <div class="row wrap">
        <input data-owner="id" placeholder="オーナーの従業員番号" value="9001" aria-label="オーナーの従業員番号" />
        <input data-owner="pw" type="password" placeholder="パスワード" autocomplete="off" aria-label="オーナーのパスワード" />
      </div>
      <div class="toast" id="toast-rules"></div>
      <div data-region="owner"></div>
    </dialog>
    <div class="drag-ghost" id="ghost" hidden></div>`;
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
  el.querySelectorAll<HTMLDetailsElement>("details[data-keep]").forEach((d, i) => d.open && open.add(i));
  const hadDetails = el.querySelectorAll("details[data-keep]").length > 0;

  el.innerHTML = html;

  el.querySelectorAll<HTMLInputElement | HTMLSelectElement>("[data-key]").forEach((i) => {
    const v = values.get(i.dataset.key!);
    if (v !== undefined) i.value = v;
  });
  if (hadDetails) el.querySelectorAll<HTMLDetailsElement>("details[data-keep]").forEach((d, i) => (d.open = open.has(i)));
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
    // 店員ログインのセレクタは、操作中でなければ現在のログイン状態に合わせる
    const sel = app.querySelector<HTMLSelectElement>(`[data-login="${r.id}"]`);
    if (sel && document.activeElement !== sel) {
      const acc = r.staff.state === "absent" ? undefined : snap.accounts.find((a) => a.name === r.staff.name);
      sel.value = acc?.barcode ?? "";
    }
    // 操作できない側の画面は表示のみ（排他）
    app.querySelector(`[data-drop="${r.id}:staff"]`)?.classList.toggle("inactive", r.active_side !== "staff");
    app.querySelector(`[data-drop="${r.id}:customer"]`)?.classList.toggle("inactive", r.active_side !== "customer");
    // 店舗メニューの歯車は、名札でログインした店員がいる（有人モード）ときだけ出す
    const gear = app.querySelector<HTMLButtonElement>(`[data-gear="${r.id}"]`);
    if (gear) gear.hidden = r.staff.state !== "ready";
    const idle = r.idle_revert_in;
    setRegion(
      `mode-${r.id}`,
      `<span class="mode mode-${r.mode}">${esc(r.mode_label)}</span>${
        idle !== null ? `<span class="idle" title="5分間操作がないとセミセルフに戻って待機します">無操作であと ${Math.floor(idle / 60000)}:${String(Math.floor((idle % 60000) / 1000)).padStart(2, "0")} でセミセルフに戻ります</span>` : ""
      }`,
    );
    setRegion(`calls-${r.id}`, staffCalls(r, snap.now));
    setRegion(`staff-${r.id}`, staffScreen(r, snap));
    setRegion(`cust-${r.id}`, customerScreen(r, snap.ff_stock));
    setRegion(
      `outlet-${r.id}`,
      r.outlet
        ? `<b>${r.outlet.toLocaleString()}円</b><small>${r.outlet_detail.map(([d, n]) => `${d.toLocaleString()}×${n}`).join(" ")}　タッチで取り出す</small>`
        : `<span class="muted">空</span>`,
    );
  }
  setRegion("mats", matPanel(snap));
  setRegion("sim", simPanel(snap));
  setRegion("owner", ownerPanel(snap));
  setRegion("audit", auditPanel(snap));
  setRegion("shelf", shelfPanel(snap));
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

function send(cmd: Command): boolean {
  sim.tick(performance.now() - t0);
  const res = JSON.parse(sim.dispatch(JSON.stringify(cmd))) as { ok: boolean; message?: string; error?: string };
  const side = sideOf(cmd);
  const where = cmd.type === "set_product_age" ? "toast-rules" : ["cash_in", "cash_out", "open_menu", "insert", "take_outlet"].includes(cmd.type) ? `toast-${cmd.register}-customer` : side === "store" ? "toast-store" : `toast-${cmd.register}-${side}`;
  if (res.ok) {
    if (res.message) toast(where, res.message, false);
  } else {
    toast(where, res.error ?? "操作できません", true);
  }
  frame();
  return res.ok;
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
    case "service":
      send({ type: "complete_service", register: reg, line_id: Number(b.dataset.line), amount: Number(val(`svc-${reg}-${b.dataset.line}`) || 0) });
      break;
    case "cashout":
      send({
        type: "cash_out",
        register: reg,
        kind: b.dataset.kind,
        amount: Number(val(`mo-amt-${reg}`) || 0),
        password: val(`mo-pw-${reg}`),
      });
      clear(`mo-pw-${reg}`);
      break;
    case "insert":
      if (send({ type: "insert", register: reg, amount: Number(val(`in-${reg}`) || 0) })) clear(`in-${reg}`);
      break;
  }
}

app.addEventListener("change", (e) => {
  const sel = e.target as HTMLSelectElement;
  if (sel.dataset.age) {
    const owner = (k: string) => app.querySelector<HTMLInputElement>(`[data-owner="${k}"]`)?.value.trim() ?? "";
    const ok = send({
      type: "set_product_age",
      owner_id: owner("id"),
      password: owner("pw"),
      code: sel.dataset.age,
      min_age: sel.value ? Number(sel.value) : null,
    });
    // 認められなかったら表示を元の設定に戻す
    if (!ok) for (const o of sel.options) o.selected = o.defaultSelected;
    return;
  }
  if (!sel.dataset.login) return;
  const register = Number(sel.dataset.login);
  if (sel.value) send({ type: "scan", register, side: "staff", code: sel.value });
  else send({ type: "end_staff_mode", register });
  sel.blur();
});

const rulesDlg = (): HTMLDialogElement => document.getElementById("rules-dlg") as HTMLDialogElement;

app.addEventListener("click", (e) => {
  const b = (e.target as Element).closest("button");
  if (!b || b.disabled) return;
  if (b.hasAttribute("data-open-rules")) return rulesDlg().showModal();
  if (b.hasAttribute("data-close-rules")) return rulesDlg().close();
  if (b.dataset.gear) {
    const reg = Number(b.dataset.gear);
    const open = snap.registers.find((x) => x.id === reg)?.menu;
    send({ type: "open_menu", register: reg, screen: open ? null : "top" });
    return;
  }
  if (b.dataset.outlet) {
    send({ type: "take_outlet", register: Number(b.dataset.outlet) });
    return;
  }
  if (b.dataset.cmd) send(JSON.parse(b.dataset.cmd) as Command);
  else if (b.dataset.form) form(b);
});

// ---------------------------------------------------------------------------
// ドラッグ＆ドロップ（ポインターイベントで実装。マウスでもタッチでも同じに動く）
// ---------------------------------------------------------------------------

function setupDrag(): void {
  const ghost = document.getElementById("ghost")!;
  let drag: { code: string; label: string; x: number; y: number; moved: boolean; id: number } | null = null;
  let over: Element | null = null;

  const target = (x: number, y: number): Element | null => document.elementFromPoint(x, y)?.closest("[data-drop]") ?? null;
  const highlight = (el: Element | null) => {
    if (over === el) return;
    over?.classList.remove("drop-over");
    el?.classList.add("drop-over");
    over = el;
  };
  const end = () => {
    drag = null;
    ghost.hidden = true;
    highlight(null);
    document.body.classList.remove("dragging");
  };

  app.addEventListener("pointerdown", (e) => {
    const b = (e.target as Element).closest<HTMLElement>("[data-code]");
    if (!b || e.button !== 0) return;
    e.preventDefault();
    drag = { code: b.dataset.code!, label: b.querySelector(".item-name")?.textContent ?? "", x: e.clientX, y: e.clientY, moved: false, id: e.pointerId };
  });

  window.addEventListener("pointermove", (e) => {
    if (!drag || e.pointerId !== drag.id) return;
    if (!drag.moved && Math.hypot(e.clientX - drag.x, e.clientY - drag.y) < 5) return;
    if (!drag.moved) {
      drag.moved = true;
      ghost.textContent = drag.label;
      ghost.hidden = false;
      document.body.classList.add("dragging");
    }
    ghost.style.transform = `translate(${e.clientX + 8}px, ${e.clientY + 8}px)`;
    highlight(target(e.clientX, e.clientY));
  });

  window.addEventListener("pointerup", (e) => {
    if (!drag || e.pointerId !== drag.id) return;
    const d = drag;
    const t = target(e.clientX, e.clientY);
    end();
    if (!d.moved) {
      toast("toast-store", "商品はドラッグして、レジの店員画面かお客様画面に落としてください", false);
      return;
    }
    if (!t) return;
    const [reg, side] = (t as HTMLElement).dataset.drop!.split(":");
    send({ type: "scan", register: Number(reg), side, code: d.code });
  });

  window.addEventListener("pointercancel", end);
}

boot().catch((e) => {
  app.innerHTML = `<p class="error">読み込みに失敗しました：${esc(e)}</p>`;
});
