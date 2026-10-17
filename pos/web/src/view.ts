// 画面の描画。どれもスナップショットから HTML 文字列を作る純粋関数で、
// ボタンの出し分けはコアが返す actions（操作可否と理由）だけで決める。
// 画面側で「押せるかどうか」を独自に判断しない。

import type {
  ActionState,
  Command,
  CustomerView,
  FfState,
  MenuScreen,
  RegisterView,
  Snapshot,
  StaffLine,
  StaffView,
} from "./types";

export function esc(s: unknown): string {
  return String(s ?? "")
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

export const yen = (n: number): string => `${n.toLocaleString("ja-JP")}円`;

const secs = (now: number, at: number): string => `${Math.max(0, Math.floor((now - at) / 1000))}秒`;

function find(actions: ActionState[], name: string): ActionState {
  return actions.find((a) => a.action === name) ?? { action: name, allowed: false, reason: null };
}

/** コマンドを持つボタン。押せないときは理由を title に出す */
export function btn(label: string, cmd: Command, a?: ActionState, cls = ""): string {
  const disabled = a && !a.allowed;
  const title = disabled && a.reason ? ` title="${esc(a.reason)}"` : "";
  return `<button class="${cls}" data-cmd="${esc(JSON.stringify(cmd))}"${disabled ? " disabled" : ""}${title}>${esc(label)}</button>`;
}

const FF_LABEL: Record<FfState, string> = {
  waiting: "お渡し前",
  received: "お渡し済",
};

/**
 * FF の明細の個数の右に置く、お渡し確認のチェックボックス。
 * 全部にチェックが入るまで会計に進めない。どちらの画面から押したかは、押した画面で決まる
 */
function ffCheck(reg: number, l: { id: number; kind: string; state?: FfState }, a: ActionState): string {
  if (l.kind !== "ff") return "";
  const title = a.allowed ? "お渡ししたらチェック" : (a.reason ?? "");
  return `<label class="ffcheck" title="${esc(title)}"><input type="checkbox" data-ffcheck="${reg}:${l.id}"${l.state === "received" ? " checked" : ""}${a.allowed ? "" : " disabled"} aria-label="お渡し確認" /></label>`;
}

const METHOD_LABEL: Record<string, string> = { cash: "現金", emoney: "電子マネー", qr: "バーコード決済", credit: "クレジットカード" };
const METHODS = ["cash", "emoney", "qr", "credit"] as const;

// ---------------------------------------------------------------------------
// 店員画面
// ---------------------------------------------------------------------------

function staffLine(reg: number, l: StaffLine, v: StaffView): string {
  const tags: string[] = [];
  if (l.age) tags.push(`<span class="tag age">年確${l.age}</span>`);
  if (l.kind === "ff") tags.push(`<span class="tag ff-${l.state}">FF ${FF_LABEL[l.state]}</span>`);
  if (l.kind === "service") tags.push(`<span class="tag ${l.done ? "ok" : "warn"}">${l.done ? "受付済" : "受付待ち"}</span>`);
  const remove = find(v.actions, "remove_line");
  let svc = "";
  if (l.kind === "service" && !l.done) {
    const a = find(v.actions, "complete_service");
    svc = `<div class="svc">
      <input type="number" inputmode="numeric" min="0" placeholder="金額" data-key="svc-${reg}-${l.id}" ${a.allowed ? "" : "disabled"} />
      <button data-form="service" data-reg="${reg}" data-line="${l.id}" ${a.allowed ? "" : "disabled"}>受付</button>
    </div>`;
  }
  return `<tr>
    <td>${esc(l.name)} ${tags.join(" ")}${svc}</td>
    <td class="num">${l.qty}${ffCheck(reg, l, find(v.actions, "check_ff"))}</td>
    <td class="num">${l.kind === "service" && !l.done ? "—" : yen(l.amount)}</td>
    <td>${btn("取消", { type: "remove_line", register: reg, side: "staff", line_id: l.id }, remove, "small ghost")}</td>
  </tr>`;
}

export function staffCalls(r: RegisterView, now: number): string {
  const v = r.staff;
  const warn = v.mat_down.length
    ? `<div class="banner danger">${esc(v.mat_down.join("・"))} 不通：呼び出しが届きません</div>`
    : "";
  const calls = v.calls
    .map(
      (c) =>
        `<div class="banner call">🔔 ${esc(c.label)}${c.detail ? `（${esc(c.detail)}）` : ""}<span class="muted"> ${secs(now, c.created_at)}経過</span></div>`,
    )
    .join("");
  return warn + calls;
}

export function staffScreen(r: RegisterView): string {
  const v = r.staff;
  const reg = r.id;
  const a = (n: string) => find(v.actions, n);
  // 新しく登録した商品ほど上に出す
  const rows = [...v.lines].reverse().map((l) => staffLine(reg, l, v)).join("");
  const paying = v.phase === "paying";

  // 会計ボタンの後は、まずポイントカードの最終確認。済むと支払方法を選べる
  const pay = paying && v.point_pending
    ? pointCheck(reg, "staff", a("confirm_point_card"))
    : paying
    ? `<div class="paybox">
        <div>年確：${v.required_age ? (v.age_pending ? `<b class="warn">${v.required_age}歳以上の確認待ち（お客様画面）</b>` : `<span class="ok">確認済</span>`) : "不要"}</div>
        <div>支払方法：${v.method ? METHOD_LABEL[v.method] : "未選択"}　投入 ${yen(v.inserted)}　残り <b>${yen(v.due)}</b></div>
        ${v.payment_blocker ? `<div class="warn">決済できません：${esc(v.payment_blocker)}</div>` : ""}
        <div class="row wrap">${METHODS
          .map((m) => btn(METHOD_LABEL[m], { type: "select_payment", register: reg, side: "staff", method: m }, a("select_payment"), `seg ${v.method === m ? "on" : ""}`))
          .join("")}</div>
      </div>`
    : "";

  // 合計と会計ボタンは登録画面の最上部に常に出す（明細が増えてもスクロールで隠れない）
  const top = regTop(
    v.total,
    paying
      ? btn("登録に戻る", { type: "back_to_registering", register: reg, side: "staff" }, a("back_to_registering"))
      : btn("会計", { type: "checkout", register: reg, side: "staff" }, a("checkout"), "primary"),
  );
  return `${top}
    <div class="staff-head">
      <span>店員：<b>${esc(v.name)}</b>${v.role === "owner" ? "（オーナー）" : ""}</span>
      <span class="muted">内税 8% ${yen(v.tax8)} / 10% ${yen(v.tax10)}　${v.point_card ? "ポイントカード 読み取り済" : "ポイントカードなし"}</span>
    </div>
    ${pay}
    <table class="lines"><thead><tr><th>品名</th><th>数</th><th>金額</th><th></th></tr></thead>
      <tbody>${rows || `<tr><td colspan="4" class="muted">明細はありません。どのバーコードからでも読めます</td></tr>`}</tbody></table>
    <div class="row">
      ${btn("取引中止", { type: "void_transaction", register: reg }, a("void_transaction"), "ghost")}
      ${btn("ロック", { type: "lock", register: reg }, a("lock"), "ghost")}
      ${btn("ログアウト", { type: "end_staff_mode", register: reg }, a("end_staff_mode"), "ghost")}
    </div>`;
}

/** 会計ボタンの後のポイントカードの最終確認。ポイントカードを読むか「なしで進む」で支払方法へ */
function pointCheck(reg: number, side: "staff" | "customer", st: ActionState): string {
  return `<div class="card point-card">
      <div class="big">ポイントカードはお持ちですか？</div>
      <p>お持ちの方は、ポイントカードを読み取ってください。読み取るとお支払い方法の選択に進みます。</p>
      ${btn("ポイントカードなしで進む", { type: "confirm_point_card", register: reg, side }, st, "primary")}
    </div>`;
}

/** 登録画面の最上部に固定する帯：合計金額と会計ボタン */
function regTop(total: number, action: string): string {
  return `<div class="reg-top"><span>合計 <b class="total">${yen(total)}</b></span>${action}</div>`;
}

/**
 * 店員画面とお客様画面は、まったく同じ内容を映す（情報空間は一つ）。
 * 内容は「いま操作している側」の画面で、もう一方は同じものをグレーで表示するだけ。
 */
export function sharedScreen(r: RegisterView, snap: Snapshot): string {
  const body = r.active_side === "staff" ? staffScreen(r) : customerScreen(r);
  return staffCalls(r, snap.now) + body;
}

// ---------------------------------------------------------------------------
// レジ画面のヘッダー：左上に歯車（店員ログイン中だけ）、中央に日付と時刻、右上に呼び出しベル。
// 時刻とベルは常に表示する。どちらが店員側・お客様側かは設置で自明なので画面には出さない
// ---------------------------------------------------------------------------

const pad = (n: number) => String(n).padStart(2, "0");

export function clockText(d: Date): string {
  return `${d.getFullYear()}年${d.getMonth() + 1}月${d.getDate()}日 ${pad(d.getHours())}:${pad(d.getMinutes())}`;
}

export function screenHead(r: RegisterView, now: Date): string {
  const call = find(r.customer.actions, "call_staff");
  const bell = `<button type="button" class="bell-btn" data-hold="${esc(JSON.stringify({ type: "call_staff", register: r.id }))}" aria-label="係員を呼ぶ（押している間鳴ります）" title="${esc(call.allowed ? "係員を呼ぶ（押している間鳴ります）" : (call.reason ?? ""))}"${call.allowed ? "" : " disabled"}>🔔</button>`;
  const gear = `<button type="button" class="gear" data-gear="${r.id}" aria-label="レジ${r.id}の店舗メニュー" title="店舗メニュー"${r.staff.state === "ready" ? "" : " hidden"}>⚙</button>`;
  return `<span class="head-l">${gear}</span><time class="clock">${clockText(now)}</time><span class="head-r">${bell}</span>`;
}

// ---------------------------------------------------------------------------
// バーコードなしアイテム欄（レジ画面の下 1/3）
// - セミセルフ：店員呼び出しベルのショートカット（FF・サービス・たばこ）。触れている間だけ鳴る
// - 有人・セルフ：FF をタブ（揚げ物・常温総菜・ドリンク・コールド・中華まん・おでん）で選ぶ。
//   各タブはページを持ち、売れ筋順に並ぶので先頭ページの最上部に売れ筋が来る
// ---------------------------------------------------------------------------

export const FF_TABS = [
  ["fried", "揚げ物"],
  ["deli", "常温総菜"],
  ["drink", "ドリンク"],
  ["cold", "コールド"],
  ["bun", "中華まん"],
  ["oden", "おでん"],
] as const;
export const FF_PER_PAGE = 8;
export type FfUi = { tab: string; page: number };

export function itemsPanel(r: RegisterView, snap: Snapshot, ui: FfUi): string {
  const reg = r.id;
  if (r.menu) return "";
  if (r.mode === "semi_self") {
    const a = (n: string) => find(r.customer.actions, n);
    const callBtn = (main: string, sub: string, cmd: Command, st: ActionState, cls: string) =>
      `<button class="call-btn ${cls}" data-hold="${esc(JSON.stringify(cmd))}"${st.allowed ? "" : ` disabled title="${esc(st.reason ?? "")}"`}><b>${esc(main)}</b>${sub ? `<small>${esc(sub)}</small>` : ""}</button>`;
    return `<div class="call-row">${[
      callBtn("FF", "揚げ物・中華まん", { type: "ff_button", register: reg }, a("ff_button"), "ff"),
      callBtn("サービス", "代行収納・宅配便", { type: "call_staff", register: reg, topic: "service" }, a("call_staff"), "svc"),
      callBtn("たばこ", "", { type: "call_staff", register: reg, topic: "tobacco" }, a("call_staff"), "tob"),
    ].join("")}</div>`;
  }
  // 有人モードは店員・お客様どちらの画面からも登録できる。セルフはお客様が登録する
  const side = r.mode === "attended" ? r.active_side : "customer";
  const view = side === "staff" ? r.staff : r.customer;
  const can = find(view.actions, "register_ff");
  const items = snap.ff_stock.filter((f) => f.tab === ui.tab);
  const pages = Math.max(1, Math.ceil(items.length / FF_PER_PAGE));
  const page = Math.min(ui.page, pages - 1);
  const tabs = FF_TABS.map(
    ([k, label]) => `<button type="button" class="ff-tab ${k === ui.tab ? "on" : ""}" data-fftab="${reg}:${k}" aria-pressed="${k === ui.tab}">${label}</button>`,
  ).join("");
  const grid = items
    .slice(page * FF_PER_PAGE, (page + 1) * FF_PER_PAGE)
    .map((f) =>
      btn(
        `${f.name}`,
        { type: "register_ff", register: reg, side, code: f.code, qty: 1 },
        f.stock > 0 ? can : { action: "register_ff", allowed: false, reason: "売り切れです" },
        `ff-item ${f.stock > 0 ? "" : "soldout"}`,
      ).replace("</button>", `<small>${f.stock > 0 ? (r.mode === "attended" ? `残${f.stock}` : yen(f.price)) : "売り切れ"}</small></button>`),
    )
    .join("");
  // ページ送りの上下矢印はページの右端。上（前）・下（次）に隠れたページがあるときだけ出す
  const arrows =
    pages > 1
      ? `<div class="ff-arrows">${
          page > 0 ? `<button type="button" class="ff-arrow" data-ffpage="${reg}:-1" aria-label="前のページ">▲</button>` : "<span></span>"
        }${page < pages - 1 ? `<button type="button" class="ff-arrow" data-ffpage="${reg}:1" aria-label="次のページ">▼</button>` : "<span></span>"}</div>`
      : "";
  return `<div class="ff-tabs" role="tablist">${tabs}</div><div class="ff-page"><div class="ff-grid">${grid}</div>${arrows}</div>`;
}

// ---------------------------------------------------------------------------
// 店舗メニュー（お客様画面の右上の歯車から。有人モードの店員だけが使う）
// 自動釣銭機の取り出し口は売り場側にあるので、入出金の操作もお客様画面側で行う
// ---------------------------------------------------------------------------

const MENU_LABEL: Record<MenuScreen, string> = {
  top: "メニュー",
  refund: "返品返金",
  emergency: "緊急出金",
  deposit: "取引外入金",
  expense: "経費支出",
  changer: "自動釣銭機（両替）",
};

/** 一度に一つの画面だけを出す：一覧 → 各項目の画面 */
export function storeMenu(r: RegisterView): string {
  const v = r.staff;
  const reg = r.id;
  const m = r.menu!;
  const go = (screen: MenuScreen | null, extra: Record<string, unknown> = {}) => ({ type: "open_menu", register: reg, screen, ...extra });
  const head = (title: string) =>
    `<div class="menu-head">${btn("‹ 戻る", go("top"), undefined, "small ghost")}<b>${esc(title)}</b></div>`;
  switch (m.screen) {
    case "top":
      return `<div class="menu-head"><b>店舗メニュー</b><span class="muted">担当：${esc(v.name ?? "")}</span>${btn("閉じる", go(null), undefined, "small ghost")}</div>
        <div class="menu-list">${(["refund", "emergency", "deposit", "expense", "changer"] as const)
          .map((k) => btn(MENU_LABEL[k], go(k), undefined, "menu-item"))
          .join("")}</div>`;
    case "refund":
    case "emergency":
    case "expense": {
      const cout = find(v.actions, "cash_out");
      return `${head(MENU_LABEL[m.screen])}
        <p class="muted">お金を抜く操作です。店舗パスワードが毎回必要で、お金は下の「出金」ボックスに出ます。</p>
        <div class="menu-form">
          <input type="number" inputmode="numeric" min="1" placeholder="金額" data-key="mo-amt-${reg}" />
          <input type="password" inputmode="numeric" placeholder="店舗パスワード" data-key="mo-pw-${reg}" autocomplete="off" />
          <button class="primary" data-form="cashout" data-reg="${reg}" data-kind="${m.screen}" ${cout.allowed ? "" : `disabled title="${esc(cout.reason ?? "")}"`}>出金</button>
        </div>`;
    }
    case "deposit":
      return `${head(MENU_LABEL.deposit)}
        <p>入金する金額を下の<b>「入金」ボックス</b>に入れて Enter を押してください。</p>
        <p class="muted">お金を入れる操作なのでパスワードは要りません。記録は担当者名つきで残ります。</p>`;
    case "changer": {
      const have = (d: number) => v.drawer.find(([x]) => x === d)?.[1] ?? 0;
      const segs = [1000, 500, 100, 50, 10]
        .map((d) => btn(`${d.toLocaleString()}円`, go("changer", { exchange_to: d }), undefined, `seg small ${m.exchange_to === d ? "on" : ""}`))
        .join("");
      return `${head(MENU_LABEL.changer)}
        <div class="row wrap">受け取る金種 ${segs}</div>
        <p>両替する金額を<b>「入金」ボックス</b>に入れて Enter を押してください。同じ額を ${m.exchange_to.toLocaleString()}円 で<b>「出金」ボックス</b>に出します。</p>
        <p class="muted">釣銭機の ${m.exchange_to.toLocaleString()}円：残り ${have(m.exchange_to)} 枚　在高 ${yen(v.drawer_total)}</p>`;
    }
  }
}

// ---------------------------------------------------------------------------
// お客様画面（公開してよい情報だけを使う）
// ---------------------------------------------------------------------------

export function customerScreen(r: RegisterView): string {
  // 店員が店舗メニューを開いている間は、お客様画面にメニューを出す
  if (r.menu) return storeMenu(r);
  const c: CustomerView = r.customer;
  const reg = r.id;
  const a = (n: string) => find(c.actions, n);
  // セミセルフ（名札なし）とセルフは、お客様が自分で登録・会計する
  const self = r.mode !== "attended";
  if (r.last_receipt && !c.lines.length) {
    const rc = r.last_receipt;
    const change = rc.change.map(([d, n]) => `${d.toLocaleString()}円×${n}`).join("　");
    return `<div class="thanks">
      <div class="big">ありがとうございました</div>
      <div>お支払い ${yen(rc.total)}（${esc(rc.method)}）</div>
      ${rc.change_total ? `<div class="change">おつり <b>${yen(rc.change_total)}</b><div class="muted">${change}</div><div class="muted">紙幣と硬貨はそろえてお返しします</div></div>` : ""}
      ${rc.point_card ? `<div>ポイント ${rc.points}pt を付与しました</div>` : ""}
    </div>`;
  }

  const banners: string[] = [];
  if (c.staff_called) banners.push(`<div class="banner call">係員をお呼びしています。しばらくお待ちください</div>`);
  else if (r.active_side === "staff" && c.staff_present) banners.push(`<div class="banner watch">係員が操作中です。しばらくお待ちください</div>`);

  // 新しく登録した商品ほど上に出す
  const rows = [...c.lines]
    .reverse()
    .map((l) => {
      const tag =
        l.kind === "ff"
          ? `<span class="tag ff-${l.state}">${{ waiting: "ご用意中", received: "お受け取り済" }[l.state]}</span>`
          : l.kind === "service" && !l.done
            ? `<span class="tag warn">係員が受付します</span>`
            : "";
      const rm = self && c.phase === "registering" ? btn("取消", { type: "remove_line", register: reg, side: "customer", line_id: l.id }, a("remove_line"), "small ghost") : "";
      return `<tr><td>${esc(l.name)} ${tag}</td><td class="num">${l.qty}${ffCheck(reg, l, a("check_ff"))}</td><td class="num">${l.kind === "service" && !l.done ? "—" : yen(l.amount)}</td><td>${rm}</td></tr>`;
    })
    .join("");

  if (!c.lines.length) {
    const empty = regTop(0, btn("お会計", { type: "checkout", register: reg, side: "customer" }, a("checkout"), "primary"));
    return `${empty}${banners.join("")}<div class="welcome">
      <div class="big">いらっしゃいませ</div>
      <p>${self ? "商品のバーコードをスキャンしてください" : "係員が商品をお読みします"}</p>
    </div>`;
  }

  let pay = "";
  if (c.phase === "paying") {
    const age =
      c.age_check !== null
        ? `<div class="card age-card">
            <div class="big">年齢確認</div>
            <p>年齢確認が必要な商品があります</p>
            ${btn(`${c.age_check}歳以上です`, { type: "confirm_age", register: reg }, a("confirm_age"), "primary big-btn")}
            ${!a("confirm_age").allowed && a("confirm_age").reason ? `<p class="muted">${esc(a("confirm_age").reason)}</p>` : ""}
          </div>`
        : "";
    const methods = a("select_payment").allowed
      ? `<div class="row center wrap">${METHODS
          .map((m) => btn(METHOD_LABEL[m], { type: "select_payment", register: reg, side: "customer", method: m }, a("select_payment"), `seg ${c.method === m ? "on" : ""}`))
          .join("")}</div>`
      : "";
    const hint = c.payment_blocker
      ? `<p class="muted">${esc(c.payment_blocker)}</p>`
      : c.method && c.method !== "cash"
        ? `<p>財布の${METHOD_LABEL[c.method]}を、ここ（お客様側の端末）にかざしてください</p>`
        : `<p>現金を投入してください${r.mode === "attended" ? "" : "（または支払方法を選んでください）"}</p>`;
    pay = c.point_pending ? `${pointCheck(reg, "customer", a("confirm_point_card"))}${age}` : `${age}<div class="paybox center">
        <div>お支払い <b class="total">${yen(c.total)}</b></div>
        ${c.inserted ? `<div>お預かり ${yen(c.inserted)}　残り <b>${yen(c.due)}</b></div>` : ""}
        ${methods}${hint}
      </div>`;
  }

  // 合計とお会計ボタンは登録画面の最上部に常に出す（有人モードでは係員がお客様画面を持っているときだけ押せる）
  const top = regTop(
    c.total,
    c.phase === "registering"
      ? btn("お会計", { type: "checkout", register: reg, side: "customer" }, a("checkout"), "primary")
      : btn("商品登録に戻る", { type: "back_to_registering", register: reg, side: "customer" }, a("back_to_registering"), "ghost"),
  );

  return `${top}${banners.join("")}
    ${pay}
    <table class="lines"><tbody>${rows}</tbody></table>
    ${c.point_card ? `<div class="totals"><span class="muted">ポイントカード 読み取り済</span></div>` : ""}`;
}

// ---------------------------------------------------------------------------
// MAT・店舗の設定
// ---------------------------------------------------------------------------

export function matPanel(s: Snapshot): string {
  return s.mats
    .map(
      (m) => `<div class="mat ${m.buzzing ? "buzz" : ""}">
        <span class="bell">🔔</span><span class="mat-id">${esc(m.id)}</span><span class="mat-state">${m.buzzing ? "鳴動中" : "待機"}</span>
      </div>`,
    )
    .join("");
}

const AGE_CHOICES = [15, 18, 20];

/** 年確ルール：売り場のすべての商品に、商品ごとの店舗ルールを付けられる */
export function ownerPanel(s: Snapshot): string {
  const rows = SHELVES.map((g) => {
    const items = s.catalog.filter(g.match);
    if (!items.length) return "";
    const body = items
      .map((p) => {
        // 酒・たばこは法令どおりで固定。店舗ルールは付けられない
        if (p.legal_age)
          return `<tr><td>${esc(p.name)}</td><td><span class="tag">法令 ${p.legal_age}歳</span></td>
          <td><span class="muted">変更できません</span></td>
          <td class="num"><b>${p.legal_age}歳以上</b></td></tr>`;
        const ages = AGE_CHOICES.slice();
        if (p.store_age && !ages.includes(p.store_age)) ages.push(p.store_age);
        ages.sort((x, y) => x - y);
        const opts = [`<option value="" ${p.store_age ? "" : "selected"}>なし</option>`]
          .concat(ages.map((a) => `<option value="${a}" ${p.store_age === a ? "selected" : ""}>${a}歳以上</option>`))
          .join("");
        return `<tr><td>${esc(p.name)}</td>
          <td><span class="muted">—</span></td>
          <td><select data-age="${esc(p.code)}" aria-label="${esc(p.name)}の店舗ルール">${opts}</select></td>
          <td class="num">${p.age ? `<b>${p.age}歳以上</b>` : `<span class="muted">年確なし</span>`}</td></tr>`;
      })
      .join("");
    return `<tr class="grp"><th colspan="4">${esc(g.label)}</th></tr>${body}`;
  }).join("");
  const hist = s.age_rules.history.length
    ? s.age_rules.history.map((h) => `<li>${esc(h.by)}：${esc(h.change)}</li>`).join("")
    : `<li class="muted">変更はまだありません</li>`;
  return `<table class="lines rules"><thead><tr><th>商品</th><th>法令</th><th>店舗ルール</th><th class="num">会計時の年確</th></tr></thead>
    <tbody>${rows}</tbody></table>
    <h4>変更履歴</h4><ul>${hist}</ul>`;
}

// ---------------------------------------------------------------------------
// ハードウェア（スキャナー・現金・電子マネー端末）
// ---------------------------------------------------------------------------

type Shelf = { label: string; match: (p: Snapshot["catalog"][number]) => boolean };

/** 売り場の棚。並び順はご指示の順（食べ物・飲み物・新聞・雑誌・代行収納・宅急便・酒・ライター）に、そのほかを足したもの */
const SHELVES: Shelf[] = [
  { label: "食べ物", match: (p) => p.category === "food" },
  { label: "飲み物", match: (p) => p.category === "drink" },
  { label: "新聞", match: (p) => p.category === "newspaper" },
  { label: "雑誌", match: (p) => p.category === "magazine" || p.category === "adult_book" },
  { label: "代行収納", match: (p) => p.category === "service" && p.name.includes("代行収納") },
  { label: "宅急便", match: (p) => p.category === "service" && p.name.includes("宅急便") },
  { label: "酒", match: (p) => p.category === "alcohol" },
  { label: "ライター・たばこ", match: (p) => p.category === "smoking_goods" || p.category === "tobacco" },
  { label: "花火", match: (p) => p.category === "fireworks" },
  { label: "そのほか", match: (p) => ["daily", "lottery", "adult_goods", "bag"].includes(p.category) },
];

function item(code: string, name: string, sub: string, cls = ""): string {
  return `<button type="button" class="item ${cls}" data-code="${esc(code)}" aria-label="${esc(name)}をドラッグ">
    <span class="item-name">${esc(name)}</span><span class="item-sub">${sub}</span></button>`;
}

export function shelfPanel(s: Snapshot): string {
  const groups = SHELVES.map((g) => {
    const items = s.catalog.filter(g.match);
    if (!items.length) return "";
    return `<div class="shelf"><h3>${esc(g.label)}</h3><div class="items">${items
      .map((p) => item(p.code, p.name, `${p.price ? yen(p.price) : "金額は受付時"}${p.age ? ` ・<b>年確${p.age}</b>` : ""}`))
      .join("")}</div></div>`;
  }).join("");
  return groups;
}

// ---------------------------------------------------------------------------
// 財布（お客様の持ち物）。レジに落とすと、お金は投入口へ、カードは読み取り・端末へ
// 実在のサービス名は使わない
// ---------------------------------------------------------------------------

function walletItem(key: string, name: string, sub: string, cls: string): string {
  return `<button type="button" class="item ${cls}" data-wallet="${esc(key)}" aria-label="${esc(name)}をドラッグ">
    <span class="item-name">${esc(name)}</span><span class="item-sub">${sub}</span></button>`;
}

export function walletPanel(): string {
  const bills = [10000, 5000, 2000, 1000].map((d) => walletItem(`cash:${d}`, `${d.toLocaleString()}円`, "紙幣", "bill"));
  const coins = [500, 100, 50, 10, 5, 1].map((d) => walletItem(`cash:${d}`, `${d}円`, "硬貨", "coin"));
  const cards = [
    item("PC12345678", "ポイントカード", "レジで読む", "card"),
    walletItem("pay:qr", "バーコード決済", "スマホ", "pay"),
    walletItem("pay:credit", "クレジットカード", "端末にかざす", "pay"),
    walletItem("pay:emoney", "電子マネー", "端末にかざす", "pay"),
  ];
  const group = (label: string, xs: string[]) => `<div class="shelf"><h3>${label}</h3><div class="items">${xs.join("")}</div></div>`;
  return group("紙幣", bills) + group("硬貨", coins) + group("カード・スマホ", cards);
}

/** コマンドがどちらの画面からの操作か（お知らせを出す場所を決める） */
export function sideOf(cmd: Command): "staff" | "customer" | "store" {
  if (cmd.side === "staff" || cmd.side === "customer") return cmd.side;
  if (["ff_button", "confirm_age", "insert_cash", "tap_emoney", "call_staff"].includes(cmd.type)) return "customer";
  if (typeof cmd.register === "number") return "staff";
  return "store";
}
