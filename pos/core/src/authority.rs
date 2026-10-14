//! 制御権モデル（問題3・5への対策）。
//!
//! 旧来機は「最後に触った側に操作権が移る」イベント駆動の制御権を持つと
//! 推測される。ここでは **操作権のトークンそのものを持たない**。
//!
//! - 誰が何をできるかは、(運用モード, 取引フェーズ, 操作する側) から
//!   純粋関数 [`permitted`] で毎回決まる。
//! - スタッフ側はログイン済み・ロック解除状態であれば、どのモード・どの
//!   フェーズでも取引を操作できる（スタッフの操作権は奪われない）。
//! - お客様側にできるのは、運用モードが許す範囲の操作だけ。お客様の操作は
//!   スタッフの操作可否に一切影響しない。
//!
//! UI はこの関数の結果（スナップショットに含まれる）からボタンを描画し、
//! コアは同じ関数で各コマンドを検証する。表示と認可がずれることはない。

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    /// 有人：スタッフが登録・会計操作、お客様は現金投入・カードのみ
    Attended,
    /// セミセルフ：スタッフが登録、お客様が支払方法を選んで支払う
    SemiSelf,
    /// フルセルフ：お客様が登録から支払いまで行う。スタッフは MAT で見守る
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
    /// 支払い中
    Paying,
    /// 釣銭の払い出し・受け取り待ち
    Settling,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StaffState {
    LoggedOut,
    Locked,
    Ready,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Scan,
    OrderFf,
    RemoveLine,
    ReadPointCard,
    Checkout,
    BackToScanning,
    SelectPayment,
    InsertCash,
    TapEmoney,
    TakeChange,
    CallStaff,
    ConfirmAge,
    VoidTransaction,
    CollectChange,
    SetMode,
}

pub const STAFF_ACTIONS: &[Action] = &[
    Action::Scan,
    Action::OrderFf,
    Action::RemoveLine,
    Action::Checkout,
    Action::BackToScanning,
    Action::SelectPayment,
    Action::ConfirmAge,
    Action::VoidTransaction,
    Action::CollectChange,
    Action::SetMode,
];

pub const CUSTOMER_ACTIONS: &[Action] = &[
    Action::Scan,
    Action::OrderFf,
    Action::RemoveLine,
    Action::ReadPointCard,
    Action::Checkout,
    Action::BackToScanning,
    Action::SelectPayment,
    Action::InsertCash,
    Action::TapEmoney,
    Action::TakeChange,
    Action::CallStaff,
];

/// 判定に必要な取引の状態。
#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub mode: Mode,
    pub phase: Phase,
    pub staff: StaffState,
    pub has_lines: bool,
    pub age_pending: bool,
    pub method: Option<PaymentMethod>,
    pub change_presented: bool,
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
        StaffState::LoggedOut => return Err("ログインしていません"),
        StaffState::Locked => return Err("画面ロック中です（PIN で解除）"),
        StaffState::Ready => {}
    }
    let open = matches!(c.phase, Phase::Registering | Phase::Paying);
    match action {
        Action::Scan | Action::OrderFf => require(open, "おつりの受け取り待ちです"),
        Action::RemoveLine | Action::VoidTransaction => {
            require(open, "おつりの受け取り待ちです")?;
            require(c.has_lines, "明細がありません")
        }
        Action::Checkout => {
            require(c.phase == Phase::Registering, "すでにお支払い中です")?;
            require(c.has_lines, "明細がありません")?;
            require(!c.age_pending, "年齢確認が済んでいません")
        }
        Action::BackToScanning | Action::SelectPayment => {
            require(c.phase == Phase::Paying, "お支払い中ではありません")
        }
        Action::ConfirmAge => require(c.age_pending, "年齢確認は不要です"),
        Action::CollectChange => require(c.change_presented, "取り出し口に釣銭はありません"),
        Action::SetMode => Ok(()),
        Action::ReadPointCard
        | Action::InsertCash
        | Action::TapEmoney
        | Action::TakeChange
        | Action::CallStaff => Err("お客様側の操作です"),
    }
}

