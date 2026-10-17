//! 新型レジで報告された問題と、壁打ちで決めた仕様を、シナリオで検証する。

use janus_pos_core::authority::{Action, Mode, Phase, Side, StaffState};
use janus_pos_core::calls::CallKind;
use janus_pos_core::store::{Command, RegisterView, Store};
use serde_json::{json, Value};

const ONIGIRI: &str = "4900000000011";
const BEER: &str = "4900000000103";
const LIGHTER: &str = "4900000000134";
const LOTTERY: &str = "4900000000158";
const FIREWORKS: &str = "4900000000165";
const PARCEL: &str = "2100000000014";

fn run(s: &mut Store, v: Value) -> Result<Option<String>, String> {
    let cmd: Command = serde_json::from_value(v.clone()).unwrap_or_else(|e| panic!("{v}: {e}"));
    s.dispatch(cmd).map(|o| o.message)
}

fn ok(s: &mut Store, v: Value) {
    if let Err(e) = run(s, v.clone()) {
        panic!("{v} failed: {e}");
    }
}

fn reg(s: &Store, id: u8) -> RegisterView {
    s.snapshot()
        .registers
        .into_iter()
        .find(|r| r.id == id)
        .unwrap()
}

fn allowed(v: &RegisterView, side: Side, a: Action) -> bool {
    let list = match side {
        Side::Staff => &v.staff.actions,
        Side::Customer => &v.customer.actions,
    };
    list.iter().find(|x| x.action == a).unwrap().allowed
}

fn scan(s: &mut Store, r: u8, side: &str, code: &str) {
    ok(
        s,
        json!({"type":"scan","register":r,"side":side,"code":code}),
    );
}

/// 名札（山田）を読んでログインし、店員画面にタッチして操作を引き継ぐ。
/// ログイン中は有人モード（ログイン直後はお客様画面が操作側）
fn login(s: &mut Store, r: u8) {
    login_as(s, r, "ST1001");
}

fn login_as(s: &mut Store, r: u8, badge: &str) {
    scan(s, r, "staff", badge);
    ok(s, json!({"type":"claim","register":r,"side":"staff"}));
}

fn advance(s: &mut Store, ms: u64) {
    let end = s.now() + ms;
    while s.now() < end {
        let t = (s.now() + 20).min(end);
        s.tick(t);
    }
}

/// 会計画面までは、どのバーコードも順番の縛りなく受け付ける。
#[test]
fn any_barcode_is_accepted_in_any_order() {
    let mut s = Store::new(1, 2, 1);
    login(&mut s, 1);
    scan(&mut s, 1, "staff", ONIGIRI);
    scan(&mut s, 1, "customer", "PC12345678");
    scan(&mut s, 1, "staff", "2000000000015");
    scan(&mut s, 1, "customer", ONIGIRI);
    let v = reg(&s, 1);
    assert_eq!(v.staff.lines.len(), 2);
    assert_eq!(v.staff.total, 160 * 2 + 3);
    assert!(v.customer.point_card);
    assert!(run(
        &mut s,
        json!({"type":"scan","register":1,"side":"staff","code":"0000"})
    )
    .is_err());
}

/// 会計画面で読んだら登録画面に戻って追加し、投入済みの現金は残る。
#[test]
fn scanning_on_payment_screen_returns_to_registering() {
    let mut s = Store::new(1, 2, 1);
    login(&mut s, 1);
    scan(&mut s, 1, "staff", ONIGIRI);
    ok(
        &mut s,
        json!({"type":"checkout","register":1,"side":"staff"}),
    );
    ok(
        &mut s,
        json!({"type":"insert_cash","register":1,"denomination":100,"count":1}),
    );
    scan(&mut s, 1, "customer", ONIGIRI);
    let v = reg(&s, 1);
    assert_eq!(v.staff.phase, Phase::Registering);
    assert_eq!(v.staff.inserted, 100);
    ok(
        &mut s,
        json!({"type":"checkout","register":1,"side":"staff"}),
    );
    ok(
        &mut s,
        json!({"type":"insert_cash","register":1,"denomination":500,"count":1}),
    );
    let v = reg(&s, 1);
    assert_eq!(v.last_receipt.as_ref().unwrap().change_total, 600 - 320);
}

