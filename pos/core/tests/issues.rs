//! 新型レジで報告された問題ごとに、このシミュレーターがどう振る舞うかを検証する。

use lycoris_pos_core::authority::{Action, Mode, Phase, Side};
use lycoris_pos_core::calls::CallKind;
use lycoris_pos_core::store::{Command, RegisterView, Snapshot, Store};

fn cmd(json: serde_json::Value) -> Command {
    serde_json::from_value(json).expect("valid command")
}

fn run(s: &mut Store, json: serde_json::Value) -> Result<Option<String>, String> {
    s.dispatch(cmd(json)).map(|o| o.message)
}

fn ok(s: &mut Store, json: serde_json::Value) {
    if let Err(e) = run(s, json.clone()) {
        panic!("{json} failed: {e}");
    }
}

fn advance(s: &mut Store, ms: u64) {
    let end = s.now() + ms;
    while s.now() < end {
        let t = (s.now() + 20).min(end);
        s.tick(t);
    }
}

fn reg(snap: &Snapshot, id: u8) -> &RegisterView {
    &snap.registers[usize::from(id) - 1]
}

fn allowed(v: &RegisterView, side: Side, a: Action) -> bool {
    let list = match side {
        Side::Staff => &v.staff_actions,
        Side::Customer => &v.customer_actions,
    };
    list.iter().find(|x| x.action == a).expect("listed").allowed
}

fn mat(n: usize) -> serde_json::Value {
    serde_json::json!({"kind":"mat","mat":n})
}

fn at_register(n: u8) -> serde_json::Value {
    serde_json::json!({"kind":"register","register":n})
}

/// 問題3・5：有人モードでは、お客様側から会計に進めず、スタッフの操作は常に有効。
/// セミセルフで支払い中でも、スタッフは登録・取消・戻しを一切ブロックされない。
#[test]
fn issue_5_customer_never_takes_over_attended_register() {
    let mut s = Store::new(1);
    ok(
        &mut s,
        serde_json::json!({"type":"scan","register":1,"side":"staff","sku":"4900001"}),
    );
    let e = run(
        &mut s,
        serde_json::json!({"type":"checkout","register":1,"side":"customer"}),
    );
    assert!(
        e.is_err(),
        "customer checkout must be rejected in attended mode"
    );
    ok(
        &mut s,
        serde_json::json!({"type":"checkout","register":1,"side":"staff"}),
    );
    // お客様が現金を入れても、スタッフは明細を追加・取消できる
    ok(
        &mut s,
        serde_json::json!({"type":"insert_cash","register":1,"denomination":100}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"scan","register":1,"side":"staff","sku":"4900003"}),
    );
    let snap = s.snapshot();
    let r = reg(&snap, 1);
    assert_eq!(
        r.phase,
        Phase::Registering,
        "staff scan pulls the register back to registering"
    );
    assert_eq!(r.inserted, 100, "inserted cash is kept as credit");
    for a in [
        Action::Scan,
        Action::RemoveLine,
        Action::Checkout,
        Action::VoidTransaction,
    ] {
        assert!(allowed(r, Side::Staff, a), "{a:?}");
    }
}

/// 問題3：セルフモードで在庫切れの FF を、スタッフの 1 操作で取り消せる。
/// お客様は支払い中のまま止められず、残額で支払いが完了する。
#[test]
fn issue_3_out_of_stock_ff_is_one_staff_action() {
    let mut s = Store::new(1);
    ok(
        &mut s,
        serde_json::json!({"type":"scan","register":3,"side":"customer","sku":"4900003"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"order_ff","register":3,"side":"customer","sku":"FF01"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"checkout","register":3,"side":"customer"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"insert_cash","register":3,"denomination":100}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"insert_cash","register":3,"denomination":50}),
    );
    let snap = s.snapshot();
    let call = snap
        .calls
        .iter()
        .find(|c| c.kind == CallKind::FastFood)
        .expect("ff call");
    let line_id = call.ff_items[0].line_id;
    // スタッフは MAT から 1 操作で取り消す（レジ3のスタッフ画面はロック中）
    ok(
        &mut s,
        serde_json::json!({"type":"mark_ff_out_of_stock","call_id":call.id,"line_id":line_id,"by":mat(0)}),
    );
    let snap = s.snapshot();
    let r = reg(&snap, 3);
    // 150円投入済み・残りは緑茶 150円 → 確認なしでそのまま精算が完了する
    assert_eq!(r.phase, Phase::Registering);
    assert!(
        r.thanks.is_some(),
        "payment completed without any extra confirmation"
    );
    assert!(snap.calls.iter().all(|c| c.kind != CallKind::FastFood));
    assert_eq!(
        snap.ff_menu.iter().find(|m| m.sku == "FF01").unwrap().stock,
        Some(0)
    );
}