fn customer(action: Action, c: &Context) -> Result<(), Denial> {
    let self_mode = c.mode == Mode::FullSelf;
    match action {
        Action::Scan | Action::OrderFf => {
            require(self_mode, "係員が登録します")?;
            require(c.phase == Phase::Registering, "お支払い中です")
        }
        Action::RemoveLine => {
            require(self_mode, "係員にお申し付けください")?;
            require(c.phase == Phase::Registering, "お支払い中です")?;
            require(c.has_lines, "明細がありません")
        }
        Action::ReadPointCard => require(
            matches!(c.phase, Phase::Registering | Phase::Paying),
            "お会計後は読み取れません",
        ),
        Action::Checkout => {
            require(self_mode, "係員が会計に進みます")?;
            require(c.phase == Phase::Registering, "すでにお支払い中です")?;
            require(c.has_lines, "商品を登録してください")?;
            require(!c.age_pending, "係員が年齢確認に向かっています")
        }
        Action::BackToScanning => {
            require(self_mode, "係員にお申し付けください")?;
            require(c.phase == Phase::Paying, "お支払い中ではありません")
        }
        Action::SelectPayment => {
            require(c.mode != Mode::Attended, "係員がお支払い方法を選びます")?;
            require(c.phase == Phase::Paying, "お支払い前です")
        }
        Action::InsertCash => {
            require(c.phase == Phase::Paying, "お会計前は投入できません")?;
            require(
                matches!(c.method, None | Some(PaymentMethod::Cash)),
                "現金以外のお支払いが選択されています",
            )
        }
        Action::TapEmoney => {
            require(c.phase == Phase::Paying, "お会計前です")?;
            require(
                matches!(c.method, Some(PaymentMethod::Emoney | PaymentMethod::Qr)),
                "電子マネー/QR が選択されていません",
            )
        }
        Action::TakeChange => require(c.change_presented, "取り出し口に釣銭はありません"),
        Action::CallStaff => require(c.mode != Mode::Attended, "係員が対応中です"),
        Action::ConfirmAge | Action::VoidTransaction | Action::CollectChange | Action::SetMode => {
            Err("係員の操作です")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_contexts() -> Vec<Context> {
        let mut v = vec![];
        for mode in [Mode::Attended, Mode::SemiSelf, Mode::FullSelf] {
            for phase in [Phase::Registering, Phase::Paying, Phase::Settling] {
                for has_lines in [false, true] {
                    for age_pending in [false, true] {
                        for method in [None, Some(PaymentMethod::Cash), Some(PaymentMethod::Emoney)]
                        {
                            for change_presented in [false, true] {
                                v.push(Context {
                                    mode,
                                    phase,
                                    staff: StaffState::Ready,
                                    has_lines,
                                    age_pending,
                                    method,
                                    change_presented,
                                });
                            }
                        }
                    }
                }
            }
        }
        v
    }

    /// 問題5：スタッフの操作可否は運用モードに依存しない
    /// （お客様が何をしてもスタッフの操作権は奪われない）。
    #[test]
    fn staff_authority_does_not_depend_on_mode() {
        for c in all_contexts() {
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
                        "{a:?} {c:?}"
                    );
                }
            }
        }
    }

    /// 問題5：有人モードではお客様は会計・支払方法選択などの「流れを変える操作」ができない。
    #[test]
    fn customer_cannot_steer_attended_register() {
        for c in all_contexts()
            .into_iter()
            .filter(|c| c.mode == Mode::Attended)
        {
            for a in [
                Action::Scan,
                Action::Checkout,
                Action::SelectPayment,
                Action::BackToScanning,
                Action::RemoveLine,
                Action::CallStaff,
            ] {
                assert!(permitted(Side::Customer, a, &c).is_err(), "{a:?} {c:?}");
            }
        }
    }

    /// 問題3：支払い中でもスタッフは常に登録へ戻せる。
    #[test]
    fn staff_can_always_pull_back_from_payment() {
        for c in all_contexts()
            .into_iter()
            .filter(|c| c.phase == Phase::Paying)
        {
            assert!(permitted(Side::Staff, Action::BackToScanning, &c).is_ok());
            assert!(permitted(Side::Staff, Action::Scan, &c).is_ok());
        }
    }

    #[test]
    fn locked_staff_screen_rejects_everything() {
        for c in all_contexts() {
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