/// 問題7：年確は会計ボタンの時点だけ。年確が済むまで決済できない。
#[test]
fn age_check_happens_at_checkout_and_blocks_payment() {
    let mut s = Store::new(1, 2, 1);
    login(&mut s, 1);
    scan(&mut s, 1, "staff", BEER);
    assert_eq!(
        reg(&s, 1).customer.age_check,
        None,
        "no age check at scan time"
    );
    ok(
        &mut s,
        json!({"type":"checkout","register":1,"side":"staff"}),
    );
    let v = reg(&s, 1);
    assert_eq!(v.customer.age_check, Some(20));
    assert!(run(
        &mut s,
        json!({"type":"insert_cash","register":1,"denomination":1000,"count":1})
    )
    .is_err());
    ok(&mut s, json!({"type":"confirm_age","register":1}));
    // 同じ取引で戻って追加しても、再確認はしない
    scan(&mut s, 1, "staff", BEER);
    ok(
        &mut s,
        json!({"type":"checkout","register":1,"side":"staff"}),
    );
    assert_eq!(reg(&s, 1).customer.age_check, None);
    ok(
        &mut s,
        json!({"type":"insert_cash","register":1,"denomination":1000,"count":1}),
    );
}

/// セミセルフでは年確が店員呼び出しになり、店員が来るまでお客様は確認できない。
#[test]
fn self_mode_age_check_calls_staff() {
    let mut s = Store::new(1, 2, 1);
    scan(&mut s, 2, "customer", BEER);
    ok(
        &mut s,
        json!({"type":"checkout","register":2,"side":"customer"}),
    );
    assert!(s
        .snapshot()
        .calls
        .iter()
        .any(|c| c.register == 2 && c.kind == CallKind::AgeCheck));
    assert!(!allowed(&reg(&s, 2), Side::Customer, Action::ConfirmAge));
    scan(&mut s, 2, "staff", "ST1002");
    ok(&mut s, json!({"type":"confirm_age","register":2}));
    // 名札を読んだので有人モード。モードはログアウトか 5 分の無操作でだけ戻る
    assert_eq!(reg(&s, 2).mode, Mode::Attended);
    assert_eq!(reg(&s, 2).staff.state, StaffState::Ready);
    assert!(s.snapshot().calls.is_empty());
}

/// 店舗ルール：オーナーだけが追加でき、法令ルールは外せない。
#[test]
fn only_owner_changes_age_rules() {
    let mut s = Store::new(1, 2, 1);
    login(&mut s, 1);
    let add = |id: &str, pw: &str| {
        json!({"type":"add_age_rule","owner_id":id,"password":pw,
               "target":{"kind":"product","code":LOTTERY},"min_age":20})
    };
    assert!(run(&mut s, add("1001", "1111")).is_err());
    ok(&mut s, add("9001", "9999"));
    assert!(run(
        &mut s,
        json!({"type":"remove_age_rule","owner_id":"9001","password":"9999","rule_id":1})
    )
    .is_err());
    scan(&mut s, 1, "staff", LOTTERY);
    scan(&mut s, 1, "staff", LIGHTER);
    ok(
        &mut s,
        json!({"type":"checkout","register":1,"side":"staff"}),
    );
    assert_eq!(reg(&s, 1).customer.age_check, Some(20));
    assert_eq!(s.snapshot().age_rules.history.len(), 1);
}

/// 問題3・5：有人モードのお客様は流れを変えられず、店員はどのモードでも操作できる。
#[test]
fn customer_cannot_take_over_attended_register() {
    let mut s = Store::new(1, 2, 1);
    login(&mut s, 1);
    scan(&mut s, 1, "staff", ONIGIRI);
    assert!(run(
        &mut s,
        json!({"type":"checkout","register":1,"side":"customer"})
    )
    .is_err());
    ok(
        &mut s,
        json!({"type":"checkout","register":1,"side":"staff"}),
    );
    assert!(run(
        &mut s,
        json!({"type":"select_payment","register":1,"side":"customer","method":"qr"})
    )
    .is_err());
    ok(
        &mut s,
        json!({"type":"back_to_registering","register":1,"side":"staff"}),
    );
    assert_eq!(reg(&s, 1).staff.phase, Phase::Registering);
}

