// 画面の描画。どれもスナップショットから HTML 文字列を作る純粋関数で、
// ボタンの出し分けはコアが返す actions（操作可否と理由）だけで決める。
// 画面側で「押せるかどうか」を独自に判断しない。

import type {
  ActionState,
  Command,
  CustomerView,
  FfState,
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
  presented: "お渡し中",
  received: "受取済",
};

const METHOD_LABEL: Record<string, string> = { cash: "現金", emoney: "電子マネー", qr: "QR決済" };

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
    <td class="num">${l.qty}</td>
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

export function staffScreen(r: RegisterView, snap: Snapshot): string {
  const v = r.staff;
  const reg = r.id;
  const a = (n: string) => find(v.actions, n);
  if (v.state !== "ready") {
    return `<div class="locked">
      <div class="big">${v.state === "locked" ? "🔒 ロック中" : "店員未ログイン"}</div>
      <p>名札（スタッフバーコード）をこの画面に落とすと${v.state === "locked" ? "解除" : "ログイン"}します</p>
      ${r.mode === "full_self" ? `<p class="muted">セルフモード：お客様の操作は続いています。右の「お客様画面」と同じ取引です。</p>` : ""}
      <div class="mirror">${miniLines(r)}</div>
    </div>`;
  }

  const rows = v.lines.map((l) => staffLine(reg, l, v)).join("");
  const paying = v.phase === "paying";
  const modeSel = (["attended", "semi_self", "full_self"] as const)
    .map((m) =>
      btn(
        { attended: "有人", semi_self: "セミセルフ", full_self: "セルフ" }[m],
        { type: "set_mode", register: reg, mode: m },
        a("set_mode"),
        `seg ${r.mode === m ? "on" : ""}`,
      ),
    )
    .join("");

  const ffMenu = snap.ff_stock
    .map((f) =>
      btn(
        `${f.name}（残${f.stock}）`,
        { type: "register_ff", register: reg, code: f.code, qty: 1 },
        f.stock > 0 ? a("register_ff") : { action: "register_ff", allowed: false, reason: "在庫がありません" },
        "small ff",
      ),
    )
    .join("");

  const pay = paying
    ? `<div class="paybox">
        <div>年確：${v.required_age ? (v.age_pending ? `<b class="warn">${v.required_age}歳以上の確認待ち（お客様画面）</b>` : `<span class="ok">確認済</span>`) : "不要"}</div>
        <div>支払方法：${v.method ? METHOD_LABEL[v.method] : "未選択"}　投入 ${yen(v.inserted)}　残り <b>${yen(v.due)}</b></div>
        ${v.payment_blocker ? `<div class="warn">決済できません：${esc(v.payment_blocker)}</div>` : ""}
        <div class="row">${(["cash", "emoney", "qr"] as const)
          .map((m) => btn(METHOD_LABEL[m], { type: "select_payment", register: reg, side: "staff", method: m }, a("select_payment"), `seg ${v.method === m ? "on" : ""}`))
          .join("")}</div>
      </div>`
    : "";

  const watching =
    r.active_side === "customer"
      ? `<div class="banner watch">お客様が操作中です（この画面は表示のみ）${v.age_pending ? "：年齢確認の待ち" : v.lines.some((l) => l.kind === "ff" && l.state === "presented") ? "：FF の受取確認の待ち" : "：お支払い中"}。商品のバーコードは読めます</div>`
      : "";
  return `${watching}<div class="staff-head">
      <span>店員：<b>${esc(v.name)}</b>${v.role === "owner" ? "（オーナー）" : ""}</span>
      <span class="segs">${modeSel}</span>
    </div>
    <table class="lines"><thead><tr><th>品名</th><th>数</th><th>金額</th><th></th></tr></thead>
      <tbody>${rows || `<tr><td colspan="4" class="muted">明細はありません。どのバーコードからでも読めます</td></tr>`}</tbody></table>
    <div class="totals">
      <span>合計 <b class="total">${yen(v.total)}</b></span>
      <span class="muted">内税 8% ${yen(v.tax8)} / 10% ${yen(v.tax10)}</span>
      <span class="muted">${v.point_card ? `ポイントカード ${esc(v.point_card)}` : "ポイントカードなし"}</span>
    </div>
    ${pay}
    <div class="row">
      ${paying ? btn("登録に戻る", { type: "back_to_registering", register: reg, side: "staff" }, a("back_to_registering")) : btn("会計", { type: "checkout", register: reg, side: "staff" }, a("checkout"), "primary")}
      ${btn("お渡し", { type: "hand_over_ff", register: reg }, a("hand_over_ff"), "ff-hand")}
      ${btn("取引中止", { type: "void_transaction", register: reg }, a("void_transaction"), "ghost")}
      ${btn("ロック", { type: "lock", register: reg }, a("lock"), "ghost")}
      ${btn("ログアウト", { type: "end_staff_mode", register: reg }, a("end_staff_mode"), "ghost")}
    </div>
    <div class="ffmenu"><span class="muted">FF 登録</span>${ffMenu}</div>
    <details class="cashout" data-keep><summary>出金（返品返金・経費支出・緊急出金・途中集金）</summary>
      <div class="row wrap">
        <select data-key="co-kind-${reg}">
          <option value="refund">返品返金</option><option value="expense">経費支出</option>
          <option value="emergency">緊急出金</option><option value="pickup">途中集金</option>
        </select>
        <input type="number" inputmode="numeric" min="1" placeholder="金額" data-key="co-amt-${reg}" />
        <input type="password" inputmode="numeric" placeholder="出金パスワード" data-key="co-pw-${reg}" autocomplete="off" />
        <button data-form="cashout" data-reg="${reg}" ${a("cash_out").allowed ? "" : `disabled title="${esc(a("cash_out").reason)}"`}>出金</button>
      </div>
      <p class="muted">ログイン中でも出金のたびにパスワードが必要です。在高 ${yen(v.drawer_total)}</p>
    </details>`;
}

