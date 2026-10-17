//! 操作権のモデル（問題3・5への対策）。
//!
//! 店員画面とお客様画面は、内部的には一つの買い物画面（情報空間）を両者が
//! 見ているもの。**画面の操作は排他**で、店員が操作しているあいだお客様画面は
//! 操作できず、お客様が操作しているあいだ店員画面は操作できない。
//!
//! 店員画面には常にお客様画面の内容が映り、店員はログイン（名札のスキャン）
//! することで操作できるようになる。
//!
//! どちらが操作するかは、(運用モード, 取引の段階, 店員のログイン状態,
//! お客様の確認待ちの有無, 画面を持っている側) から純粋関数 [`active_side`] で
//! 一意に決まる。年確・FF 受取などお客様本人の確認が要る場面は必ずお客様側（問題5）。
//! それ以外の有人モードでは、画面を持っている側が操作する。ログイン直後は
//! お客様側が持ち、持っている側がしばらく触られていないときだけ、もう一方が
//! タッチで引き継げる。操作中の画面が勝手に取られることはない（問題3）。
//!
//! スキャナー・現金投入口・電子マネー端末は画面ではない（機器）ので排他の対象外。
//! バーコードはいつでも読め、会計画面で読めば登録画面に戻る。
//!
//! 画面のボタンも [`permitted`] の結果から作り、コアは同じ関数で各操作を検証する。
//! 「画面に見えているから操作できる」という状態は生まれない。

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    /// 有人：店員が登録と会計を行い、お客様は現金投入・カード・確認操作だけ
    Attended,
    /// セミセルフ：店員が登録し、お客様が支払方法を選んで支払う
    SemiSelf,
    /// セルフ：お客様が登録から支払いまで行い、店員は MAT の呼び出しで駆けつける
    FullSelf,
}

impl Mode {
    pub fn label(self) -> &'static str {
        match self {
            Mode::Attended => "有人",
            Mode::SemiSelf => "セミセルフ",
            Mode::FullSelf => "セルフ",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Staff,
    Customer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    /// 商品登録中（明細が空なら待機中）
    Registering,
    /// 会計画面
    Paying,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Cash,
    Emoney,
    Qr,
}

impl PaymentMethod {
    pub fn label(self) -> &'static str {
        match self {
            PaymentMethod::Cash => "現金",
            PaymentMethod::Emoney => "電子マネー",
            PaymentMethod::Qr => "QR決済",
        }
    }
}

/// 店員側の状態。`Ready` のときが「店員操作モード」。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StaffState {
    Absent,
    Locked,
    Ready,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Scan,
    RegisterFf,
    FfButton,
    HandOverFf,
    ConfirmFfReceipt,
    CompleteService,
    RemoveLine,
    Checkout,
    BackToRegistering,
    SelectPayment,
    ConfirmAge,
    InsertCash,
    TapEmoney,
    CallStaff,
    VoidTransaction,
    SetMode,
    Lock,
    EndStaffMode,
    CashOut,
    CashIn,
}

pub const STAFF_ACTIONS: &[Action] = &[
    Action::Scan,
    Action::RegisterFf,
    Action::HandOverFf,
    Action::CompleteService,
    Action::RemoveLine,
    Action::Checkout,
    Action::BackToRegistering,
    Action::SelectPayment,
    Action::VoidTransaction,
    Action::SetMode,
    Action::Lock,
    Action::EndStaffMode,
    Action::CashOut,
    Action::CashIn,
];

pub const CUSTOMER_ACTIONS: &[Action] = &[
    Action::Scan,
    Action::RegisterFf,
    Action::FfButton,
    Action::ConfirmFfReceipt,
    Action::RemoveLine,
    Action::Checkout,
    Action::BackToRegistering,
    Action::SelectPayment,
    Action::ConfirmAge,
    Action::InsertCash,
    Action::TapEmoney,
    Action::CallStaff,
];

/// 判定に必要な取引の状態。
#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub mode: Mode,
    pub phase: Phase,
    pub staff: StaffState,
    pub has_lines: bool,
    /// 会計時の年確がまだ済んでいない
    pub age_pending: bool,
    /// 登録済みでまだお渡ししていない FF がある
    pub ff_waiting: bool,
    /// お渡し済みで、お客様の受取確認待ちの FF がある
    pub ff_presented: bool,
    /// 店員の受付が済んでいないサービス商品がある
    pub service_pending: bool,
    pub method: Option<PaymentMethod>,
    /// セミセルフで、店員が会計に進めてお客様に支払いを任せている
    pub customer_turn: bool,
    /// 店員がお客様画面で店舗メニュー（入出金・両替）を開いている
    pub menu_open: bool,
    /// 有人モードで、いま画面を持っている側（ログイン直後はお客様側。
    /// 持っている側がしばらく触られていなければ、もう一方がタッチで引き継ぐ）
    pub holder: Side,
}