/// FF：セミセルフの FF ボタンで呼び出し → 名札でログイン（有人）して登録 → お渡し → お客様の受取確認。
#[test]
fn fast_food_flow_in_semi_self_mode() {
    let mut s = Store::new(1, 2, 1);
    scan(&mut s, 2, "customer", ONIGIRI);
    assert!(run(
        &mut s,
        json!({"type":"register_ff","register":2,"code":"FF01","qty":1})
    )
    .is_err());
    ok(&mut s, json!({"type":"ff_button","register":2}));
    login_as(&mut s, 2, "ST1001");
    ok(
        &mut s,
        json!({"type":"register_ff","register":2,"code":"FF01","qty":2}),
    );
    assert!(
        run(
            &mut s,
            json!({"type":"register_ff","register":2,"code":"FF04","qty":1})
        )
        .is_err(),
        "sold out"
    );
    // 店員が操作している間、お客様画面は操作できない（排他）
    assert!(run(
        &mut s,
        json!({"type":"checkout","register":2,"side":"customer"})
    )
    .is_err());
    // 店員が会計に進めても、FF のお渡しが済むまで決済できない
    ok(
        &mut s,
        json!({"type":"checkout","register":2,"side":"staff"}),
    );
    assert!(run(&mut s, json!({"type":"tap_emoney","register":2})).is_err());
    ok(&mut s, json!({"type":"hand_over_ff","register":2}));
    assert_eq!(reg(&s, 2).mode, Mode::Attended, "badge keeps the register attended");
    ok(&mut s, json!({"type":"confirm_ff_receipt","register":2}));
    // 有人モードなので支払方法は店員が選ぶ
    assert!(run(
        &mut s,
        json!({"type":"select_payment","register":2,"side":"customer","method":"emoney"})
    )
    .is_err());
    ok(
        &mut s,
        json!({"type":"select_payment","register":2,"side":"staff","method":"emoney"}),
    );
    ok(&mut s, json!({"type":"tap_emoney","register":2}));
    assert_eq!(reg(&s, 2).last_receipt.unwrap().total, 160 + 440);
}

/// 要店員のサービス商品：お客様が読むと呼び出し、店員の受付まで決済できない。
#[test]
fn service_item_needs_staff() {
    let mut s = Store::new(1, 2, 1);
    scan(&mut s, 2, "customer", PARCEL);
    assert!(s
        .snapshot()
        .calls
        .iter()
        .any(|c| c.kind == CallKind::Service));
    login_as(&mut s, 2, "ST1001");
    let id = reg(&s, 2).staff.lines[0].line.id;
    ok(
        &mut s,
        json!({"type":"complete_service","register":2,"line_id":id,"amount":1200}),
    );
    assert_eq!(reg(&s, 2).mode, Mode::Attended);
    assert_eq!(reg(&s, 2).customer.total, 1200);
}

/// 問題6：出金は毎回、ログインとは別の出金パスワードが要り、記録が残る。
#[test]
fn cash_out_requires_separate_password_every_time() {
    let mut s = Store::new(1, 2, 1);
    login(&mut s, 1);
    let co = |pw: &str| json!({"type":"cash_out","register":1,"kind":"emergency","amount":5000,"password":pw});
    assert!(
        run(&mut s, co("1111")).is_err(),
        "login password is not the cash-out password"
    );
    ok(&mut s, co("7777"));
    ok(&mut s, co("7777"));
    let a = s.snapshot().audit;
    assert_eq!(a.len(), 3);
    assert_eq!(a[1].staff, "山田");
    // 未ログインのレジでは出金できない
    assert!(run(
        &mut s,
        json!({"type":"cash_out","register":2,"kind":"refund","amount":100,"password":"7777"})
    )
    .is_err());
}