function miniLines(r: RegisterView): string {
  const c = r.customer;
  if (!c.lines.length) return "";
  return `<div class="muted">${c.lines.map((l) => `${esc(l.name)}×${l.qty}`).join("、")}　計 ${yen(c.total)}</div>`;
}

// ---------------------------------------------------------------------------
// お客様画面（公開してよい情報だけを使う）
// ---------------------------------------------------------------------------

export function customerScreen(r: RegisterView): string {
  const c: CustomerView = r.customer;
  const reg = r.id;
  const a = (n: string) => find(c.actions, n);
  const self = r.mode === "full_self";

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

  const presented = c.lines.filter((l) => l.kind === "ff" && l.state === "presented");
  const ffCard = presented.length
    ? `<div class="card ff-card">
        <div class="big">こちらの商品をお渡しします</div>
        <ul>${presented.map((l) => `<li>${esc(l.name)} × ${l.qty}</li>`).join("")}</ul>
        ${btn("受け取りました", { type: "confirm_ff_receipt", register: reg }, a("confirm_ff_receipt"), "primary big-btn")}
      </div>`
    : "";

  const rows = c.lines
    .map((l) => {
      const tag =
        l.kind === "ff"
          ? `<span class="tag ff-${l.state}">${{ waiting: "ご用意中", presented: "お渡し中", received: "お受け取り済" }[l.state]}</span>`
          : l.kind === "service" && !l.done
            ? `<span class="tag warn">係員が受付します</span>`
            : "";
      const rm = self && c.phase === "registering" ? btn("取消", { type: "remove_line", register: reg, side: "customer", line_id: l.id }, a("remove_line"), "small ghost") : "";
      return `<tr><td>${esc(l.name)} ${tag}</td><td class="num">${l.qty}</td><td class="num">${l.kind === "service" && !l.done ? "—" : yen(l.amount)}</td><td>${rm}</td></tr>`;
    })
    .join("");

  if (!c.lines.length) {
    return `${banners.join("")}<div class="welcome">
      <div class="big">いらっしゃいませ</div>
      <p>${self ? "商品のバーコードをスキャンしてください" : "係員が商品をお読みします"}</p>
      ${self ? `<div class="row center">${btn("FF（揚げ物・中華まん）を注文", { type: "ff_button", register: reg }, a("ff_button"), "ff")}</div>` : ""}
      ${r.mode !== "attended" ? `<div class="row center">${btn("係員を呼ぶ", { type: "call_staff", register: reg }, a("call_staff"), "ghost")}</div>` : ""}
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
      ? `<div class="row center">${(["cash", "emoney", "qr"] as const)
          .map((m) => btn(METHOD_LABEL[m], { type: "select_payment", register: reg, side: "customer", method: m }, a("select_payment"), `seg ${c.method === m ? "on" : ""}`))
          .join("")}</div>`
      : "";
    const hint = c.payment_blocker
      ? `<p class="muted">${esc(c.payment_blocker)}</p>`
      : c.method === "emoney" || c.method === "qr"
        ? `<p>端末にタッチしてください</p>`
        : `<p>現金を投入してください${r.mode === "attended" ? "" : "（または支払方法を選んでください）"}</p>`;
    pay = `${age}<div class="paybox center">
        <div>お支払い <b class="total">${yen(c.total)}</b></div>
        ${c.inserted ? `<div>お預かり ${yen(c.inserted)}　残り <b>${yen(c.due)}</b></div>` : ""}
        ${methods}${hint}
      </div>`;
  }

  const actionsRow =
    c.phase === "registering"
      ? `<div class="row">
          ${self ? btn("お会計", { type: "checkout", register: reg, side: "customer" }, a("checkout"), "primary") : ""}
          ${self ? btn("FF を注文", { type: "ff_button", register: reg }, a("ff_button"), "ff") : ""}
          ${r.mode !== "attended" ? btn("係員を呼ぶ", { type: "call_staff", register: reg }, a("call_staff"), "ghost") : ""}
        </div>`
      : `<div class="row">${self ? btn("商品登録に戻る", { type: "back_to_registering", register: reg, side: "customer" }, a("back_to_registering"), "ghost") : ""}
          ${r.mode !== "attended" ? btn("係員を呼ぶ", { type: "call_staff", register: reg }, a("call_staff"), "ghost") : ""}</div>`;

  return `${banners.join("")}${ffCard}
    <table class="lines"><tbody>${rows}</tbody></table>
    <div class="totals"><span>合計 <b class="total">${yen(c.total)}</b></span>${c.point_card ? `<span class="muted">ポイントカード 読み取り済</span>` : ""}</div>
    ${pay}${actionsRow}`;
}

// ---------------------------------------------------------------------------
// MAT・店舗の設定
// ---------------------------------------------------------------------------

export function matPanel(s: Snapshot): string {
  return s.mats
    .map(
      (m) => `<div class="mat ${m.buzzing ? "buzz" : ""} ${m.link_ok ? "" : "down"}">
        <div class="bell">🔔</div>
        <div class="mat-id">${esc(m.id)}</div>
        <div class="mat-state">${m.link_ok ? (m.buzzing ? "鳴動中" : "待機") : "MAT不通"}</div>
      </div>`,
    )
    .join("");
}

export function simPanel(s: Snapshot): string {
  const links = s.mat_links
    .map((l, i) => {
      const off = s.mats[i]?.offline ?? false;
      return `<div class="row wrap">${esc(l.id)}：${l.link_ok ? "通信OK" : "<b class='warn'>不通</b>"}　送信${l.sent} 再送${l.retries} 鳴動${s.mats[i]?.rings ?? 0}回
        ${btn(off ? "電源を戻す" : "電源断/圏外にする", { type: "set_mat_offline", mat: i, offline: !off }, undefined, "small ghost")}</div>`;
    })
    .join("");
  const loss = [0, 30, 60, 90]
    .map((p) => btn(`${p}%`, { type: "set_mat_loss", percent: p }, undefined, `seg small ${s.mat_loss === p ? "on" : ""}`))
    .join("");
  const calls = s.calls.length
    ? s.calls.map((c) => `<li>レジ${c.register}：${esc(c.label)}（${secs(s.now, c.created_at)}）</li>`).join("")
    : `<li class="muted">なし</li>`;
  return `<div class="row wrap">MAT 通信ロス：${loss}</div>${links}
    <div class="muted">未対応（店員がまだ来ていない）呼び出し ${s.bell.open_calls} 件。残っている間は ${10} 秒ごとに鳴らし直します。</div>
    <h4>開いている呼び出し（シミュレーター表示）</h4><ul>${calls}</ul>`;
}

export function ownerPanel(s: Snapshot): string {
  const rules = s.age_rules.rules
    .map(
      (r) => `<tr><td>${esc(r.label)}</td><td class="num">${r.min_age}歳</td>
        <td>${r.legal ? `<span class="tag">法令（外せません）</span>` : `<button class="small ghost" data-form="remove-rule" data-rule="${r.id}">削除</button>`}</td></tr>`,
    )
    .join("");
  const cats = s.categories.map(([k, label]) => `<option value="${esc(k)}">${esc(label)}</option>`).join("");
  const prods = s.catalog.map((p) => `<option value="${esc(p.code)}">${esc(p.name)}</option>`).join("");
  const hist = s.age_rules.history.length
    ? s.age_rules.history.map((h) => `<li>${esc(h.by)}：${esc(h.change)}</li>`).join("")
    : `<li class="muted">変更はまだありません</li>`;
  return `<p class="muted">年確ルールを変えられるのはオーナーだけです。操作のたびにオーナーのパスワードで再認証します。</p>
    <div class="row wrap">
      <input data-key="ow-id" placeholder="オーナーの従業員番号" value="9001" />
      <input data-key="ow-pw" type="password" placeholder="パスワード" autocomplete="off" />
    </div>
    <table class="lines"><thead><tr><th>対象</th><th>年齢</th><th></th></tr></thead><tbody>${rules}</tbody></table>
    <div class="row wrap">
      <select data-key="ar-kind"><option value="category">分類で指定</option><option value="product">商品で指定</option></select>
      <select data-key="ar-cat">${cats}</select>
      <select data-key="ar-code">${prods}</select>
      <input data-key="ar-age" type="number" min="1" max="99" value="20" style="width:5em" />歳以上
      <button data-form="add-rule">ルールを追加</button>
    </div>
    <h4>変更履歴</h4><ul>${hist}</ul>`;
}

export function auditPanel(s: Snapshot): string {
  if (!s.audit.length) return `<p class="muted">出金操作の記録はまだありません</p>`;
  return `<table class="lines"><thead><tr><th>時刻</th><th>レジ</th><th>操作者</th><th>操作</th><th>金額</th><th>結果</th></tr></thead><tbody>
    ${s.audit
      .slice()
      .reverse()
      .map((e) => `<tr><td>${Math.floor(e.at / 1000)}秒</td><td>${e.register}</td><td>${esc(e.staff)}</td><td>${esc(e.operation)}</td><td class="num">${yen(e.amount)}</td><td>${esc(e.result)}</td></tr>`)
      .join("")}</tbody></table>`;
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
  const cards =
    item("PC12345678", "ポイントカード", "お客様のカード", "card") +
    s.accounts.map((a) => item(a.barcode, `名札 ${a.name}`, "スタッフバーコード", "staff")).join("");
  return `${groups}<div class="shelf"><h3>カード・名札</h3><div class="items">${cards}</div></div>`;
}

export function cashPanel(reg: number): string {
  const denoms = [10000, 5000, 2000, 1000, 500, 100, 50, 10, 5, 1];
  return `<div class="row wrap">${denoms
    .map((d) => `<button class="cash ${d >= 1000 ? "bill" : "coin"}" data-cmd="${esc(JSON.stringify({ type: "insert_cash", register: reg, denomination: d, count: 1 }))}">${d.toLocaleString()}</button>`)
    .join("")}
    <button class="emoney" data-cmd="${esc(JSON.stringify({ type: "tap_emoney", register: reg }))}">📶 端末にタッチ</button></div>`;
}

/** コマンドがどちらの画面からの操作か（お知らせを出す場所を決める） */
export function sideOf(cmd: Command): "staff" | "customer" | "store" {
  if (cmd.side === "staff" || cmd.side === "customer") return cmd.side;
  if (["ff_button", "confirm_ff_receipt", "confirm_age", "insert_cash", "tap_emoney", "call_staff"].includes(cmd.type)) return "customer";
  if (typeof cmd.register === "number") return "staff";
  return "store";
}