impl Context {
    /// 決済を止めている理由（なければ None）。決済の関門はここに集める。
    pub fn payment_blocker(&self) -> Option<&'static str> {
        if self.age_pending {
            Some("年齢確認が済んでいません")
        } else if self.ff_waiting || self.ff_presented {
            Some("FF のお渡しと受取確認が済んでいません")
        } else if self.service_pending {
            Some("サービス商品の受付が済んでいません")
        } else {
            None
        }
    }
}

pub type Denial = &'static str;

/// 今、画面を操作できる側。もう一方の画面は表示だけになる。
pub fn active_side(c: &Context) -> Side {
    if c.menu_open {
        // 店舗メニューはお客様画面に出す（釣銭機の出し入れ口が売り場側にあるため）
        return Side::Customer;
    }
    if c.staff != StaffState::Ready {
        // 店員が操作できない（未ログイン・ロック中）ならお客様側
        return Side::Customer;
    }
    if c.age_pending || c.ff_presented {
        // お客様本人の確認（年確・FF 受取）を待っている間はお客様側
        return Side::Customer;
    }
    if c.customer_turn {
        // セミセルフで店員が会計に進めた後はお客様の支払い。
        // 店員が名札を読む（ログインする）と店員側に戻る
        return Side::Customer;
    }
    // 有人モードでは、画面を持っている側が操作する（登録・FF はどちらの画面からでもできる）
    c.holder
}

/// 画面ではなく機器の操作（排他の対象外）
pub fn is_device_action(action: Action) -> bool {
    matches!(
        action,
        Action::Scan | Action::InsertCash | Action::TapEmoney
    )
}

pub fn permitted(side: Side, action: Action, c: &Context) -> Result<(), Denial> {
    match side {
        Side::Staff => staff(action, c)?,
        Side::Customer => customer(action, c)?,
    }
    // 店舗メニューの入出金は、店員がお客様画面側で行う
    let menu_op = c.menu_open
        && side == Side::Staff
        && matches!(action, Action::CashIn | Action::CashOut | Action::EndStaffMode);
    if c.menu_open && side == Side::Staff && !menu_op && !is_device_action(action) {
        return Err("お客様画面で店舗メニューを操作中です");
    }
    if !menu_op && !is_device_action(action) && active_side(c) != side {
        return Err(match side {
            Side::Staff => "お客様が操作中です",
            Side::Customer => "係員が操作中です",
        });
    }
    Ok(())
}

fn require(cond: bool, reason: Denial) -> Result<(), Denial> {
    if cond {
        Ok(())
    } else {
        Err(reason)
    }
}

fn staff(action: Action, c: &Context) -> Result<(), Denial> {
    match c.staff {
        StaffState::Absent => return Err("スタッフバーコードでログインしてください"),
        StaffState::Locked => return Err("画面ロック中です（スタッフバーコードで解除）"),
        StaffState::Ready => {}
    }
    let paying = c.phase == Phase::Paying;
    match action {
        Action::Scan
        | Action::RegisterFf
        | Action::SetMode
        | Action::Lock
        | Action::EndStaffMode => Ok(()),
        Action::CashOut => require(!c.has_lines, "取引中は出金操作ができません"),
        Action::CashIn => require(!c.has_lines, "取引中は入金操作ができません"),
        Action::HandOverFf => require(c.ff_waiting, "お渡し待ちの FF はありません"),
        Action::CompleteService => require(c.service_pending, "受付待ちのサービス商品はありません"),
        Action::RemoveLine | Action::VoidTransaction => require(c.has_lines, "明細がありません"),
        Action::Checkout => {
            require(!paying, "すでに会計画面です")?;
            require(c.has_lines, "明細がありません")
        }
        Action::BackToRegistering | Action::SelectPayment => {
            require(paying, "会計画面ではありません")
        }
        Action::FfButton
        | Action::ConfirmFfReceipt
        | Action::ConfirmAge
        | Action::InsertCash
        | Action::TapEmoney
        | Action::CallStaff => Err("お客様側の操作です"),
    }
}