/// 問題3：在庫 0 の FF はお客様が注文できない（そもそも取り消しが発生しない）。
#[test]
fn issue_3_sold_out_ff_cannot_be_ordered() {
    let mut s = Store::new(1);
    let e = run(
        &mut s,
        serde_json::json!({"type":"order_ff","register":3,"side":"customer","sku":"FF04"}),
    );
    assert!(e.unwrap_err().contains("品切れ"));
}

/// 問題4：ポイントカードは確認ボタンなしで反映される。
#[test]
fn issue_4_point_card_needs_no_confirmation() {
    let mut s = Store::new(1);
    ok(
        &mut s,
        serde_json::json!({"type":"scan","register":1,"side":"staff","sku":"4900001"}),
    );
    let msg = run(
        &mut s,
        serde_json::json!({"type":"read_point_card","register":1,"card_no":"1234-5678-9012"}),
    )
    .unwrap();
    assert!(msg.unwrap().contains("****9012"));
    let snap = s.snapshot();
    assert_eq!(reg(&snap, 1).point_card.as_deref(), Some("****9012"));
    assert_eq!(
        reg(&snap, 1).phase,
        Phase::Registering,
        "no extra step was inserted"
    );
}

/// 問題4：現金を入れるだけで現金払いになり、金額に達した時点で自動で精算される。
#[test]
fn issue_4_cash_payment_needs_no_extra_buttons() {
    let mut s = Store::new(1);
    ok(
        &mut s,
        serde_json::json!({"type":"scan","register":2,"side":"staff","sku":"4900001"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"checkout","register":2,"side":"staff"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"insert_cash","register":2,"denomination":1000}),
    );
    let snap = s.snapshot();
    assert_eq!(reg(&snap, 2).phase, Phase::Settling);
}

/// 問題6：ログイン中・画面表示中でも、責任者の PIN なしでは返金・緊急出金はできない。
#[test]
fn issue_6_privileged_operations_require_manager_pin() {
    let mut s = Store::new(1);
    ok(
        &mut s,
        serde_json::json!({"type":"scan","register":1,"side":"staff","sku":"4900001"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"checkout","register":1,"side":"staff"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"insert_cash","register":1,"denomination":1000}),
    );
    advance(&mut s, 5_000);
    ok(
        &mut s,
        serde_json::json!({"type":"take_change","register":1}),
    );
    let receipt = s.snapshot().receipts[0].id;

    // ログイン中のクルー自身の PIN では通らない
    let e = run(
        &mut s,
        serde_json::json!({"type":"refund","register":1,"receipt_id":receipt,"manager_id":"1001","pin":"1111"}),
    );
    assert!(e.is_err());
    // PIN 空欄・推測でも通らない
    for pin in ["", "0000"] {
        let e = run(
            &mut s,
            serde_json::json!({"type":"emergency_withdrawal","register":1,"amount":10000,"manager_id":"9001","pin":pin}),
        );
        assert!(e.is_err());
    }
    let snap = s.snapshot();
    assert!(
        snap.audit.iter().all(|a| !a.ok),
        "denied attempts are audited"
    );
    assert_eq!(snap.audit.len(), 3);
    // 責任者 PIN なら通る
    ok(
        &mut s,
        serde_json::json!({"type":"refund","register":1,"receipt_id":receipt,"manager_id":"9001","pin":"9999"}),
    );
    assert!(s.snapshot().receipts[0].refunded);
}

/// 問題6：ロック中のスタッフ画面からは何もできない（部外者が身を乗り出しても無効）。
#[test]
fn issue_6_locked_screen_rejects_staff_operations() {
    let mut s = Store::new(1);
    // レジ3は初期状態でロック中
    let e = run(
        &mut s,
        serde_json::json!({"type":"scan","register":3,"side":"staff","sku":"4900001"}),
    );
    assert!(e.unwrap_err().contains("ロック"));
    let e = run(
        &mut s,
        serde_json::json!({"type":"set_mode","register":3,"mode":"attended"}),
    );
    assert!(e.is_err());
}

/// 問題7：年確の要求は、モードや支払方法に関係なく「スキャンした瞬間」に必ず立つ。
#[test]
fn issue_7_age_check_always_triggers_at_scan_time() {
    for (register, side) in [(1u8, "staff"), (2, "staff"), (3, "customer")] {
        let mut s = Store::new(1);
        ok(
            &mut s,
            serde_json::json!({"type":"scan","register":register,"side":side,"sku":"4900010"}),
        );
        let snap = s.snapshot();
        let r = reg(&snap, register);
        assert_eq!(r.age, "pending", "register {register}");
        assert!(snap
            .calls
            .iter()
            .any(|c| c.kind == CallKind::AgeCheck && c.register == register));
        // 会計は年確が済むまで進めない（会計時に新たな判定は起きない）
        assert!(!allowed(
            r,
            if side == "staff" {
                Side::Staff
            } else {
                Side::Customer
            },
            Action::Checkout
        ));
    }
}

