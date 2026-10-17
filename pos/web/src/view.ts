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
  // 店員画面には常にお客様画面の内容を映す。ログイン（名札）で操作できるようになる
  if (r.active_side !== "staff" && r.menu) {
    return `<div class="mirror-head"><b>お客様画面で店舗メニューを操作中</b>　${esc(MENU_LABEL[r.menu.screen])}</div>`;
  }
  if (r.active_side !== "staff") {
    const why =
      v.state === "absent"
        ? "店員未ログイン"
        : v.state === "locked"
          ? "ロック中"
          : v.age_pending
            ? "お客様の年齢確認待ち"
            : v.lines.some((l) => l.kind === "ff" && l.state === "presented")
              ? "お客様の FF 受取確認待ち"
              : "お客様のお支払い中";
    const how =
      v.state === "ready" && (v.age_pending || v.lines.some((l) => l.kind === "ff" && l.state === "presented"))
        ? "お客様の確認が済むと操作に戻ります"
        : "レジ名の右の「店員」でログインすると有人モードになり、操作できます";
    return `<div class="mirror-head"><b>${why}</b>　${how}</div>
      <div class="mirror">${customerScreen(r, snap.ff_stock)}</div>`;
  }


  const rows = v.lines.map((l) => staffLine(reg, l, v)).join("");
  const paying = v.phase === "paying";

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

  return `<div class="staff-head">
      <span>店員：<b>${esc(v.name)}</b>${v.role === "owner" ? "（オーナー）" : ""}</span>
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
    <div class="ffmenu"><span class="muted">FF 登録</span>${ffMenu}</div>`;
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
        <p class="muted">お金を抜く操作です。店舗パスワードが毎回必要で、お金は排出口に出ます。</p>
        <div class="menu-form">
          <input type="number" inputmode="numeric" min="1" placeholder="金額" data-key="mo-amt-${reg}" />
          <input type="password" inputmode="numeric" placeholder="店舗パスワード" data-key="mo-pw-${reg}" autocomplete="off" />
          <button class="primary" data-form="cashout" data-reg="${reg}" data-kind="${m.screen}" ${cout.allowed ? "" : `disabled title="${esc(cout.reason ?? "")}"`}>出金</button>
        </div>`;
    }
    case "deposit":
      return `${head(MENU_LABEL.deposit)}
        <p>入金するお金を<b>投入口</b>に入れてください。</p>
        <p class="muted">お金を入れる操作なのでパスワードは要りません。記録は担当者名つきで残ります。</p>`;
    case "changer": {
      const have = (d: number) => v.drawer.find(([x]) => x === d)?.[1] ?? 0;
      const segs = [1000, 500, 100, 50, 10]
        .map((d) => btn(`${d.toLocaleString()}円`, go("changer", { exchange_to: d }), undefined, `seg small ${m.exchange_to === d ? "on" : ""}`))
        .join("");
      return `${head(MENU_LABEL.changer)}
        <div class="row wrap">受け取る金種 ${segs}</div>
        <p>両替するお金を<b>投入口</b>に入れてください。同じ額を ${m.exchange_to.toLocaleString()}円 で<b>排出口</b>に出します。</p>
        <p class="muted">釣銭機の ${m.exchange_to.toLocaleString()}円：残り ${have(m.exchange_to)} 枚　在高 ${yen(v.drawer_total)}</p>`;
    }
  }
}

// ---------------------------------------------------------------------------
// お客様画面（公開してよい情報だけを使う）
// ---------------------------------------------------------------------------

export function customerScreen(r: RegisterView, ff: Snapshot["ff_stock"] = []): string {
  // 店員が店舗メニューを開いている間は、お客様画面にメニューを出す
  if (r.menu) return storeMenu(r);
  const c: CustomerView = r.customer;
  const reg = r.id;
  const a = (n: string) => find(c.actions, n);
  // セミセルフ（名札なし）とセルフは、お客様が自分で登録・会計する
  const self = r.mode !== "attended";
  // セルフモード（将来の拡張）では、お客様がケースから取った FF も自分で登録する
  const ffMenu = r.mode === "full_self"
    ? `<div class="ffmenu center"><span class="muted">FF（揚げ物・中華まん）</span>${ff
        .map((f) =>
          btn(
            `${f.name} ${yen(f.price)}`,
            { type: "register_ff", register: reg, side: "customer", code: f.code, qty: 1 },
            f.stock > 0 ? a("register_ff") : { action: "register_ff", allowed: false, reason: "売り切れです" },
            "small ff",
          ),
        )
        .join("")}</div>`
    : "";
  // セミセルフでは FF は係員が登録する（ボタンで呼び出す）
  const ffOrder = r.mode === "semi_self" ? btn("FF（揚げ物・中華まん）を注文", { type: "ff_button", register: reg }, a("ff_button"), "ff") : "";

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
      ${ffMenu}${ffOrder ? `<div class="row center">${ffOrder}</div>` : ""}
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
          ${ffOrder}
          ${r.mode !== "attended" ? btn("係員を呼ぶ", { type: "call_staff", register: reg }, a("call_staff"), "ghost") : ""}
        </div>${ffMenu}`
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
  const cards =
    item("PC12345678", "ポイントカード", "お客様のカード", "card");
  return `${groups}<div class="shelf"><h3>カード</h3><div class="items">${cards}</div></div>`;
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
