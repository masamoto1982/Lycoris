//! 新型レジで報告された問題と、壁打ちで決めた仕様を、シナリオで検証する。

use lycoris_pos_core::authority::{Action, Mode, Phase, Side, StaffState};
use lycoris_pos_core::calls::CallKind;
use lycoris_pos_core::store::{Command, RegisterView, Store};
use serde_json::{json, Value};

const ONIGIRI: &str = "4900000000011";
const BEER: &str = "4900000000103";
const LIGHTER: &str = "4900000000134";
const LOTTERY: &str = "4900000000158";
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

/// セルフでは年確が店員呼び出しになり、店員が来るまでお客様は確認できない。
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
    // 用件が片付いたので店員操作モードは自動で終わる
    assert_eq!(reg(&s, 2).staff.state, StaffState::Absent);
    assert!(s.snapshot().calls.is_empty());
}

/// 店舗ルール：オーナーだけが追加でき、法令ルールは外せない。
#[test]
fn only_owner_changes_age_rules() {
    let mut s = Store::new(1, 2, 1);
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
    ok(
        &mut s,
        json!({"type":"set_mode","register":1,"mode":"attended"}),
    );
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

/// FF：FF ボタンで呼び出し → 店員操作モードで登録 → お渡し → お客様の受取確認。
#[test]
fn fast_food_flow_in_self_mode() {
    let mut s = Store::new(1, 2, 1);
    scan(&mut s, 2, "customer", ONIGIRI);
    assert!(run(
        &mut s,
        json!({"type":"register_ff","register":2,"code":"FF01","qty":1})
    )
    .is_err());
    ok(&mut s, json!({"type":"ff_button","register":2}));
    scan(&mut s, 2, "staff", "ST1001");
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
    ok(
        &mut s,
        json!({"type":"checkout","register":2,"side":"customer"}),
    );
    assert!(run(&mut s, json!({"type":"tap_emoney","register":2})).is_err());
    ok(&mut s, json!({"type":"hand_over_ff","register":2}));
    assert_eq!(
        reg(&s, 2).staff.state,
        StaffState::Absent,
        "staff mode ends at hand-over"
    );
    ok(&mut s, json!({"type":"confirm_ff_receipt","register":2}));
    ok(
        &mut s,
        json!({"type":"select_payment","register":2,"side":"customer","method":"emoney"}),
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
    scan(&mut s, 2, "staff", "ST1001");
    let id = reg(&s, 2).staff.lines[0].line.id;
    ok(
        &mut s,
        json!({"type":"complete_service","register":2,"line_id":id,"amount":1200}),
    );
    assert_eq!(reg(&s, 2).staff.state, StaffState::Absent);
    assert_eq!(reg(&s, 2).customer.total, 1200);
}

/// 問題6：出金は毎回、ログインとは別の出金パスワードが要り、記録が残る。
#[test]
fn cash_out_requires_separate_password_every_time() {
    let mut s = Store::new(1, 2, 1);
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
    scan(&mut s, 4, "staff", "ST1002");
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
    scan(&mut s, 1, "staff", BEER);
    scan(&mut s, 1, "staff", "PC12345678");
    let v = serde_json::to_value(reg(&s, 1)).unwrap();
    let c = v["customer"].to_string();
    assert!(!c.contains("alcohol") && !c.contains(BEER) && !c.contains("12345678"));
    assert_eq!(v["staff"]["lines"][0]["age"], 20);
    assert_eq!(reg(&s, 1).mode, Mode::SemiSelf);
}