fn customer(action: Action, c: &Context) -> Result<(), Denial> {
    // セミセルフとセルフは、お客様が自分で登録・会計する（違いは FF の自己登録だけ）
    let self_mode = c.mode != Mode::Attended;
    let paying = c.phase == Phase::Paying;
    match action {
        // 会計画面でも受け付け、登録画面に引き返して追加する
        Action::Scan => Ok(()),
        Action::FfButton => {
            require(c.mode == Mode::SemiSelf, "FF は係員にお申し付けください")?;
            require(!paying, "お会計中です")
        }
        // セルフモードではお客様が自分で取った FF を、有人モードでは係員がお客様画面から登録する
        Action::RegisterFf => {
            require(c.mode != Mode::SemiSelf, "FF は係員にお申し付けください")?;
            require(!paying, "お会計中です")
        }
        Action::ConfirmFfReceipt => require(c.ff_presented, "お渡し中の FF はありません"),
        Action::RemoveLine => {
            require(self_mode, "係員にお申し付けください")?;
            require(!paying, "お会計中です")?;
            require(c.has_lines, "明細がありません")
        }
        Action::Checkout => {
            require(self_mode, "係員がお会計に進みます")?;
            require(!paying, "すでにお会計中です")?;
            require(c.has_lines, "商品を登録してください")
        }
        Action::BackToRegistering => {
            require(self_mode, "係員にお申し付けください")?;
            require(paying, "お会計中ではありません")
        }
        Action::SelectPayment => {
            require(c.mode != Mode::Attended, "係員がお支払い方法を選びます")?;
            require(paying, "お会計前です")
        }
        Action::ConfirmAge => {
            require(paying && c.age_pending, "年齢確認は不要です")?;
            require(
                !self_mode || c.staff == StaffState::Ready,
                "係員が向かっています。しばらくお待ちください",
            )
        }
        Action::InsertCash => {
            require(paying, "お会計前は投入できません")?;
            if let Some(r) = c.payment_blocker() {
                return Err(r);
            }
            require(
                matches!(c.method, None | Some(PaymentMethod::Cash)),
                "現金以外のお支払いが選ばれています",
            )
        }
        Action::TapEmoney => {
            require(paying, "お会計前です")?;
            if let Some(r) = c.payment_blocker() {
                return Err(r);
            }
            require(
                matches!(c.method, Some(PaymentMethod::Emoney | PaymentMethod::Qr)),
                "電子マネー/QR が選ばれていません",
            )
        }
        Action::CallStaff => require(c.mode != Mode::Attended, "係員が対応中です"),
        Action::HandOverFf
        | Action::CompleteService
        | Action::VoidTransaction
        | Action::SetMode
        | Action::Lock
        | Action::EndStaffMode
        | Action::CashOut
        | Action::CashIn => Err("係員の操作です"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contexts() -> Vec<Context> {
        let mut v = vec![];
        let b = [false, true];
        for mode in [Mode::Attended, Mode::SemiSelf, Mode::FullSelf] {
            for phase in [Phase::Registering, Phase::Paying] {
                for has_lines in b {
                    for age_pending in b {
                        for (ff_waiting, ff_presented) in
                            [(false, false), (true, false), (false, true), (true, true)]
                        {
                            for service_pending in b {
                                for method in
                                    [None, Some(PaymentMethod::Cash), Some(PaymentMethod::Emoney)]
                                {
                                    for customer_turn in b {
                                        v.push(Context {
                                            menu_open: false,
                                            holder: Side::Staff,
                                            customer_turn,
                                            mode,
                                            phase,
                                            staff: StaffState::Ready,
                                            has_lines,
                                            age_pending: age_pending && phase == Phase::Paying,
                                            ff_waiting,
                                            ff_presented,
                                            service_pending,
                                            method,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        v
    }

    /// 画面の操作は排他：同じ状態で両方の画面が操作できることはない。
    #[test]
    fn screens_are_mutually_exclusive() {
        for c in contexts() {
            let any = |side: Side, list: &[Action]| {
                list.iter()
                    .any(|&a| !is_device_action(a) && permitted(side, a, &c).is_ok())
            };
            assert!(
                !(any(Side::Staff, STAFF_ACTIONS) && any(Side::Customer, CUSTOMER_ACTIONS)),
                "{c:?}"
            );
        }
    }

    /// 問題5：有人モードでお客様が操作側になるのは、本人の確認を待つときだけ。
    #[test]
    fn attended_customer_is_active_only_for_confirmations() {
        for c in contexts()
            .into_iter()
            .filter(|c| c.mode == Mode::Attended && !c.customer_turn)
        {
            let expect = if c.age_pending || c.ff_presented {
                Side::Customer
            } else {
                Side::Staff
            };
            assert_eq!(active_side(&c), expect);
        }
    }

    /// 問題5：有人モードのお客様は、会計・支払方法の選択など流れを変える操作ができない。
    #[test]
    fn customer_cannot_steer_attended_register() {
        for c in contexts().into_iter().filter(|c| c.mode == Mode::Attended) {
            for a in [
                Action::Checkout,
                Action::SelectPayment,
                Action::BackToRegistering,
                Action::RemoveLine,
            ] {
                assert!(permitted(Side::Customer, a, &c).is_err(), "{a:?}");
            }
        }
    }

    /// 年確・FF・サービス受付が済むまで、どのモードでも決済できない。
    #[test]
    fn payment_is_gated_until_blockers_clear() {
        for c in contexts().into_iter().filter(|c| c.phase == Phase::Paying) {
            let blocked = c.age_pending || c.ff_waiting || c.service_pending;
            for a in [Action::InsertCash, Action::TapEmoney] {
                if blocked {
                    assert!(permitted(Side::Customer, a, &c).is_err());
                }
            }
        }
    }

    #[test]
    fn locked_staff_screen_rejects_everything() {
        for c in contexts() {
            let locked = Context {
                staff: StaffState::Locked,
                ..c
            };
            for &a in STAFF_ACTIONS {
                assert!(permitted(Side::Staff, a, &locked).is_err());
            }
        }
    }
}
