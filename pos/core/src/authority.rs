//! 操作権のモデル（問題3・5への対策）。
//!
//! 操作権を受け渡す仕組み（トークン）そのものを持たない。誰が何をできるかは
//! (運用モード, 取引の段階, どちら側の画面か, 取引の状態) から純粋関数
//! [`permitted`] で毎回決まる。
//!
//! - 店員側はログイン済み・ロック解除中であれば、どのモード・どの段階でも
//!   取引を操作できる。お客様の操作で店員が締め出されることはない。
//! - お客様側にできるのは、運用モードが許す範囲の操作だけ。
//! - 画面のボタンもこの関数の結果から作り、コアは同じ関数で各操作を検証する。
//!   「画面に見えているから操作できる」という状態は生まれない。

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
];

pub const CUSTOMER_ACTIONS: &[Action] = &[
    Action::Scan,
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

pub fn permitted(side: Side, action: Action, c: &Context) -> Result<(), Denial> {
    match side {
        Side::Staff => staff(action, c),
        Side::Customer => customer(action, c),
    }
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
    let self_mode = c.mode == Mode::FullSelf;
    let paying = c.phase == Phase::Paying;
    match action {
        // 会計画面でも受け付け、登録画面に引き返して追加する
        Action::Scan => Ok(()),
        Action::FfButton => {
            require(self_mode, "FF は係員にお申し付けください")?;
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
        Action::RegisterFf
        | Action::HandOverFf
        | Action::CompleteService
        | Action::VoidTransaction
        | Action::SetMode
        | Action::Lock
        | Action::EndStaffMode
        | Action::CashOut => Err("係員の操作です"),
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
                        for ff_waiting in b {
                            for service_pending in b {
                                for method in
                                    [None, Some(PaymentMethod::Cash), Some(PaymentMethod::Emoney)]
                                {
                                    v.push(Context {
                                        mode,
                                        phase,
                                        staff: StaffState::Ready,
                                        has_lines,
                                        age_pending,
                                        ff_waiting,
                                        ff_presented: false,
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
        v
    }

    /// 問題5：店員の操作可否は運用モードに左右されない。
    #[test]
    fn staff_authority_does_not_depend_on_mode() {
        for c in contexts() {
            for &a in STAFF_ACTIONS {
                let base = permitted(
                    Side::Staff,
                    a,
                    &Context {
                        mode: Mode::Attended,
                        ..c
                    },
                );
                for mode in [Mode::SemiSelf, Mode::FullSelf] {
                    assert_eq!(
                        permitted(Side::Staff, a, &Context { mode, ..c }),
                        base,
                        "{a:?}"
                    );
                }
            }
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