/// 問題8：呼び出しは発生したレジだけに出て、ほかのレジには出ない。
#[test]
fn calls_stay_on_originating_register() {
    let mut s = Store::new(1, 4, 2);
    ok(&mut s, json!({"type":"call_staff","register":2}));
    ok(&mut s, json!({"type":"call_staff","register":4}));
    let snap = s.snapshot();
    assert_eq!(snap.registers[1].staff.calls.len(), 1);
    assert!(snap.registers[0].staff.calls.is_empty());
    assert!(snap.registers[2].staff.calls.is_empty());
    // 後から来たレジ4を先に片付けても、レジ2は残る
    login_as(&mut s, 4, "ST1002");
    let snap = s.snapshot();
    assert_eq!(snap.calls.len(), 1);
    assert_eq!(snap.calls[0].register, 2);
}

/// 問題10：通信ロスがあっても再送で MAT は鳴り、不通は検知される。
#[test]
fn mat_rings_despite_loss_and_reports_link_down() {
    let mut s = Store::new(7, 2, 1);
    ok(&mut s, json!({"type":"set_mat_loss","percent":60}));
    advance(&mut s, 3_000);
    ok(&mut s, json!({"type":"call_staff","register":2}));
    advance(&mut s, 8_000);
    let snap = s.snapshot();
    assert!(snap.mats[0].rings >= 1);
    assert!(snap.mat_links[0].retries > 0 || snap.mat_links[0].acked_seq == snap.bell.seq);
    // 未対応の間は鳴らし直す
    let before = snap.mats[0].rings;
    advance(&mut s, 25_000);
    assert!(s.snapshot().mats[0].rings > before);

    ok(
        &mut s,
        json!({"type":"set_mat_offline","mat":0,"offline":true}),
    );
    advance(&mut s, 8_000);
    let snap = s.snapshot();
    assert!(!snap.mats[0].link_ok);
    assert_eq!(snap.registers[0].staff.mat_down, vec!["MAT-1".to_string()]);
}

/// お客様画面には分類や年確対象の情報を出さない（店員画面にだけ出す）。
#[test]
fn customer_view_hides_staff_only_information() {
    let mut s = Store::new(1, 2, 1);
    login(&mut s, 1);
    scan(&mut s, 1, "staff", BEER);
    scan(&mut s, 1, "staff", "PC12345678");
    let v = serde_json::to_value(reg(&s, 1)).unwrap();
    let c = v["customer"].to_string();
    assert!(!c.contains("alcohol") && !c.contains(BEER) && !c.contains("12345678"));
    assert_eq!(v["staff"]["lines"][0]["age"], 20);
    assert_eq!(reg(&s, 1).mode, Mode::Attended);
}

/// 画面の操作は排他：セミセルフ（名札なし）はお客様画面、有人（名札あり）は店員画面が操作する。
#[test]
fn staff_and_customer_screens_are_exclusive() {
    let mut s = Store::new(1, 2, 1);
    scan(&mut s, 1, "customer", ONIGIRI);
    let v = reg(&s, 1);
    assert_eq!(v.mode, Mode::SemiSelf);
    assert_eq!(v.active_side, Side::Customer);
    assert!(run(&mut s, json!({"type":"void_transaction","register":1})).is_err());
    ok(&mut s, json!({"type":"checkout","register":1,"side":"customer"}));
    ok(
        &mut s,
        json!({"type":"select_payment","register":1,"side":"customer","method":"qr"}),
    );
    // 店員が名札を読むと有人モードになり、支払い中でも店員側が操作する
    scan(&mut s, 1, "staff", "ST1002");
    let v = reg(&s, 1);
    assert_eq!(v.mode, Mode::Attended);
    assert_eq!(v.active_side, Side::Staff);
    assert!(run(
        &mut s,
        json!({"type":"select_payment","register":1,"side":"customer","method":"cash"})
    )
    .is_err());
    ok(
        &mut s,
        json!({"type":"back_to_registering","register":1,"side":"staff"}),
    );
    // ログアウトするとセミセルフに戻り、お客様が続きを操作する
    ok(&mut s, json!({"type":"end_staff_mode","register":1}));
    let v = reg(&s, 1);
    assert_eq!(v.mode, Mode::SemiSelf);
    assert_eq!(v.active_side, Side::Customer);
    ok(&mut s, json!({"type":"checkout","register":1,"side":"customer"}));
}

