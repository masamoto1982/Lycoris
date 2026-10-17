//! 店舗全体（レジ・MAT・共有設定）の状態と、コマンド処理。
//!
//! レジ 1 台につき情報空間は 1 つだけ。店員画面とお客様画面は、同じ取引を
//! 別の切り口で見せる「ビュー」にすぎない（問題2）。お客様ビューには公開してよい
//! 情報だけを載せる。画面が 1 枚の機種になっても、ビューの出し分けを変えるだけで済む。

use serde::{Deserialize, Serialize};

use crate::age_rules::{AgeRules, RuleTarget};
use crate::auth::{self, Authenticator, Role, ACCOUNTS};
use crate::authority::{
    active_side, permitted, Action, Context, Mode, PaymentMethod, Phase, Side, StaffState,
    CUSTOMER_ACTIONS, STAFF_ACTIONS,
};
use crate::calls::{Call, CallKind, Calls, ResolvedCall};
use crate::cash::{is_valid_denomination, Cash, Drawer};
use crate::catalog::{self, Category, Product, Scanned, TaxRate, CATALOG, INITIAL_FF_STOCK};
use crate::mat::{Bell, Mat, RERING_MS};
use crate::rng::Rng;

pub const MAX_REGISTERS: u8 = 10;
pub const MAX_MATS: u8 = 5;

// ---------------------------------------------------------------------------
// コマンド
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CashOutKind {
    /// 返品返金
    Refund,
    /// 経費支出
    Expense,
    /// 緊急出金
    Emergency,
    /// 途中集金
    Pickup,
    /// 自動釣銭機からの回収
    ChangerCollect,
}

/// 取引外の入金（お金を入れる操作はパスワード不要）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CashInKind {
    /// 取引外入金
    Deposit,
    /// 自動釣銭機への釣銭補充
    ChangerRefill,
}

impl CashInKind {
    pub fn label(self) -> &'static str {
        match self {
            CashInKind::Deposit => "取引外入金",
            CashInKind::ChangerRefill => "釣銭補充",
        }
    }
}