/// 問題7：「来たよ」と「年確」は MAT の 1 操作にまとまる。2 本目の酒では再要求しない。
#[test]
fn issue_7_single_action_confirms_age() {
    let mut s = Store::new(1);
    ok(
        &mut s,
        serde_json::json!({"type":"scan","register":3,"side":"customer","sku":"4900010"}),
    );
    let call = s.snapshot().calls[0].id;
    ok(
        &mut s,
        serde_json::json!({"type":"resolve_call","call_id":call,"by":mat(0)}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"scan","register":3,"side":"customer","sku":"4900011"}),
    );
    let snap = s.snapshot();
    assert_eq!(reg(&snap, 3).age, "verified");
    assert!(snap.calls.is_empty());
    assert!(allowed(reg(&snap, 3), Side::Customer, Action::Checkout));
}

/// 問題8：呼び出しは発生したレジにだけ鳴動し、互いをブロックしない。
#[test]
fn issue_8_calls_are_per_register_and_non_blocking() {
    let mut s = Store::new(1);
    ok(
        &mut s,
        serde_json::json!({"type":"order_ff","register":3,"side":"customer","sku":"FF01"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"set_mode","register":2,"mode":"full_self"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"call_staff","register":2}),
    );
    let snap = s.snapshot();
    assert!(reg(&snap, 3).alerting);
    assert!(reg(&snap, 2).alerting);
    assert!(
        !reg(&snap, 1).alerting,
        "register 1 is not interrupted by other registers"
    );
    // レジ3の FF が未完了のまま、レジ2の呼び出しを先に完了できる
    let help = snap
        .calls
        .iter()
        .find(|c| c.kind == CallKind::CustomerHelp)
        .unwrap()
        .id;
    ok(
        &mut s,
        serde_json::json!({"type":"resolve_call","call_id":help,"by":mat(1)}),
    );
    let snap = s.snapshot();
    assert_eq!(snap.calls.len(), 1);
    assert_eq!(snap.calls[0].kind, CallKind::FastFood);
}

/// 問題9：FF の呼び出しには、何を何個渡すかが含まれる。
#[test]
fn issue_9_ff_call_lists_items() {
    let mut s = Store::new(1);
    for sku in ["FF01", "FF01", "FF03"] {
        ok(
            &mut s,
            serde_json::json!({"type":"order_ff","register":3,"side":"customer","sku":sku}),
        );
    }
    let snap = s.snapshot();
    let call = &snap.calls[0];
    assert_eq!(call.ff_items.len(), 2);
    assert_eq!(call.ff_items[0].name, "フライドチキン");
    assert_eq!(call.ff_items[0].qty, 2);
    assert_eq!(call.detail, "フライドチキン×2、アメリカンドッグ×1");
    // 1 品ずつ渡し済みにできる
    let first = call.ff_items[0].line_id;
    ok(
        &mut s,
        serde_json::json!({"type":"hand_over_ff","call_id":call.id,"line_id":first,"by":mat(0)}),
    );
    let snap = s.snapshot();
    assert_eq!(snap.calls[0].detail, "アメリカンドッグ×1");
    // スタッフ画面（他レジ）からは「その場の作業」はできない
    let e = run(
        &mut s,
        serde_json::json!({"type":"hand_over_ff","call_id":snap.calls[0].id,"line_id":null,"by":at_register(1)}),
    );
    assert!(e.is_err());
}

/// 問題10：通信ロスがあっても、両方の MAT が必ず最新の呼び出しに追いつく。
/// 届かない MAT は「不通」として検知される。
#[test]
fn issue_10_mats_converge_under_packet_loss() {
    let mut s = Store::new(42);
    ok(
        &mut s,
        serde_json::json!({"type":"set_fault","fault":{"kind":"mat_loss","percent":60}}),
    );
    advance(&mut s, 1_000);
    ok(
        &mut s,
        serde_json::json!({"type":"order_ff","register":3,"side":"customer","sku":"FF02"}),
    );
    advance(&mut s, 10_000);
    let snap = s.snapshot();
    for m in &snap.mats {
        assert_eq!(m.view_seq, snap.call_seq, "MAT-{} is up to date", m.id);
        assert_eq!(m.view.len(), 1);
    }
    assert!(
        snap.mats.iter().any(|m| m.retries > 0),
        "loss was recovered by retries"
    );

    ok(
        &mut s,
        serde_json::json!({"type":"set_fault","fault":{"kind":"mat_loss","percent":0}}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"set_fault","fault":{"kind":"mat_offline","mat":1,"offline":true}}),
    );
    advance(&mut s, 7_000);
    let snap = s.snapshot();
    assert!(snap.mats[0].link_ok);
    assert!(
        !snap.mats[1].link_ok,
        "an unreachable MAT is detected, not silently ignored"
    );
}