/// 年確ルールは売り場のどの商品にも個別に付けられる（オーナーのみ、法令は外せない）。
#[test]
fn owner_sets_age_rule_per_product() {
    let mut s = Store::new(1, 2, 1);
    let set = |id: &str, pw: &str, code: &str, age: Option<u8>| {
        json!({"type":"set_product_age","owner_id":id,"password":pw,"code":code,"min_age":age})
    };
    assert!(run(&mut s, set("1001", "1111", LOTTERY, Some(20))).is_err());
    ok(&mut s, set("9001", "9999", LOTTERY, Some(20)));
    ok(&mut s, set("9001", "9999", LIGHTER, None));
    // 酒・たばこは法令どおりで変更不可
    assert!(run(&mut s, set("9001", "9999", BEER, None)).is_err());
    assert!(run(&mut s, set("9001", "9999", BEER, Some(25))).is_err());
    // 花火も売り場の商品として個別に設定できる
    ok(&mut s, set("9001", "9999", FIREWORKS, Some(18)));
    let cat = s.snapshot().catalog;
    let age = |c: &str| cat.iter().find(|p| p.code == c).unwrap().age;
    assert_eq!(age(LOTTERY), Some(20));
    assert_eq!(age(LIGHTER), None);
    assert_eq!(age(BEER), Some(20));
    assert_eq!(age(FIREWORKS), Some(18));
}

/// 運用モードは名札で決まる。手で有人／セミセルフは選べず、セルフは機能 OFF。
#[test]
fn mode_follows_staff_badge() {
    let mut s = Store::new(1, 2, 1);
    assert!(s.snapshot().registers.iter().all(|r| r.mode == Mode::SemiSelf
        && r.staff.state == StaffState::Absent));
    login(&mut s, 1);
    assert_eq!(reg(&s, 1).mode, Mode::Attended);
    for m in ["attended", "semi_self", "full_self"] {
        assert!(run(&mut s, json!({"type":"set_mode","register":1,"mode":m})).is_err());
    }
    ok(&mut s, json!({"type":"end_staff_mode","register":1}));
    assert_eq!(reg(&s, 1).mode, Mode::SemiSelf);
    assert!(run(&mut s, json!({"type":"set_mode","register":1,"mode":"full_self"})).is_err());
}

/// セルフモード（将来の拡張）：有効にすると、お客様が FF も自分で登録できる。
#[test]
fn customer_registers_ff_in_self_mode() {
    let mut s = Store::new(1, 2, 1);
    let ff = |r: u8| json!({"type":"register_ff","register":r,"side":"customer","code":"FF01","qty":1});
    // セミセルフでは FF は店員の登録
    assert!(run(&mut s, ff(2)).is_err());
    s.set_self_mode_enabled(true);
    ok(&mut s, json!({"type":"set_mode","register":2,"mode":"full_self"}));
    ok(&mut s, ff(2));
    let v = reg(&s, 2);
    assert_eq!(v.customer.lines.len(), 1);
    assert!(s.snapshot().calls.is_empty(), "no staff call for self-picked FF");
    ok(&mut s, json!({"type":"checkout","register":2,"side":"customer"}));
    ok(&mut s, json!({"type":"insert_cash","register":2,"denomination":1000,"count":1}));
    assert!(reg(&s, 2).customer.lines.is_empty(), "paid and settled");
}

/// 名札でログインした有人モードも、5 分間操作がなければログアウトしてセミセルフに戻る。
/// 混雑時に急に切り替わらないよう、取引中は戻さない。
#[test]
fn idle_register_returns_to_semi_self() {
    let mut s = Store::new(1, 2, 1);
    login(&mut s, 2);
    advance(&mut s, 4 * 60_000);
    // 途中で操作があれば数え直す
    scan(&mut s, 2, "staff", ONIGIRI);
    ok(&mut s, json!({"type":"void_transaction","register":2}));
    advance(&mut s, 4 * 60_000);
    assert_eq!(reg(&s, 2).mode, Mode::Attended);
    advance(&mut s, 61_000);
    let v = reg(&s, 2);
    assert_eq!(v.mode, Mode::SemiSelf);
    assert_eq!(v.staff.state, StaffState::Absent);
    assert!(v.idle_revert_in.is_none());
    // 取引中は戻さない
    login(&mut s, 1);
    scan(&mut s, 1, "staff", ONIGIRI);
    advance(&mut s, 6 * 60_000);
    assert_eq!(reg(&s, 1).mode, Mode::Attended);
}