impl CashOutKind {
    pub fn label(self) -> &'static str {
        match self {
            CashOutKind::Refund => "返品返金",
            CashOutKind::Expense => "経費支出",
            CashOutKind::Emergency => "緊急出金",
            CashOutKind::Pickup => "途中集金",
            CashOutKind::ChangerCollect => "釣銭機回収",
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    Scan {
        register: u8,
        side: Side,
        code: String,
    },
    RegisterFf {
        register: u8,
        /// 省略時は店員側。セルフモードではお客様側からも登録できる
        #[serde(default = "staff_side")]
        side: Side,
        code: String,
        qty: u32,
    },
    FfButton {
        register: u8,
    },
    HandOverFf {
        register: u8,
    },
    ConfirmFfReceipt {
        register: u8,
    },
    CompleteService {
        register: u8,
        line_id: u32,
        amount: u32,
    },
    RemoveLine {
        register: u8,
        side: Side,
        line_id: u32,
    },
    Checkout {
        register: u8,
        side: Side,
    },
    BackToRegistering {
        register: u8,
        side: Side,
    },
    SelectPayment {
        register: u8,
        side: Side,
        method: PaymentMethod,
    },
    ConfirmAge {
        register: u8,
    },
    InsertCash {
        register: u8,
        denomination: u32,
        count: u32,
    },
    TapEmoney {
        register: u8,
    },
    CallStaff {
        register: u8,
    },
    VoidTransaction {
        register: u8,
    },
    SetMode {
        register: u8,
        mode: Mode,
    },
    Lock {
        register: u8,
    },
    EndStaffMode {
        register: u8,
    },
    CashOut {
        register: u8,
        kind: CashOutKind,
        amount: u32,
        password: String,
    },
    CashIn {
        register: u8,
        kind: CashInKind,
        amount: u32,
    },
    AddAgeRule {
        owner_id: String,
        password: String,
        target: RuleTarget,
        min_age: u8,
    },
    RemoveAgeRule {
        owner_id: String,
        password: String,
        rule_id: u32,
    },
    SetProductAge {
        owner_id: String,
        password: String,
        code: String,
        min_age: Option<u8>,
    },
    SetMatLoss {
        percent: u8,
    },
    SetMatOffline {
        mat: usize,
        offline: bool,
    },
}

impl Command {
    /// 対象のレジ（店舗全体の操作なら None）
    pub fn register(&self) -> Option<u8> {
        use Command as C;
        match self {
            C::Scan { register, .. }
            | C::RegisterFf { register, .. }
            | C::FfButton { register, .. }
            | C::HandOverFf { register, .. }
            | C::ConfirmFfReceipt { register, .. }
            | C::CompleteService { register, .. }
            | C::RemoveLine { register, .. }
            | C::Checkout { register, .. }
            | C::BackToRegistering { register, .. }
            | C::SelectPayment { register, .. }
            | C::ConfirmAge { register, .. }
            | C::InsertCash { register, .. }
            | C::TapEmoney { register, .. }
            | C::CallStaff { register, .. }
            | C::VoidTransaction { register, .. }
            | C::SetMode { register, .. }
            | C::Lock { register, .. }
            | C::EndStaffMode { register, .. }
            | C::CashOut { register, .. }
            | C::CashIn { register, .. } => Some(*register),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Outcome {
    pub message: Option<String>,
}

fn done(msg: impl Into<String>) -> Result<Outcome, String> {
    Ok(Outcome {
        message: Some(msg.into()),
    })
}

// ---------------------------------------------------------------------------
// 取引
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FfState {
    /// 登録済み・お渡し前
    Waiting,
    /// お渡し済み・お客様の受取確認待ち
    Presented,
    /// 受取確認済み
    Received,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LineKind {
    Item,
    Ff { state: FfState },
    Service { done: bool },
}

#[derive(Clone, Debug, Serialize)]
pub struct Line {
    pub id: u32,
    pub code: &'static str,
    pub name: &'static str,
    pub unit_price: u32,
    pub qty: u32,
    pub tax: TaxRate,
    pub category: Category,
    #[serde(flatten)]
    pub kind: LineKind,
}

impl Line {
    pub fn amount(&self) -> u32 {
        self.unit_price * self.qty
    }
}

#[derive(Clone, Debug, Default)]
struct Txn {
    lines: Vec<Line>,
    next_line: u32,
    paying: bool,
    point_card: Option<String>,
    /// 会計ボタンの時点で求められた年齢（年確不要なら None）
    required_age: Option<u8>,
    age_verified: bool,
    method: Option<PaymentMethod>,
    inserted: Cash,
    /// セミセルフで店員が会計に進め、お客様に支払いを任せている（店員が名札を読むと解除）
    customer_turn: bool,
}

impl Txn {
    fn total(&self) -> u32 {
        self.lines.iter().map(Line::amount).sum()
    }

    fn tax_of(&self, rate: TaxRate) -> u32 {
        let gross: u32 = self
            .lines
            .iter()
            .filter(|l| l.tax == rate)
            .map(Line::amount)
            .sum();
        match rate {
            TaxRate::Reduced8 => gross * 8 / 108,
            TaxRate::Standard10 => gross * 10 / 110,
        }
    }

    fn any_ff(&self, s: FfState) -> bool {
        self.lines
            .iter()
            .any(|l| matches!(l.kind, LineKind::Ff { state } if state == s))
    }

    fn service_pending(&self) -> bool {
        self.lines
            .iter()
            .any(|l| matches!(l.kind, LineKind::Service { done: false }))
    }

    fn age_pending(&self) -> bool {
        self.paying && self.required_age.is_some() && !self.age_verified
    }

    fn push(&mut self, p: &'static Product, qty: u32, kind: LineKind) {
        // 同じ商品はまとめる（FF は状態が同じお渡し前どうしだけ）
        if let Some(l) = self.lines.iter_mut().find(|l| {
            l.code == p.code && l.kind == kind && !matches!(kind, LineKind::Service { .. })
        }) {
            l.qty += qty;
            return;
        }
        self.next_line += 1;
        self.lines.push(Line {
            id: self.next_line,
            code: p.code,
            name: p.name,
            unit_price: p.price,
            qty,
            tax: p.tax,
            category: p.category,
            kind,
        });
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Receipt {
    pub no: u32,
    pub at: u64,
    pub total: u32,
    pub method: &'static str,
    pub paid: u32,
    pub change: Cash,
    pub change_total: u32,
    pub items: u32,
    pub point_card: Option<String>,
    pub points: u32,
}

#[derive(Clone, Copy, Debug)]
struct StaffSession {
    account: &'static auth::StaffAccount,
    locked: bool,
}

#[derive(Clone, Debug)]
struct Register {
    id: u8,
    mode: Mode,
    staff: Option<StaffSession>,
    txn: Txn,
    drawer: Drawer,
    last_receipt: Option<Receipt>,
    /// 最後に操作された時刻（無操作でセミセルフに戻すため）
    last_op: u64,
}

impl Register {
    /// 運用モードは名札で決まる：名札でログイン中なら有人、いなければセミセルフ。
    /// セルフ（将来の拡張）は、有効なときに明示的に選んだ場合だけ。
    fn sync_mode(&mut self, self_mode_enabled: bool) {
        self.mode = if self.staff.is_some() {
            Mode::Attended
        } else if self_mode_enabled && self.mode == Mode::FullSelf {
            Mode::FullSelf
        } else {
            Mode::SemiSelf
        };
    }

    /// 取引も呼び出しもなく、待機に戻してよい状態か
    fn idle(&self) -> bool {
        self.txn.lines.is_empty() && !self.txn.paying
    }

    fn staff_state(&self) -> StaffState {
        match self.staff {
            None => StaffState::Absent,
            Some(s) if s.locked => StaffState::Locked,
            Some(_) => StaffState::Ready,
        }
    }

    fn context(&self) -> Context {
        let t = &self.txn;
        Context {
            mode: self.mode,
            phase: if t.paying {
                Phase::Paying
            } else {
                Phase::Registering
            },
            staff: self.staff_state(),
            has_lines: !t.lines.is_empty(),
            age_pending: t.age_pending(),
            ff_waiting: t.any_ff(FfState::Waiting),
            ff_presented: t.any_ff(FfState::Presented),
            service_pending: t.service_pending(),
            method: t.method,
            customer_turn: t.customer_turn,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AuditEntry {
    pub at: u64,
    pub register: u8,
    pub staff: &'static str,
    pub operation: &'static str,
    pub amount: u32,
    pub result: String,
}

// ---------------------------------------------------------------------------
// 店舗
// ---------------------------------------------------------------------------

pub struct Store {
    now: u64,
    /// セルフモード（お客様が FF も自分で登録する）は将来の拡張用。既定は OFF
    self_mode_enabled: bool,
    rng: Rng,
    registers: Vec<Register>,
    mats: Vec<Mat>,
    calls: Calls,
    age_rules: AgeRules,
    auth: Authenticator,
    ff_stock: Vec<(&'static str, u32)>,
    audit: Vec<AuditEntry>,
    mat_loss: u8,
    bell: Bell,
    seen_raised: u64,
    last_ring_at: u64,
    receipts: u32,
}

fn staff_side() -> Side {
    Side::Staff
}

/// 無操作がこの時間続くと、レジはセミセルフモードに戻って待機する
pub const IDLE_REVERT_MS: u64 = 5 * 60 * 1000;

impl Store {
    /// 基本構成はレジ 2 台・MAT 1 台。レジは 10 台、MAT は 5 台まで増やせる。
    pub fn new(seed: u64, registers: u8, mats: u8) -> Store {
        let registers = registers.clamp(1, MAX_REGISTERS);
        let mats = mats.clamp(1, MAX_MATS);
        let regs = (1..=registers)
            .map(|id| {
                // 既定はセミセルフ（名札なし）
                Register {
                    id,
                    mode: Mode::SemiSelf,
                    staff: None,
                    txn: Txn::default(),
                    drawer: Drawer::default(),
                    last_receipt: None,
                    last_op: 0,
                }
            })
            .collect();
        Store {
            now: 0,
            self_mode_enabled: false,
            rng: Rng::new(seed),
            registers: regs,
            mats: (1..=mats).map(|i| Mat::new(&format!("MAT-{i}"))).collect(),
            calls: Calls::default(),
            age_rules: AgeRules::default(),
            auth: Authenticator::default(),
            ff_stock: INITIAL_FF_STOCK.to_vec(),
            audit: vec![],
            mat_loss: 0,
            bell: Bell::default(),
            seen_raised: 0,
            last_ring_at: 0,
            receipts: 0,
        }
    }

    pub fn now(&self) -> u64 {
        self.now
    }

    // ---- 時間の経過：MAT の鳴動状態の同期 --------------------------------

    pub fn tick(&mut self, now: u64) {
        self.now = self.now.max(now);
        let now = self.now;
        // 「未対応」の呼び出し＝そのレジに店員がまだ来ていないもの
        let open = self
            .calls
            .open
            .iter()
            .filter(|c| {
                self.reg(c.register)
                    .is_some_and(|r| r.staff_state() == StaffState::Absent)
            })
            .count();
        let mut bell = self.bell;
        bell.open_calls = open;
        if self.calls.raised > self.seen_raised {
            self.seen_raised = self.calls.raised;
            bell.ring_token += 1;
            self.last_ring_at = now;
        } else if open > 0 && now.saturating_sub(self.last_ring_at) >= RERING_MS {
            bell.ring_token += 1;
            self.last_ring_at = now;
        }
        if bell != self.bell {
            bell.seq += 1;
            self.bell = bell;
            for m in &mut self.mats {
                m.kick();
            }
        }
        for m in &mut self.mats {
            m.tick(now, self.bell, self.mat_loss, &mut self.rng);
        }
        // 無操作が 5 分続いたら、名札のログインを外してセミセルフに戻る
        for r in &mut self.registers {
            let open = self.calls.open.iter().any(|c| c.register == r.id);
            if r.mode != Mode::SemiSelf
                && r.idle()
                && !open
                && now.saturating_sub(r.last_op) >= IDLE_REVERT_MS
            {
                r.staff = None;
                r.mode = Mode::SemiSelf;
                r.last_op = now;
            }
        }
    }

    // ---- コマンド --------------------------------------------------------

    fn reg(&self, id: u8) -> Option<&Register> {
        self.registers.iter().find(|r| r.id == id)
    }

    fn reg_mut(&mut self, id: u8) -> Result<&mut Register, String> {
        self.registers
            .iter_mut()
            .find(|r| r.id == id)
            .ok_or_else(|| format!("レジ{id}はありません"))
    }

    fn check(&self, register: u8, side: Side, action: Action) -> Result<(), String> {
        let r = self
            .reg(register)
            .ok_or_else(|| format!("レジ{register}はありません"))?;
        permitted(side, action, &r.context()).map_err(String::from)
    }

    pub fn dispatch(&mut self, cmd: Command) -> Result<Outcome, String> {
        // 成否にかかわらず、触られたレジは「操作あり」
        if let Some(id) = cmd.register() {
            let now = self.now;
            if let Ok(r) = self.reg_mut(id) {
                r.last_op = now;
            }
        }
        let res = self.apply(cmd);
        let self_ok = self.self_mode_enabled;
        for r in &mut self.registers {
            r.sync_mode(self_ok);
        }
        res
    }

    /// セルフモード（将来の拡張）を使えるようにする
    pub fn set_self_mode_enabled(&mut self, on: bool) {
        self.self_mode_enabled = on;
        for r in &mut self.registers {
            r.sync_mode(on);
        }
    }

    fn apply(&mut self, cmd: Command) -> Result<Outcome, String> {
        use Command as C;
        match cmd {
            C::Scan {
                register,
                side,
                code,
            } => self.scan(register, side, &code),
            C::RegisterFf {
                register,
                side,
                code,
                qty,
            } => self.register_ff(register, side, &code, qty),
            C::FfButton { register } => {
                self.check(register, Side::Customer, Action::FfButton)?;
                self.call(register, CallKind::FastFood, "FF のご注文");
                done("係員をお呼びしました。FF のご注文を伺います")
            }
            C::HandOverFf { register } => {
                self.check(register, Side::Staff, Action::HandOverFf)?;
                let r = self.reg_mut(register)?;
                for l in &mut r.txn.lines {
                    if l.kind
                        == (LineKind::Ff {
                            state: FfState::Waiting,
                        })
                    {
                        l.kind = LineKind::Ff {
                            state: FfState::Presented,
                        };
                    }
                }
                done("お渡ししました。お客様画面で受取確認を待っています")
            }
            C::ConfirmFfReceipt { register } => {
                self.check(register, Side::Customer, Action::ConfirmFfReceipt)?;
                let r = self.reg_mut(register)?;
                for l in &mut r.txn.lines {
                    if l.kind
                        == (LineKind::Ff {
                            state: FfState::Presented,
                        })
                    {
                        l.kind = LineKind::Ff {
                            state: FfState::Received,
                        };
                    }
                }
                done("FF の受け取りを確認しました")
            }
            C::CompleteService {
                register,
                line_id,
                amount,
            } => {
                self.check(register, Side::Staff, Action::CompleteService)?;
                let r = self.reg_mut(register)?;
                let l = r
                    .txn
                    .lines
                    .iter_mut()
                    .find(|l| {
                        l.id == line_id && matches!(l.kind, LineKind::Service { done: false })
                    })
                    .ok_or("受付待ちのサービス商品が見つかりません")?;
                l.unit_price = amount;
                l.kind = LineKind::Service { done: true };
                let name = l.name;
                if !self.reg_mut(register)?.txn.service_pending() {
                    self.calls.resolve(register, CallKind::Service, self.now);
                }
                done(format!("{name}を受け付けました（{amount}円）"))
            }
            C::RemoveLine {
                register,
                side,
                line_id,
            } => {
                self.check(register, side, Action::RemoveLine)?;
                let r = self.reg_mut(register)?;
                let i = r
                    .txn
                    .lines
                    .iter()
                    .position(|l| l.id == line_id)
                    .ok_or("明細が見つかりません")?;
                let l = r.txn.lines.remove(i);
                if l.kind
                    == (LineKind::Ff {
                        state: FfState::Waiting,
                    })
                {
                    self.restock(l.code, l.qty);
                }
                let r = self.reg_mut(register)?;
                if !r.txn.service_pending() {
                    self.calls.resolve(register, CallKind::Service, self.now);
                }
                if side == Side::Staff {
                }
                done(format!("{}を取り消しました", l.name))
            }
            C::Checkout { register, side } => self.checkout(register, side),
            C::BackToRegistering { register, side } => {
                self.check(register, side, Action::BackToRegistering)?;
                let r = self.reg_mut(register)?;
                r.txn.paying = false;
                r.txn.method = None;
                r.txn.customer_turn = false;
                done("登録画面に戻りました")
            }
            C::SelectPayment {
                register,
                side,
                method,
            } => {
                self.check(register, side, Action::SelectPayment)?;
                let r = self.reg_mut(register)?;
                if r.txn.inserted.total() > 0 && method != PaymentMethod::Cash {
                    return Err("現金が投入済みです。現金でお支払いください".into());
                }
                r.txn.method = Some(method);
                done(format!("{}を選びました", method.label()))
            }
            C::ConfirmAge { register } => {
                self.check(register, Side::Customer, Action::ConfirmAge)?;
                let r = self.reg_mut(register)?;
                r.txn.age_verified = true;
                let age = r.txn.required_age.unwrap_or(0);
                self.calls.resolve(register, CallKind::AgeCheck, self.now);
                done(format!("{age}歳以上であることを確認しました"))
            }
            C::InsertCash {
                register,
                denomination,
                count,
            } => self.insert_cash(register, denomination, count),
            C::TapEmoney { register } => {
                self.check(register, Side::Customer, Action::TapEmoney)?;
                let r = self.reg_mut(register)?;
                let method = r.txn.method.unwrap_or(PaymentMethod::Emoney);
                let total = r.txn.total();
                self.settle(register, method, total, Cash::default())
            }
            C::CallStaff { register } => {
                self.check(register, Side::Customer, Action::CallStaff)?;
                self.call(register, CallKind::CustomerHelp, "お客様からの呼び出し");
                done("係員をお呼びしました")
            }
            C::VoidTransaction { register } => {
                self.check(register, Side::Staff, Action::VoidTransaction)?;
                let now = self.now;
                let r = self.reg_mut(register)?;
                let txn = std::mem::take(&mut r.txn);
                let back = txn.inserted.total();
                for l in &txn.lines {
                    if l.kind
                        == (LineKind::Ff {
                            state: FfState::Waiting,
                        })
                    {
                        self.restock(l.code, l.qty);
                    }
                }
                for k in [CallKind::AgeCheck, CallKind::FastFood, CallKind::Service] {
                    self.calls.resolve(register, k, now);
                }
                if back > 0 {
                    done(format!("取引を中止しました。投入金 {back}円を返却します"))
                } else {
                    done("取引を中止しました")
                }
            }
            C::SetMode { register, mode } => {
                // 有人／セミセルフは名札で切り替わる。選べるのはセルフ（有効なときだけ）
                if mode != Mode::FullSelf {
                    return Err("運用モードは名札で切り替わります（名札あり＝有人、なし＝セミセルフ）".into());
                }
                if !self.self_mode_enabled {
                    return Err("セルフモードは使えません（将来の拡張用）".into());
                }
                let r = self.reg_mut(register)?;
                if r.staff.is_some() || !r.txn.lines.is_empty() {
                    return Err("ログアウトして取引のない状態で切り替えてください".into());
                }
                r.mode = Mode::FullSelf;
                done("セルフモードにしました")
            }
            C::Lock { register } => {
                self.check(register, Side::Staff, Action::Lock)?;
                if let Some(s) = &mut self.reg_mut(register)?.staff {
                    s.locked = true;
                }
                done("画面をロックしました（スタッフバーコードで解除）")
            }
            C::EndStaffMode { register } => {
                self.check(register, Side::Staff, Action::EndStaffMode)?;
                self.reg_mut(register)?.staff = None;
                done("ログアウトしました（セミセルフモード）")
            }
            C::CashOut {
                register,
                kind,
                amount,
                password,
            } => self.cash_out(register, kind, amount, &password),
            C::CashIn {
                register,
                kind,
                amount,
            } => self.cash_in(register, kind, amount),
            C::AddAgeRule {
                owner_id,
                password,
                target,
                min_age,
            } => {
                let by = self.owner(&owner_id, &password)?;
                self.age_rules.add(target, min_age, by, self.now)?;
                done("年確ルールを追加しました")
            }
            C::RemoveAgeRule {
                owner_id,
                password,
                rule_id,
            } => {
                let by = self.owner(&owner_id, &password)?;
                self.age_rules.remove(rule_id, by, self.now)?;
                done("年確ルールを削除しました")
            }
            C::SetProductAge {
                owner_id,
                password,
                code,
                min_age,
            } => {
                let by = self.owner(&owner_id, &password)?;
                self.age_rules.set_product(&code, min_age, by, self.now)?;
                done("年確ルールを変更しました")
            }
            C::SetMatLoss { percent } => {
                self.mat_loss = percent.min(100);
                done(format!("MAT の通信ロスを {}% にしました", self.mat_loss))
            }
            C::SetMatOffline { mat, offline } => {
                let m = self.mats.get_mut(mat).ok_or("MAT が見つかりません")?;
                m.offline = offline;
                done(format!(
                    "{}を{}にしました",
                    m.id,
                    if offline { "不通" } else { "復旧" }
                ))
            }
        }
    }

    fn owner(&mut self, id: &str, password: &str) -> Result<&'static str, String> {
        self.auth.verify_owner(id, password, self.now)?;
        Ok(auth::account(id).map_or("?", |a| a.name))
    }

    fn call(&mut self, register: u8, kind: CallKind, detail: &str) {
        // 店員がすでにそのレジで操作できる状態なら、呼び出しは不要
        if self
            .reg(register)
            .is_some_and(|r| r.staff_state() == StaffState::Ready)
        {
            return;
        }
        self.calls.raise(register, kind, self.now, detail);
    }

    fn restock(&mut self, code: &str, qty: u32) {
        if let Some(e) = self.ff_stock.iter_mut().find(|(c, _)| *c == code) {
            e.1 += qty;
        }
    }

    fn scan(&mut self, register: u8, side: Side, code: &str) -> Result<Outcome, String> {
        let scanned = catalog::classify(code);
        // スタッフバーコードは、ロック中・未ログインでも読む（ログイン・解除）
        if let Scanned::Staff(id) = &scanned {
            let acc = auth::account(id).ok_or("登録されていないスタッフバーコードです")?;
            let now = self.now;
            let r = self.reg_mut(register)?;
            let was = r.staff_state();
            let took_over = r.txn.customer_turn;
            r.staff = Some(StaffSession {
                account: acc,
                locked: false,
            });
            // ログインで店員側が操作できるようになる（お客様の支払い中でも）
            r.txn.customer_turn = false;
            self.calls.resolve(register, CallKind::CustomerHelp, now);
            return done(match was {
                _ if took_over => format!("{}さんが操作を引き継ぎました", acc.name),
                StaffState::Locked => format!("{}さんがロックを解除しました", acc.name),
                _ => format!("{}さんがログインしました（有人モード）", acc.name),
            });
        }
        self.check(register, side, Action::Scan)?;
        let now = self.now;
        let r = self.reg_mut(register)?;
        r.last_receipt = None;
        // 会計画面で読まれたら、登録画面に引き返して追加する（投入済みの現金はそのまま）
        let back = if r.txn.paying {
            r.txn.paying = false;
            r.txn.method = None;
            r.txn.customer_turn = false;
            "（登録画面に戻りました）"
        } else {
            ""
        };
        match scanned {
            Scanned::Staff(_) => unreachable!(),
            Scanned::Unknown => Err("登録されていないバーコードです".into()),
            Scanned::PointCard(no) => {
                r.txn.point_card = Some(no);
                done(format!("ポイントカードを読みました{back}"))
            }
            Scanned::Product(p) if p.needs_staff => {
                r.txn.push(p, 1, LineKind::Service { done: false });
                let ready = r.staff_state() == StaffState::Ready;
                if !ready {
                    self.calls.raise(register, CallKind::Service, now, p.name);
                }
                done(if ready {
                    format!("{}：受付内容を入力してください{back}", p.name)
                } else {
                    format!("{}：係員をお呼びしました{back}", p.name)
                })
            }
            Scanned::Product(p) => {
                r.txn.push(p, 1, LineKind::Item);
                done(format!("{} {}円{back}", p.name, p.price))
            }
        }
    }

    fn register_ff(
        &mut self,
        register: u8,
        side: Side,
        code: &str,
        qty: u32,
    ) -> Result<Outcome, String> {
        self.check(register, side, Action::RegisterFf)?;
        let p = catalog::find(code)
            .filter(|p| p.category == Category::FastFood)
            .ok_or("FF の商品が見つかりません")?;
        let qty = qty.max(1);
        let stock = self
            .ff_stock
            .iter_mut()
            .find(|(c, _)| *c == code)
            .ok_or("在庫がありません")?;
        if stock.1 < qty {
            return Err(format!("{}は在庫が足りません（残り {}）", p.name, stock.1));
        }
        stock.1 -= qty;
        let now = self.now;
        let r = self.reg_mut(register)?;
        r.last_receipt = None;
        if r.txn.paying {
            r.txn.paying = false;
            r.txn.method = None;
            r.txn.customer_turn = false;
        }
        if side == Side::Customer {
            // お客様が自分で取った FF。お渡し・受取確認はない
            r.txn.push(
                p,
                qty,
                LineKind::Ff {
                    state: FfState::Received,
                },
            );
            return done(format!("{} ×{qty} を登録しました", p.name));
        }
        r.txn.push(
            p,
            qty,
            LineKind::Ff {
                state: FfState::Waiting,
            },
        );
        self.calls.resolve(register, CallKind::FastFood, now);
        done(format!(
            "{} ×{qty} を登録しました。準備ができたら「お渡し」を押してください",
            p.name
        ))
    }

    fn checkout(&mut self, register: u8, side: Side) -> Result<Outcome, String> {
        self.check(register, side, Action::Checkout)?;
        let rules = &self.age_rules;
        let r = self
            .registers
            .iter_mut()
            .find(|r| r.id == register)
            .ok_or("レジがありません")?;
        // 年確の判定はこの時点だけ。対象品が複数あっても最も厳しい年齢で 1 回
        let age = r
            .txn
            .lines
            .iter()
            .filter_map(|l| rules.age_for(l.code, l.category))
            .max();
        r.txn.paying = true;
        r.txn.required_age = age;
        // セミセルフでは、会計に進めたら支払いはお客様に任せる
        r.txn.customer_turn = r.mode == Mode::SemiSelf;
        let ready = r.staff_state() == StaffState::Ready;
        let pending = r.txn.age_pending();
        let mode = r.mode;
        if pending && mode != Mode::Attended && !ready {
            self.calls.raise(
                register,
                CallKind::AgeCheck,
                self.now,
                format!("{}歳以上", age.unwrap_or(0)),
            );
            return done("年齢確認のため係員をお呼びしました");
        }
        if !pending {
            self.calls.resolve(register, CallKind::AgeCheck, self.now);
        }
        if pending {
            done(format!(
                "お客様画面で「{}歳以上です」の確認をお願いします",
                age.unwrap_or(0)
            ))
        } else {
            done("お会計に進みました")
        }
    }

    fn insert_cash(&mut self, register: u8, d: u32, count: u32) -> Result<Outcome, String> {
        self.check(register, Side::Customer, Action::InsertCash)?;
        if !is_valid_denomination(d) {
            return Err("使えない金種です".into());
        }
        let r = self.reg_mut(register)?;
        r.txn.method = Some(PaymentMethod::Cash);
        r.txn.inserted.add(d, count.max(1));
        let paid = r.txn.inserted.total();
        let total = r.txn.total();
        if paid < total {
            return done(format!("{paid}円 投入（あと {}円）", total - paid));
        }
        let inserted = r.txn.inserted.clone();
        self.settle(register, PaymentMethod::Cash, paid, inserted)
    }

    fn settle(
        &mut self,
        register: u8,
        method: PaymentMethod,
        paid: u32,
        inserted: Cash,
    ) -> Result<Outcome, String> {
        self.receipts += 1;
        let no = self.receipts;
        let now = self.now;
        let r = self.reg_mut(register)?;
        let total = r.txn.total();
        // 釣銭は紙幣・硬貨をそろえて一度に渡したものとする（排出タイミングは扱わない。cash.rs 参照）
        for (d, n) in &inserted.0 {
            r.drawer.cash.add(*d, *n);
        }
        let change = match r.drawer.take(paid - total, true) {
            Ok(c) => c,
            Err(e) => {
                for (d, n) in &inserted.0 {
                    if let Some(x) = r.drawer.cash.0.iter_mut().find(|(x, _)| x == d) {
                        x.1 -= n;
                    }
                }
                self.receipts -= 1;
                return Err(format!("{e}。係員をお呼びください"));
            }
        };
        let txn = std::mem::take(&mut r.txn);
        let receipt = Receipt {
            no,
            at: now,
            total,
            method: method.label(),
            paid,
            change_total: change.total(),
            change,
            items: txn.lines.iter().map(|l| l.qty).sum(),
            points: if txn.point_card.is_some() {
                total / 100
            } else {
                0
            },
            point_card: txn.point_card,
        };
        let msg = if receipt.change_total > 0 {
            format!("お支払いが完了しました。おつり {}円", receipt.change_total)
        } else {
            "お支払いが完了しました".to_string()
        };
        r.last_receipt = Some(receipt);
        for k in [CallKind::AgeCheck, CallKind::FastFood, CallKind::Service] {
            self.calls.resolve(register, k, now);
        }
        done(msg)
    }

    fn cash_out(
        &mut self,
        register: u8,
        kind: CashOutKind,
        amount: u32,
        password: &str,
    ) -> Result<Outcome, String> {
        self.check(register, Side::Staff, Action::CashOut)?;
        let now = self.now;
        let staff = self
            .reg(register)
            .and_then(|r| r.staff)
            .map_or("?", |s| s.account.name);
        let log = |s: &mut Store, result: String| {
            s.audit.push(AuditEntry {
                at: now,
                register,
                staff,
                operation: kind.label(),
                amount,
                result,
            });
        };
        if amount == 0 {
            return Err("金額を入力してください".into());
        }
        // ログイン中でも、出金のたびに出金パスワードを求める（問題6）
        if let Err(e) = self.auth.verify_cash_out(password, now) {
            log(self, format!("拒否：{e}"));
            return Err(e);
        }
        let r = self.reg_mut(register)?;
        match r.drawer.take(amount, false) {
            Ok(_) => {
                log(self, "実行".into());
                done(format!("{} {amount}円を出金しました", kind.label()))
            }
            Err(e) => {
                log(self, format!("失敗：{e}"));
                Err(e)
            }
        }
    }

    /// お金を入れる操作。抜く操作と違ってパスワードは求めない（記録は残す）
    fn cash_in(&mut self, register: u8, kind: CashInKind, amount: u32) -> Result<Outcome, String> {
        self.check(register, Side::Staff, Action::CashIn)?;
        if amount == 0 {
            return Err("金額を入力してください".into());
        }
        let now = self.now;
        let r = self.reg_mut(register)?;
        let staff = r.staff.map_or("?", |s| s.account.name);
        r.drawer.put(amount);
        self.audit.push(AuditEntry {
            at: now,
            register,
            staff,
            operation: kind.label(),
            amount,
            result: "実行".into(),
        });
        done(format!("{} {amount}円を入金しました", kind.label()))
    }

    // ---- スナップショット ------------------------------------------------

    pub fn snapshot(&self) -> Snapshot {
        let now = self.now;
        Snapshot {
            now,
            registers: self
                .registers
                .iter()
                .map(|r| self.register_view(r))
                .collect(),
            mats: self
                .mats
                .iter()
                .map(|m| MatView {
                    id: m.id.clone(),
                    buzzing: m.buzzing(now),
                    link_ok: m.rx_ok(now),
                    rings: m.rings,
                    offline: m.offline,
                })
                .collect(),
            mat_links: self
                .mats
                .iter()
                .map(|m| MatLink {
                    id: m.id.clone(),
                    link_ok: m.link_ok(now),
                    acked_seq: m.acked_seq,
                    sent: m.sent,
                    retries: m.retries,
                })
                .collect(),
            bell: self.bell,
            calls: self.calls.open.clone(),
            call_history: self.calls.history.clone(),
            age_rules: AgeRulesView {
                rules: self.age_rules.rules.clone(),
                history: self.age_rules.history.clone(),
            },
            audit: self.audit.clone(),
            ff_stock: self.ff_menu(),
            mat_loss: self.mat_loss,
            accounts: ACCOUNTS
                .iter()
                .map(|a| AccountView {
                    barcode: format!("{}{}", catalog::STAFF_PREFIX, a.id),
                    id: a.id,
                    name: a.name,
                    role: a.role,
                })
                .collect(),
            catalog: CATALOG
                .iter()
                .filter(|p| p.category != Category::FastFood)
                .map(|p| CatalogItem {
                    code: p.code,
                    name: p.name,
                    price: p.price,
                    category: p.category,
                    category_label: p.category.label(),
                    age: self.age_rules.age_for(p.code, p.category),
                    legal_age: self.age_rules.legal_age_for(p.code, p.category),
                    store_age: self.age_rules.store_age_for(p.code),
                })
                .collect(),
            categories: Category::ALL.iter().map(|c| (*c, c.label())).collect(),
        }
    }

    fn ff_menu(&self) -> Vec<FfMenuItem> {
        self.ff_stock
            .iter()
            .filter_map(|(code, stock)| {
                catalog::find(code).map(|p| FfMenuItem {
                    code: p.code,
                    name: p.name,
                    price: p.price,
                    stock: *stock,
                })
            })
            .collect()
    }

    fn register_view(&self, r: &Register) -> RegisterView {
        let c = r.context();
        let t = &r.txn;
        let actions = |side: Side, list: &[Action]| {
            list.iter()
                .map(|&a| {
                    let res = permitted(side, a, &c);
                    ActionState {
                        action: a,
                        allowed: res.is_ok(),
                        reason: res.err(),
                    }
                })
                .collect::<Vec<_>>()
        };
        let total = t.total();
        let inserted = t.inserted.total();
        let calls = self.calls.for_register(r.id);
        let staff_called = !calls.is_empty() && c.staff != StaffState::Ready;
        let staff = r.staff.map(|s| s.account);
        let mat_down = self
            .mats
            .iter()
            .filter(|m| !m.link_ok(self.now))
            .map(|m| m.id.clone())
            .collect();

        let lines: Vec<StaffLine> = t
            .lines
            .iter()
            .map(|l| StaffLine {
                line: l.clone(),
                amount: l.amount(),
                age: self.age_rules.age_for(l.code, l.category),
            })
            .collect();

        let customer_lines = t
            .lines
            .iter()
            .map(|l| CustomerLine {
                id: l.id,
                name: l.name,
                qty: l.qty,
                amount: l.amount(),
                kind: l.kind,
            })
            .collect();

        RegisterView {
            id: r.id,
            active_side: active_side(&c),
            mode: r.mode,
            mode_label: r.mode.label(),
            idle_revert_in: (r.mode != Mode::SemiSelf && r.idle())
                .then(|| IDLE_REVERT_MS.saturating_sub(self.now.saturating_sub(r.last_op))),
            staff: StaffView {
                state: c.staff,
                name: staff.map(|a| a.name),
                role: staff.map(|a| a.role),
                phase: c.phase,
                lines,
                total,
                tax8: t.tax_of(TaxRate::Reduced8),
                tax10: t.tax_of(TaxRate::Standard10),
                point_card: t.point_card.clone(),
                required_age: t.required_age.filter(|_| t.paying),
                age_verified: t.age_verified,
                age_pending: c.age_pending,
                method: t.method,
                inserted,
                due: total.saturating_sub(inserted),
                payment_blocker: c.payment_blocker(),
                calls,
                actions: actions(Side::Staff, STAFF_ACTIONS),
                drawer_total: r.drawer.cash.total(),
                drawer: r.drawer.cash.0.clone(),
                mat_down,
            },
            customer: CustomerView {
                phase: c.phase,
                lines: customer_lines,
                total,
                inserted,
                due: total.saturating_sub(inserted),
                point_card: t.point_card.is_some(),
                age_check: c.age_pending.then_some(t.required_age.unwrap_or(0)),
                staff_called,
                staff_present: c.staff == StaffState::Ready,
                method: t.method,
                payment_blocker: c.payment_blocker(),
                actions: actions(Side::Customer, CUSTOMER_ACTIONS),
            },
            last_receipt: r.last_receipt.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// ビュー
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize)]
pub struct ActionState {
    pub action: Action,
    pub allowed: bool,
    pub reason: Option<&'static str>,
}

#[derive(Clone, Debug, Serialize)]
pub struct StaffLine {
    #[serde(flatten)]
    pub line: Line,
    pub amount: u32,
    /// 年確ルールに該当する年齢（店員画面のみの参考表示。判定は会計時）
    pub age: Option<u8>,
}

/// お客様画面に出してよい明細（分類・年確対象かどうか・商品コードは出さない）
#[derive(Clone, Debug, Serialize)]
pub struct CustomerLine {
    pub id: u32,
    pub name: &'static str,
    pub qty: u32,
    pub amount: u32,
    #[serde(flatten)]
    pub kind: LineKind,
}

#[derive(Clone, Debug, Serialize)]
pub struct StaffView {
    pub state: StaffState,
    pub name: Option<&'static str>,
    pub role: Option<Role>,
    pub phase: Phase,
    pub lines: Vec<StaffLine>,
    pub total: u32,
    pub tax8: u32,
    pub tax10: u32,
    pub point_card: Option<String>,
    pub required_age: Option<u8>,
    pub age_verified: bool,
    pub age_pending: bool,
    pub method: Option<PaymentMethod>,
    pub inserted: u32,
    pub due: u32,
    pub payment_blocker: Option<&'static str>,
    pub calls: Vec<Call>,
    pub actions: Vec<ActionState>,
    pub drawer_total: u32,
    /// 自動釣銭機の在高（金種, 枚数）
    pub drawer: Vec<(u32, u32)>,
    pub mat_down: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct CustomerView {
    pub phase: Phase,
    pub lines: Vec<CustomerLine>,
    pub total: u32,
    pub inserted: u32,
    pub due: u32,
    pub point_card: bool,
    /// 年確中なら求める年齢
    pub age_check: Option<u8>,
    pub staff_called: bool,
    pub staff_present: bool,
    pub method: Option<PaymentMethod>,
    pub payment_blocker: Option<&'static str>,
    pub actions: Vec<ActionState>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RegisterView {
    pub id: u8,
    /// 今、画面を操作できる側（もう一方は表示だけ）
    pub active_side: Side,
    pub mode: Mode,
    pub mode_label: &'static str,
    /// セミセルフ以外で待機中なら、セミセルフに戻るまでの残り時間（ミリ秒）
    pub idle_revert_in: Option<u64>,
    pub staff: StaffView,
    pub customer: CustomerView,
    pub last_receipt: Option<Receipt>,
}

/// MAT 自身の画面：鳴動と不通表示だけ
#[derive(Clone, Debug, Serialize)]
pub struct MatView {
    pub id: String,
    pub buzzing: bool,
    pub link_ok: bool,
    pub rings: u64,
    pub offline: bool,
}

/// レジ側から見た MAT の通信状態（シミュレーター表示用）
#[derive(Clone, Debug, Serialize)]
pub struct MatLink {
    pub id: String,
    pub link_ok: bool,
    pub acked_seq: u64,
    pub sent: u64,
    pub retries: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct AgeRulesView {
    pub rules: Vec<crate::age_rules::AgeRule>,
    pub history: Vec<crate::age_rules::RuleChange>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FfMenuItem {
    pub code: &'static str,
    pub name: &'static str,
    pub price: u32,
    pub stock: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct AccountView {
    pub id: &'static str,
    pub barcode: String,
    pub name: &'static str,
    pub role: Role,
}

#[derive(Clone, Debug, Serialize)]
pub struct CatalogItem {
    pub code: &'static str,
    pub name: &'static str,
    pub price: u32,
    pub category: Category,
    pub category_label: &'static str,
    pub age: Option<u8>,
    pub legal_age: Option<u8>,
    pub store_age: Option<u8>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Snapshot {
    pub now: u64,
    pub registers: Vec<RegisterView>,
    pub mats: Vec<MatView>,
    pub mat_links: Vec<MatLink>,
    pub bell: Bell,
    pub calls: Vec<Call>,
    pub call_history: Vec<ResolvedCall>,
    pub age_rules: AgeRulesView,
    pub audit: Vec<AuditEntry>,
    pub ff_stock: Vec<FfMenuItem>,
    pub mat_loss: u8,
    pub accounts: Vec<AccountView>,
    pub catalog: Vec<CatalogItem>,
    pub categories: Vec<(Category, &'static str)>,
}