/// 問題11：紙幣保持センサーの異常時は、放出せず回収・再排出する。
/// 問題12：紙幣と硬貨は両方そろってから同時に提示する。
#[test]
fn issues_11_12_change_is_presented_together_and_never_scattered() {
    let mut s = Store::new(1);
    ok(
        &mut s,
        serde_json::json!({"type":"set_fault","fault":{"kind":"coin_delay","enabled":true}}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"set_fault","fault":{"kind":"bill_glitch","enabled":true}}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"scan","register":1,"side":"staff","sku":"4900001"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"checkout","register":1,"side":"staff"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"insert_cash","register":1,"denomination":5000}),
    );
    // 釣銭 4,840円：紙幣 4,000円 + 硬貨 840円
    let mut presented_during_staging = false;
    for _ in 0..500 {
        advance(&mut s, 20);
        let snap = s.snapshot();
        let v = serde_json::to_value(&reg(&snap, 1).dispenser).unwrap();
        if v["state"] == "presented" {
            assert_eq!(v["bills"].as_array().unwrap().len(), 1);
            assert!(!v["coins"].as_array().unwrap().is_empty());
            presented_during_staging = true;
            break;
        }
    }
    assert!(presented_during_staging);
    let snap = s.snapshot();
    assert!(snap
        .events
        .iter()
        .any(|e| e.text.contains("回収して再排出")));
    // 取り忘れは呼び出しになる
    advance(&mut s, 16_000);
    let snap = s.snapshot();
    assert!(snap
        .calls
        .iter()
        .any(|c| c.kind == CallKind::ChangeNotTaken && c.alert));
    ok(
        &mut s,
        serde_json::json!({"type":"take_change","register":1}),
    );
    assert!(s.snapshot().calls.is_empty());
}

/// スタッフはセルフモードのレジでも、ロック解除すれば即座に操作できる。
#[test]
fn staff_can_take_over_self_register_after_unlock() {
    let mut s = Store::new(1);
    ok(
        &mut s,
        serde_json::json!({"type":"scan","register":3,"side":"customer","sku":"4900001"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"checkout","register":3,"side":"customer"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"unlock","register":3,"staff_id":"1001","pin":"1111"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"back_to_scanning","register":3,"side":"staff"}),
    );
    let snap = s.snapshot();
    assert_eq!(reg(&snap, 3).phase, Phase::Registering);
    assert!(
        reg(&snap, 3).customer_notice.is_some(),
        "customer is told what happened"
    );
    assert_eq!(reg(&snap, 3).mode, Mode::FullSelf);
}

/// 取引中止では投入済みの現金が返却され、FF 在庫も戻る。
#[test]
fn void_returns_cash_and_restores_stock() {
    let mut s = Store::new(1);
    ok(
        &mut s,
        serde_json::json!({"type":"order_ff","register":1,"side":"staff","sku":"FF02"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"checkout","register":1,"side":"staff"}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"insert_cash","register":1,"denomination":100}),
    );
    ok(
        &mut s,
        serde_json::json!({"type":"void_transaction","register":1}),
    );
    let snap = s.snapshot();
    assert_eq!(reg(&snap, 1).phase, Phase::Settling);
    assert_eq!(
        snap.ff_menu.iter().find(|m| m.sku == "FF02").unwrap().stock,
        Some(2)
    );
    assert!(snap.calls.is_empty());
    advance(&mut s, 3_000);
    ok(
        &mut s,
        serde_json::json!({"type":"take_change","register":1}),
    );
    let snap = s.snapshot();
    assert!(reg(&snap, 1).thanks.is_none(), "a void issues no receipt");
    assert!(snap.receipts.is_empty());
}

#[test]
fn json_boundary_reports_errors_without_panicking() {
    let mut s = Store::new(1);
    let out = lycoris_pos_core::dispatch_json(&mut s, "{not json");
    assert!(out.contains("\"ok\":false"));
    let out = lycoris_pos_core::dispatch_json(
        &mut s,
        r#"{"type":"scan","register":9,"side":"staff","sku":"4900001"}"#,
    );
    assert!(out.contains("存在しません"));
    let _: serde_json::Value = serde_json::from_str(&lycoris_pos_core::snapshot_json(&s)).unwrap();
}