/// 店舗メニュー：お金を入れる操作はパスワード不要、抜く操作は店舗パスワードが要る。どちらも記録が残る。
#[test]
fn cash_in_needs_no_password_but_cash_out_does() {
    let mut s = Store::new(1, 2, 1);
    let cin = json!({"type":"cash_in","register":1,"kind":"deposit","amount":3000});
    // 名札でログインした有人モードでだけ使える
    assert!(run(&mut s, cin.clone()).is_err());
    login(&mut s, 1);
    let before = reg(&s, 1).staff.drawer_total;
    ok(&mut s, cin);
    ok(&mut s, json!({"type":"cash_in","register":1,"kind":"changer_refill","amount":1500}));
    assert_eq!(reg(&s, 1).staff.drawer_total, before + 4500);
    let out = |pw: &str| json!({"type":"cash_out","register":1,"kind":"changer_collect","amount":4500,"password":pw});
    assert!(run(&mut s, out("")).is_err());
    ok(&mut s, out("7777"));
    assert_eq!(reg(&s, 1).staff.drawer_total, before);
    let ops: Vec<_> = s.snapshot().audit.iter().map(|a| a.operation).collect();
    assert_eq!(ops, vec!["取引外入金", "釣銭補充", "釣銭機回収", "釣銭機回収"]);
    // 取引中は入出金できない
    scan(&mut s, 1, "staff", ONIGIRI);
    assert!(run(&mut s, json!({"type":"cash_in","register":1,"kind":"deposit","amount":100})).is_err());
}

/// 投入口と排出口：投入口に金額を入れて支払い、おつりは排出口に出て取り出すまで残る。
#[test]
fn inlet_pays_and_change_waits_in_outlet() {
    let mut s = Store::new(1, 2, 1);
    scan(&mut s, 1, "customer", ONIGIRI);
    // お会計前は受け付けない
    assert!(run(&mut s, json!({"type":"insert","register":1,"amount":1000})).is_err());
    ok(&mut s, json!({"type":"checkout","register":1,"side":"customer"}));
    ok(&mut s, json!({"type":"insert","register":1,"amount":1000}));
    let v = reg(&s, 1);
    assert_eq!(v.outlet, 1000 - 160);
    ok(&mut s, json!({"type":"take_outlet","register":1}));
    assert_eq!(reg(&s, 1).outlet, 0);
    assert!(run(&mut s, json!({"type":"take_outlet","register":1})).is_err());
}

/// 店舗メニューはお客様画面に一画面ずつ出す。開いている間はお客様画面側で操作し、
/// 入金・両替は投入口、出金と両替のお金は排出口を使う。
#[test]
fn store_menu_uses_inlet_and_outlet() {
    let mut s = Store::new(1, 2, 1);
    let open = |screen: &str| json!({"type":"open_menu","register":1,"screen":screen});
    assert!(run(&mut s, open("top")).is_err(), "only for logged-in staff");
    login(&mut s, 1);
    ok(&mut s, open("top"));
    let v = reg(&s, 1);
    assert_eq!(v.active_side, Side::Customer);
    assert!(run(&mut s, json!({"type":"lock","register":1})).is_err(), "staff screen waits");
    // 取引外入金：投入口に入れた金額が入金になる
    ok(&mut s, open("deposit"));
    let before = reg(&s, 1).staff.drawer_total;
    ok(&mut s, json!({"type":"insert","register":1,"amount":2000}));
    assert_eq!(reg(&s, 1).staff.drawer_total, before + 2000);
    // 自動釣銭機：一万円を千円札に両替して排出口へ
    ok(&mut s, json!({"type":"open_menu","register":1,"screen":"changer","exchange_to":1000}));
    assert!(run(&mut s, json!({"type":"insert","register":1,"amount":1500})).is_err());
    ok(&mut s, json!({"type":"insert","register":1,"amount":10000}));
    let v = reg(&s, 1);
    assert_eq!(v.outlet_detail, vec![(1000, 10)]);
    assert_eq!(v.staff.drawer_total, before + 2000);
    ok(&mut s, json!({"type":"take_outlet","register":1}));
    // 緊急出金：パスワードが要り、お金は排出口に出る
    ok(&mut s, open("emergency"));
    assert!(run(&mut s, json!({"type":"cash_out","register":1,"kind":"emergency","amount":5000,"password":"x"})).is_err());
    ok(&mut s, json!({"type":"cash_out","register":1,"kind":"emergency","amount":5000,"password":"7777"}));
    assert_eq!(reg(&s, 1).outlet, 5000);
    // 閉じると店員画面に戻る。ログアウトでもメニューは閉じる
    ok(&mut s, json!({"type":"open_menu","register":1,"screen":null}));
    assert_eq!(reg(&s, 1).active_side, Side::Staff);
    ok(&mut s, open("top"));
    ok(&mut s, json!({"type":"scan","register":1,"side":"staff","code":"ST1002"}));
    ok(&mut s, json!({"type":"end_staff_mode","register":1}));
    assert!(reg(&s, 1).menu.is_none());
}

/// 有人モード：ログイン直後はお客様画面が操作側。FF はどちらの画面からも登録でき、
/// 操作している画面が触られていなければ、もう一方がタッチで引き継ぐ。
#[test]
fn attended_screen_is_held_by_one_side_at_a_time() {
    let mut s = Store::new(1, 2, 1);
    scan(&mut s, 1, "staff", "ST1001");
    let v = reg(&s, 1);
    assert_eq!(v.mode, Mode::Attended);
    assert_eq!(v.active_side, Side::Customer);
    assert!(allowed(&v, Side::Customer, Action::RegisterFf));
    assert!(!allowed(&v, Side::Staff, Action::RegisterFf));
    // お客様画面から FF を登録（係員がお渡しする）
    ok(
        &mut s,
        json!({"type":"register_ff","register":1,"side":"customer","code":"FF01","qty":1}),
    );
    assert!(reg(&s, 1).staff.calls.is_empty());
    // お客様画面を触った直後は、店員画面から引き継げない
    let e = run(&mut s, json!({"type":"claim","register":1,"side":"staff"})).unwrap_err();
    assert!(e.contains("操作中"), "{e}");
    assert!(run(
        &mut s,
        json!({"type":"register_ff","register":1,"side":"staff","code":"FF02","qty":1})
    )
    .is_err());
    // しばらく触られなければ、店員画面にタッチして引き継ぐ
    advance(&mut s, 3_000);
    ok(&mut s, json!({"type":"claim","register":1,"side":"staff"}));
    assert_eq!(reg(&s, 1).active_side, Side::Staff);
    ok(
        &mut s,
        json!({"type":"register_ff","register":1,"side":"staff","code":"FF02","qty":1}),
    );
    // 2 品とも「お渡し待ち」
    let v = reg(&s, 1);
    assert_eq!(v.staff.lines.len(), 2);
    assert!(allowed(&v, Side::Staff, Action::HandOverFf));
    assert!(run(
        &mut s,
        json!({"type":"register_ff","register":1,"side":"customer","code":"FF01","qty":1})
    )
    .is_err());
    // お渡しするとお客様の受取確認（お客様側）。確認後は店員側のまま
    ok(&mut s, json!({"type":"hand_over_ff","register":1}));
    assert_eq!(reg(&s, 1).active_side, Side::Customer);
    let e = run(&mut s, json!({"type":"claim","register":1,"side":"staff"})).unwrap_err();
    assert!(e.contains("確認"), "{e}");
    ok(&mut s, json!({"type":"confirm_ff_receipt","register":1}));
    assert_eq!(reg(&s, 1).active_side, Side::Staff);
}
