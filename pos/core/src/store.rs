//! 店舗全体（レジ3台・MAT 2台）の状態と、コマンド処理。
//!
//! すべての画面（スタッフ画面・お客様画面・MAT）は、ここが生成する
//! **単一のスナップショット** から描画される。画面ごとに別の状態を持たない
//! ため、スタッフ画面にお客様画面をそのまま映し出せる（問題2）。

use serde::{Deserialize, Serialize};

use crate::auth::{Authenticator, Role, ACCOUNTS};
use crate::authority::{
    permitted, Action, Context, Mode, PaymentMethod, Phase, Side, StaffState, CUSTOMER_ACTIONS,
    STAFF_ACTIONS,
};
use crate::calls::{Call, CallKind, Calls, FfItem, FfStatus, ResolvedCall};
use crate::cash::{is_valid_denomination, Cash, Dispenser, DispenserEvent, Drawer};
use crate::catalog::{self, TaxRate, CATALOG, INITIAL_FF_STOCK};
use crate::mat::Mat;
use crate::rng::Rng;

pub const REGISTER_COUNT: u8 = 3;
pub const IDLE_LOCK_MS: u64 = 180_000;
pub const CHANGE_REMIND_MS: u64 = 6_000;
pub const CHANGE_CALL_MS: u64 = 15_000;
pub const NOTICE_MS: u64 = 4_000;
pub const THANKS_MS: u64 = 4_000;

// ---------------------------------------------------------------------------
// コマンド
// ---------------------------------------------------------------------------

/// 「その場の作業」を誰が行ったか。MAT は携行者の認証済み端末として扱う。
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Operator {
    Register { register: u8 },
    Mat { mat: usize },
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Fault {
    MatLoss { percent: u8 },
    MatOffline { mat: usize, offline: bool },
    CoinDelay { enabled: bool },
    BillGlitch { enabled: bool },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    Login {
        register: u8,
        staff_id: String,
        pin: String,
    },
    Logout {
        register: u8,
    },
    Lock {
        register: u8,
    },
    Unlock {
        register: u8,
        staff_id: String,
        pin: String,
    },
    SetMode {
        register: u8,
        mode: Mode,
    },
    Scan {
        register: u8,
        side: Side,
        sku: String,
    },
    OrderFf {
        register: u8,
        side: Side,
        sku: String,
    },
    RemoveLine {
        register: u8,
        side: Side,
        line_id: u32,
    },
    ReadPointCard {
        register: u8,
        card_no: String,
    },
    Checkout {
        register: u8,
        side: Side,
    },
    BackToScanning {
        register: u8,
        side: Side,
    },
    SelectPayment {
        register: u8,
        side: Side,
        method: PaymentMethod,
    },
    InsertCash {
        register: u8,
        denomination: u32,
    },
    TapEmoney {
        register: u8,
    },
    TakeChange {
        register: u8,
    },
    CollectChange {
        register: u8,
    },
    CallStaff {
        register: u8,
    },
    ConfirmAge {
        register: u8,
        by: Operator,
    },
    AckCall {
        call_id: u32,
        by: Operator,
    },
    ResolveCall {
        call_id: u32,
        by: Operator,
    },
    HandOverFf {
        call_id: u32,
        line_id: Option<u32>,
        by: Operator,
    },
    MarkFfOutOfStock {
        call_id: u32,
        line_id: u32,
        by: Operator,
    },
    VoidTransaction {
        register: u8,
    },
    Refund {
        register: u8,
        receipt_id: u32,
        manager_id: String,
        pin: String,
    },
    EmergencyWithdrawal {
        register: u8,
        amount: u32,
        manager_id: String,
        pin: String,
    },
    SetFault {
        fault: Fault,
    },
}

/// 成功時の補足メッセージ。UI は閉じるボタンのないトーストで一定時間だけ表示する。
#[derive(Clone, Debug, Default, Serialize)]
pub struct Outcome {
    pub message: Option<String>,
}

impl Outcome {
    fn none() -> Self {
        Outcome { message: None }
    }
    fn msg(s: impl Into<String>) -> Self {
        Outcome {
            message: Some(s.into()),
        }
    }
}

// ---------------------------------------------------------------------------
// 状態
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize)]
pub struct Line {
    pub id: u32,
    pub sku: String,
    pub name: String,
    pub unit_price: u32,
    pub qty: u32,
    pub tax: TaxRate,
    pub age_restricted: bool,
    pub fast_food: bool,
    pub added_by: Side,
}

impl Line {
    pub fn amount(&self) -> u32 {
        self.unit_price * self.qty
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Session {
    pub staff_id: String,
    pub name: String,
    pub role: Role,
    pub locked: bool,
    #[serde(skip)]
    last_activity: u64,
}

#[derive(Clone, Debug)]
struct Transaction {
    id: u32,
    lines: Vec<Line>,
    next_line: u32,
    phase: Phase,
    age_verified: bool,
    method: Option<PaymentMethod>,
    inserted: Cash,
    point_card: Option<String>,
    /// 精算中の取引に対応するレシート（取引中止・返金の払い出しでは None）
    receipt_id: Option<u32>,
}

impl Transaction {
    fn new(id: u32) -> Self {
        Transaction {
            id,
            lines: vec![],
            next_line: 1,
            phase: Phase::Registering,
            age_verified: false,
            method: None,
            inserted: Cash::default(),
            point_card: None,
            receipt_id: None,
        }
    }

    fn total(&self) -> u32 {
        self.lines.iter().map(Line::amount).sum()
    }

    fn age_pending(&self) -> bool {
        !self.age_verified && self.lines.iter().any(|l| l.age_restricted)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Receipt {
    pub id: u32,
    pub register: u8,
    pub at: u64,
    pub lines: Vec<Line>,
    pub total: u32,
    pub method: PaymentMethod,
    pub paid: u32,
    pub change: u32,
    pub point_card: Option<String>,
    pub points: u32,
    pub refunded: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct Notice {
    pub text: String,
    pub until: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Activity {
    pub at: u64,
    pub text: String,
}

#[derive(Clone, Debug)]
struct Register {
    id: u8,
    mode: Mode,
    session: Option<Session>,
    txn: Transaction,
    drawer: Drawer,
    dispenser: Dispenser,
    customer_notice: Option<Notice>,
    staff_notice: Option<Notice>,
    customer_activity: Vec<Activity>,
    last_receipt: Option<(u32, u64)>,
}

#[derive(Clone, Debug, Serialize)]
pub struct AuditEntry {
    pub at: u64,
    pub register: u8,
    pub operation: String,
    pub staff: String,
    pub result: String,
    pub ok: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct Event {
    pub at: u64,
    pub register: Option<u8>,
    pub text: String,
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct Faults {
    pub mat_loss_percent: u8,
    pub coin_delay: bool,
    pub bill_glitch: bool,
}

pub struct Store {
    now: u64,
    registers: Vec<Register>,
    calls: Calls,
    kicked_seq: u64,
    mats: Vec<Mat>,
    ff_stock: Vec<(&'static str, u32)>,
    receipts: Vec<Receipt>,
    next_txn: u32,
    next_receipt: u32,
    auth: Authenticator,
    audit: Vec<AuditEntry>,
    events: Vec<Event>,
    faults: Faults,
    rng: Rng,
}

type R<T> = Result<T, String>;

fn deny(reason: &str) -> String {
    reason.to_string()
}

fn yen(n: u32) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    format!("{out}円")
}

fn mask_card(card: &str) -> String {
    let digits: String = card.chars().filter(char::is_ascii_digit).collect();
    let tail: String = digits
        .chars()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("****{tail}")
}

impl Store {
    pub fn new(seed: u64) -> Store {
        let mut s = Store {
            now: 0,
            registers: (1..=REGISTER_COUNT)
                .map(|id| Register {
                    id,
                    mode: Mode::Attended,
                    session: None,
                    txn: Transaction::new(u32::from(id)),
                    drawer: Drawer::default(),
                    dispenser: Dispenser::Idle,
                    customer_notice: None,
                    staff_notice: None,
                    customer_activity: vec![],
                    last_receipt: None,
                })
                .collect(),
            calls: Calls::default(),
            kicked_seq: 0,
            mats: vec![Mat::new("A", "山田"), Mat::new("B", "鈴木")],
            ff_stock: INITIAL_FF_STOCK.to_vec(),
            receipts: vec![],
            next_txn: u32::from(REGISTER_COUNT) + 1,
            next_receipt: 1,
            auth: Authenticator::default(),
            audit: vec![],
            events: vec![],
            faults: Faults::default(),
            rng: Rng::new(seed),
        };
        // 初期シナリオ：ワンオペ。レジ1 有人、レジ2 セミセルフ、レジ3 セルフ（スタッフ離席でロック）
        for (id, mode, locked) in [
            (1u8, Mode::Attended, false),
            (2, Mode::SemiSelf, false),
            (3, Mode::FullSelf, true),
        ] {
            let r = &mut s.registers[usize::from(id) - 1];
            r.mode = mode;
            r.session = Some(Session {
                staff_id: "1001".into(),
                name: "山田".into(),
                role: Role::Crew,
                locked,
                last_activity: 0,
            });
        }
        s
    }

    pub fn now(&self) -> u64 {
        self.now
    }

    // ---- helpers ---------------------------------------------------------

    fn idx(&self, register: u8) -> R<usize> {
        if (1..=REGISTER_COUNT).contains(&register) {
            Ok(usize::from(register) - 1)
        } else {
            Err(format!("レジ{register} は存在しません"))
        }
    }

    fn staff_state(r: &Register) -> StaffState {
        match &r.session {
            None => StaffState::LoggedOut,
            Some(s) if s.locked => StaffState::Locked,
            Some(_) => StaffState::Ready,
        }
    }

    fn context(r: &Register, staff: StaffState) -> Context {
        Context {
            mode: r.mode,
            phase: r.txn.phase,
            staff,
            has_lines: !r.txn.lines.is_empty(),
            age_pending: r.txn.age_pending(),
            method: r.txn.method,
            change_presented: matches!(r.dispenser, Dispenser::Presented { .. }),
        }
    }

    fn check(&mut self, i: usize, side: Side, action: Action) -> R<()> {
        let r = &self.registers[i];
        let c = Self::context(r, Self::staff_state(r));
        permitted(side, action, &c).map_err(deny)?;
        if side == Side::Staff {
            self.touch(i);
        }
        Ok(())
    }

    fn touch(&mut self, i: usize) {
        let now = self.now;
        if let Some(s) = self.registers[i].session.as_mut() {
            s.last_activity = now;
        }
    }

    /// 「その場の作業」の実行者を検証し、表示名を返す。
    fn operator(&mut self, register_idx: usize, by: Operator, action: Action) -> R<String> {
        match by {
            Operator::Mat { mat } => {
                let m = self
                    .mats
                    .get(mat)
                    .ok_or_else(|| deny("MAT が存在しません"))?;
                let name = format!("{}（MAT-{}）", m.staff_name, m.id);
                let r = &self.registers[register_idx];
                permitted(Side::Staff, action, &Self::context(r, StaffState::Ready))
                    .map_err(deny)?;
                Ok(name)
            }
            Operator::Register { register } => {
                let i = self.idx(register)?;
                if i != register_idx {
                    return Err(deny("この作業は対象レジか MAT で行ってください"));
                }
                self.check(i, Side::Staff, action)?;
                let s = self.registers[i].session.as_ref().expect("checked");
                Ok(format!("{}（レジ{}）", s.name, register))
            }
        }
    }

    /// 呼び出しの確認・完了だけを行う実行者（どのレジのスタッフ画面からでも可）。
    fn any_operator(&mut self, by: Operator) -> R<String> {
        match by {
            Operator::Mat { mat } => {
                let m = self
                    .mats
                    .get(mat)
                    .ok_or_else(|| deny("MAT が存在しません"))?;
                Ok(format!("{}（MAT-{}）", m.staff_name, m.id))
            }
            Operator::Register { register } => {
                let i = self.idx(register)?;
                match Self::staff_state(&self.registers[i]) {
                    StaffState::Ready => {}
                    StaffState::Locked => return Err(deny("画面ロック中です（PIN で解除）")),
                    StaffState::LoggedOut => return Err(deny("ログインしていません")),
                }
                self.touch(i);
                let s = self.registers[i].session.as_ref().expect("ready");
                Ok(format!("{}（レジ{}）", s.name, register))
            }
        }
    }

    fn event(&mut self, register: Option<u8>, text: impl Into<String>) {
        self.events.push(Event {
            at: self.now,
            register,
            text: text.into(),
        });
        if self.events.len() > 80 {
            self.events.remove(0);
        }
    }

    fn audit(
        &mut self,
        register: u8,
        operation: &str,
        staff: &str,
        result: Result<String, String>,
    ) {
        let (ok, result) = match result {
            Ok(s) => (true, s),
            Err(s) => (false, s),
        };
        self.audit.push(AuditEntry {
            at: self.now,
            register,
            operation: operation.into(),
            staff: staff.into(),
            result,
            ok,
        });
        if self.audit.len() > 50 {
            self.audit.remove(0);
        }
    }

    fn customer_notice(&mut self, i: usize, text: impl Into<String>) {
        self.registers[i].customer_notice = Some(Notice {
            text: text.into(),
            until: self.now + NOTICE_MS,
        });
    }

    fn staff_notice(&mut self, i: usize, text: impl Into<String>) {
        self.registers[i].staff_notice = Some(Notice {
            text: text.into(),
            until: self.now + NOTICE_MS,
        });
    }

    fn activity(&mut self, i: usize, text: impl Into<String>) {
        let a = Activity {
            at: self.now,
            text: text.into(),
        };
        let v = &mut self.registers[i].customer_activity;
        v.push(a);
        if v.len() > 8 {
            v.remove(0);
        }
    }

    fn stock_mut(&mut self, sku: &str) -> Option<&mut u32> {
        self.ff_stock
            .iter_mut()
            .find(|(s, _)| *s == sku)
            .map(|(_, n)| n)
    }

    fn alert_for(mode: Mode, kind: CallKind) -> bool {
        match kind {
            // スタッフが目の前にいるモードでは、年確・FF は鳴動させずに明細として扱う
            CallKind::AgeCheck | CallKind::FastFood => mode == Mode::FullSelf,
            CallKind::CustomerHelp | CallKind::ChangeNotTaken | CallKind::Hardware => true,
        }
    }

    // ---- 明細 --------------------------------------------------------------

    fn add_line(&mut self, i: usize, product: &catalog::Product, side: Side) -> u32 {
        let reg_id = self.registers[i].id;
        let txn_id = self.registers[i].txn.id;
        // FF は渡し待ちの行にだけ数量を足す（渡し済みの行は別行にする）
        let waiting_ff_line = |line: &Line, calls: &Calls| {
            calls
                .find(reg_id, CallKind::FastFood, txn_id)
                .and_then(|c| c.ff_items.iter().find(|it| it.line_id == line.id))
                .is_some_and(|it| it.status == FfStatus::Waiting)
        };
        let existing = self.registers[i].txn.lines.iter().position(|l| {
            l.sku == product.sku && (!l.fast_food || waiting_ff_line(l, &self.calls))
        });
        let txn = &mut self.registers[i].txn;
        let line_id = if let Some(p) = existing {
            txn.lines[p].qty += 1;
            txn.lines[p].id
        } else {
            let id = txn.next_line;
            txn.next_line += 1;
            txn.lines.push(Line {
                id,
                sku: product.sku.into(),
                name: product.name.into(),
                unit_price: product.price,
                qty: 1,
                tax: product.tax,
                age_restricted: product.age_restricted,
                fast_food: product.fast_food,
                added_by: side,
            });
            id
        };
        self.on_line_added(i, product, line_id);
        line_id
    }

    /// 明細追加時の後処理。年確の判定は **ここだけ** で行う（問題7）。
    /// スキャンした瞬間に必ず年確要求が立ち、会計時に改めて判定することはない。
    fn on_line_added(&mut self, i: usize, product: &catalog::Product, line_id: u32) {
        let reg_id = self.registers[i].id;
        let mode = self.registers[i].mode;
        let txn_id = self.registers[i].txn.id;
        let now = self.now;
        if product.age_restricted
            && !self.registers[i].txn.age_verified
            && self
                .calls
                .find(reg_id, CallKind::AgeCheck, txn_id)
                .is_none()
        {
            let alert = Self::alert_for(mode, CallKind::AgeCheck);
            self.calls.open_call(
                reg_id,
                CallKind::AgeCheck,
                txn_id,
                now,
                alert,
                format!("{} をスキャン", product.name),
            );
            self.event(
                Some(reg_id),
                format!("年確要求（スキャン時）: {}", product.name),
            );
        }
        if product.fast_food {
            if self
                .calls
                .find(reg_id, CallKind::FastFood, txn_id)
                .is_none()
            {
                let alert = Self::alert_for(mode, CallKind::FastFood);
                self.calls.open_call(
                    reg_id,
                    CallKind::FastFood,
                    txn_id,
                    now,
                    alert,
                    String::new(),
                );
            }
            let call = self
                .calls
                .find_mut_by(reg_id, CallKind::FastFood, txn_id)
                .expect("just ensured");
            if let Some(it) = call.ff_items.iter_mut().find(|it| it.line_id == line_id) {
                it.qty += 1;
            } else {
                call.ff_items.push(FfItem {
                    line_id,
                    sku: product.sku.into(),
                    name: product.name.into(),
                    qty: 1,
                    status: FfStatus::Waiting,
                });
            }
            call.detail = ff_summary(&call.ff_items);
            self.calls.bump();
        }
    }

    /// 明細が減ったときの後処理：不要になった年確・FF 呼び出しを自動で閉じる。
    fn on_lines_changed(&mut self, i: usize, by: &str) {
        let reg_id = self.registers[i].id;
        let txn_id = self.registers[i].txn.id;
        let now = self.now;
        if !self.registers[i].txn.lines.iter().any(|l| l.age_restricted) {
            if let Some(id) = self
                .calls
                .find(reg_id, CallKind::AgeCheck, txn_id)
                .map(|c| c.id)
            {
                self.calls.resolve(id, now, by);
            }
        }
        if let Some(c) = self.calls.find(reg_id, CallKind::FastFood, txn_id) {
            if c.ff_done() {
                let id = c.id;
                self.calls.resolve(id, now, by);
            }
        }
        if self.registers[i].txn.lines.is_empty() && self.registers[i].txn.phase == Phase::Paying {
            self.registers[i].txn.phase = Phase::Registering;
            self.registers[i].txn.method = None;
        }
    }

    // ---- 精算 --------------------------------------------------------------

    fn settle(&mut self, i: usize, method: PaymentMethod) {
        let now = self.now;
        let reg_id = self.registers[i].id;
        let txn = &self.registers[i].txn;
        let total = txn.total();
        let inserted = txn.inserted.total();
        let (paid, change) = match method {
            PaymentMethod::Cash => (inserted, inserted - total),
            // 現金投入後に電子決済へ切り替えた場合、投入分はそのまま返却する
            _ => (total, inserted),
        };
        let points = if txn.point_card.is_some() {
            total / 200
        } else {
            0
        };
        let receipt = Receipt {
            id: self.next_receipt,
            register: reg_id,
            at: now,
            lines: txn.lines.clone(),
            total,
            method,
            paid,
            change,
            point_card: txn.point_card.clone(),
            points,
            refunded: false,
        };
        self.next_receipt += 1;
        let rid = receipt.id;
        self.receipts.push(receipt);
        self.registers[i].txn.receipt_id = Some(rid);
        self.registers[i].txn.method = Some(method);
        self.event(
            Some(reg_id),
            format!("会計完了 #{rid} {} {}", method.label(), yen(total)),
        );
        self.pay_out(i, change, "おつり");
    }

    /// 払い出し（釣銭・取引中止の返却・返金）。0 円なら即時に取引を締める。
    fn pay_out(&mut self, i: usize, amount: u32, what: &str) {
        let reg_id = self.registers[i].id;
        let txn_id = self.registers[i].txn.id;
        let now = self.now;
        self.registers[i].txn.phase = Phase::Settling;
        if amount == 0 {
            self.finish_txn(i);
            return;
        }
        let cash = match self.registers[i].drawer.take(amount) {
            Ok(c) => c,
            Err((partial, short)) => {
                self.calls.open_call(
                    reg_id,
                    CallKind::Hardware,
                    txn_id,
                    now,
                    true,
                    format!("{what}の釣銭不足 {} — 手渡しをお願いします", yen(short)),
                );
                partial
            }
        };
        if cash.total() == 0 {
            self.finish_txn(i);
            return;
        }
        self.registers[i].dispenser = Dispenser::start(cash, now, self.faults.coin_delay);
    }

    fn finish_txn(&mut self, i: usize) {
        let now = self.now;
        let r = &mut self.registers[i];
        r.last_receipt = r.txn.receipt_id.map(|id| (id, now));
        r.txn = Transaction::new(self.next_txn);
        r.dispenser = Dispenser::Idle;
        self.next_txn += 1;
    }

    fn try_settle_cash(&mut self, i: usize) {
        let txn = &self.registers[i].txn;
        if txn.phase == Phase::Paying
            && matches!(txn.method, None | Some(PaymentMethod::Cash))
            && !txn.lines.is_empty()
            && txn.inserted.total() >= txn.total()
        {
            self.settle(i, PaymentMethod::Cash);
        }
    }

    // ---- dispatch ----------------------------------------------------------

    pub fn dispatch(&mut self, cmd: Command) -> R<Outcome> {
        match cmd {
            Command::Login {
                register,
                staff_id,
                pin,
            }
            | Command::Unlock {
                register,
                staff_id,
                pin,
            } => {
                let i = self.idx(register)?;
                let acc = self
                    .auth
                    .verify(&staff_id, &pin, self.now)
                    .map_err(|e| e.message())?;
                self.registers[i].session = Some(Session {
                    staff_id: acc.id.into(),
                    name: acc.name.into(),
                    role: acc.role,
                    locked: false,
                    last_activity: self.now,
                });
                self.event(Some(register), format!("{} がログイン", acc.name));
                Ok(Outcome::none())
            }
            Command::Logout { register } => {
                let i = self.idx(register)?;
                if Self::staff_state(&self.registers[i]) != StaffState::Ready {
                    return Err(deny("ログアウトするにはロックを解除してください"));
                }
                self.registers[i].session = None;
                self.event(Some(register), "ログアウト");
                Ok(Outcome::none())
            }
            Command::Lock { register } => {
                let i = self.idx(register)?;
                match self.registers[i].session.as_mut() {
                    Some(s) => {
                        s.locked = true;
                        Ok(Outcome::none())
                    }
                    None => Err(deny("ログインしていません")),
                }
            }
            Command::SetMode { register, mode } => {
                let i = self.idx(register)?;
                self.check(i, Side::Staff, Action::SetMode)?;
                self.registers[i].mode = mode;
                for c in self
                    .calls
                    .open
                    .iter_mut()
                    .filter(|c| c.register == register)
                {
                    c.alert = Self::alert_for(mode, c.kind);
                }
                self.calls.bump();
                self.event(Some(register), format!("{}モードに切替", mode.label()));
                Ok(Outcome::none())
            }
            Command::Scan {
                register,
                side,
                sku,
            } => {
                let i = self.idx(register)?;
                let p = catalog::find(&sku).ok_or_else(|| deny("登録されていない商品です"))?;
                if p.fast_food {
                    return Err(deny("FF はメニューから注文してください"));
                }
                self.check(i, side, Action::Scan)?;
                self.back_to_registering_if_paying(i, side);
                self.add_line(i, p, side);
                if side == Side::Customer {
                    self.activity(i, format!("{} をスキャン", p.name));
                }
                Ok(Outcome::none())
            }
            Command::OrderFf {
                register,
                side,
                sku,
            } => {
                let i = self.idx(register)?;
                let p = catalog::find(&sku)
                    .filter(|p| p.fast_food)
                    .ok_or_else(|| deny("FF 商品ではありません"))?;
                self.check(i, side, Action::OrderFf)?;
                let stock = self
                    .stock_mut(p.sku)
                    .ok_or_else(|| deny("在庫情報がありません"))?;
                if *stock == 0 {
                    return Err(format!("{} は品切れです", p.name));
                }
                *stock -= 1;
                self.back_to_registering_if_paying(i, side);
                self.add_line(i, p, side);
                if side == Side::Customer {
                    self.activity(i, format!("{} を注文", p.name));
                }
                Ok(Outcome::none())
            }
            Command::RemoveLine {
                register,
                side,
                line_id,
            } => {
                let i = self.idx(register)?;
                self.check(i, side, Action::RemoveLine)?;
                let name = self.remove_unit(i, line_id)?;
                let by = match side {
                    Side::Staff => "スタッフ",
                    Side::Customer => "お客様",
                };
                self.on_lines_changed(i, by);
                match side {
                    Side::Staff if self.registers[i].mode != Mode::Attended => {
                        self.customer_notice(i, format!("係員が「{name}」を1点取り消しました"));
                    }
                    Side::Customer => self.activity(i, format!("{name} を取消")),
                    _ => {}
                }
                self.try_settle_cash(i);
                Ok(Outcome::none())
            }
            Command::ReadPointCard { register, card_no } => {
                let i = self.idx(register)?;
                self.check(i, Side::Customer, Action::ReadPointCard)?;
                let masked = mask_card(&card_no);
                self.registers[i].txn.point_card = Some(masked.clone());
                self.activity(i, format!("ポイントカード {masked}"));
                // 問題4：読み取り結果は確認ボタンなしで反映し、トーストで知らせるだけ
                Ok(Outcome::msg(format!(
                    "ポイントカード {masked} を読み取りました"
                )))
            }
            Command::Checkout { register, side } => {
                let i = self.idx(register)?;
                self.check(i, side, Action::Checkout)?;
                let txn = &mut self.registers[i].txn;
                txn.phase = Phase::Paying;
                txn.method = None;
                if side == Side::Customer {
                    self.activity(i, "お会計へ進む");
                }
                self.try_settle_cash(i);
                Ok(Outcome::none())
            }
            Command::BackToScanning { register, side } => {
                let i = self.idx(register)?;
                self.check(i, side, Action::BackToScanning)?;
                let txn = &mut self.registers[i].txn;
                txn.phase = Phase::Registering;
                txn.method = None;
                match side {
                    Side::Staff if self.registers[i].mode != Mode::Attended => {
                        self.customer_notice(
                            i,
                            "係員が商品登録に戻しました（投入済みの現金はそのままです）",
                        );
                    }
                    Side::Customer => self.activity(i, "商品登録に戻る"),
                    _ => {}
                }
                Ok(Outcome::none())
            }
            Command::SelectPayment {
                register,
                side,
                method,
            } => {
                let i = self.idx(register)?;
                self.check(i, side, Action::SelectPayment)?;
                self.registers[i].txn.method = Some(method);
                if side == Side::Customer {
                    self.activity(i, format!("{} を選択", method.label()));
                }
                self.try_settle_cash(i);
                Ok(Outcome::none())
            }
            Command::InsertCash {
                register,
                denomination,
            } => {
                let i = self.idx(register)?;
                if !is_valid_denomination(denomination) {
                    return Err(deny("取り扱えない金種です"));
                }
                self.check(i, Side::Customer, Action::InsertCash)?;
                // 問題4：現金を入れた時点で「現金払い」とみなす。選択ボタンを挟まない
                let r = &mut self.registers[i];
                r.txn.method.get_or_insert(PaymentMethod::Cash);
                r.txn.inserted.add(denomination, 1);
                r.drawer.cash.add(denomination, 1);
                self.activity(i, format!("{} 投入", yen(denomination)));
                self.try_settle_cash(i);
                Ok(Outcome::none())
            }
            Command::TapEmoney { register } => {
                let i = self.idx(register)?;
                self.check(i, Side::Customer, Action::TapEmoney)?;
                let method = self.registers[i].txn.method.expect("checked");
                self.activity(i, format!("{} で支払い", method.label()));
                self.settle(i, method);
                Ok(Outcome::none())
            }
            Command::TakeChange { register } => {
                let i = self.idx(register)?;
                self.check(i, Side::Customer, Action::TakeChange)?;
                self.activity(i, "おつりを受け取り");
                self.release_change(i, "お客様");
                Ok(Outcome::none())
            }
            Command::CollectChange { register } => {
                let i = self.idx(register)?;
                self.check(i, Side::Staff, Action::CollectChange)?;
                let name = self.registers[i]
                    .session
                    .as_ref()
                    .map(|s| s.name.clone())
                    .unwrap_or_default();
                self.event(Some(register), "取り忘れ釣銭を回収・保管");
                self.release_change(i, &name);
                Ok(Outcome::msg("取り忘れ釣銭を回収しました"))
            }
            Command::CallStaff { register } => {
                let i = self.idx(register)?;
                self.check(i, Side::Customer, Action::CallStaff)?;
                let txn_id = self.registers[i].txn.id;
                self.activity(i, "店員呼び出し");
                if self
                    .calls
                    .find(register, CallKind::CustomerHelp, txn_id)
                    .is_some()
                {
                    return Ok(Outcome::msg("係員を呼び出し中です"));
                }
                self.calls.open_call(
                    register,
                    CallKind::CustomerHelp,
                    txn_id,
                    self.now,
                    true,
                    "お客様が呼び出しボタンを押しました".into(),
                );
                Ok(Outcome::msg("係員を呼び出しました"))
            }
            Command::ConfirmAge { register, by } => {
                let i = self.idx(register)?;
                self.confirm_age(i, by)
            }
            Command::AckCall { call_id, by } => {
                let name = self.any_operator(by)?;
                let c = self
                    .calls
                    .get_mut(call_id)
                    .ok_or_else(|| deny("呼び出しは完了済みです"))?;
                c.acked_by = Some(name);
                self.calls.bump();
                Ok(Outcome::none())
            }
            Command::ResolveCall { call_id, by } => self.resolve_call(call_id, by),
            Command::HandOverFf {
                call_id,
                line_id,
                by,
            } => self.hand_over_ff(call_id, line_id, by),
            Command::MarkFfOutOfStock {
                call_id,
                line_id,
                by,
            } => self.mark_out_of_stock(call_id, line_id, by),
            Command::VoidTransaction { register } => {
                let i = self.idx(register)?;
                self.check(i, Side::Staff, Action::VoidTransaction)?;
                let txn_id = self.registers[i].txn.id;
                for l in self.registers[i].txn.lines.clone() {
                    if l.fast_food {
                        if let Some(s) = self.stock_mut(&l.sku) {
                            *s += l.qty;
                        }
                    }
                }
                self.calls.resolve_where(self.now, "取引中止", |c| {
                    c.register == register && c.txn_id == txn_id
                });
                let refund = self.registers[i].txn.inserted.total();
                self.registers[i].txn.lines.clear();
                self.registers[i].txn.receipt_id = None;
                self.event(Some(register), "取引中止");
                self.customer_notice(i, "お取引を中止しました");
                self.pay_out(i, refund, "取引中止の返却");
                Ok(Outcome::msg("取引を中止しました"))
            }
            Command::Refund {
                register,
                receipt_id,
                manager_id,
                pin,
            } => self.refund(register, receipt_id, &manager_id, &pin),
            Command::EmergencyWithdrawal {
                register,
                amount,
                manager_id,
                pin,
            } => self.withdraw(register, amount, &manager_id, &pin),
            Command::SetFault { fault } => {
                match fault {
                    Fault::MatLoss { percent } => self.faults.mat_loss_percent = percent.min(100),
                    Fault::MatOffline { mat, offline } => {
                        if let Some(m) = self.mats.get_mut(mat) {
                            m.offline = offline;
                        }
                    }
                    Fault::CoinDelay { enabled } => self.faults.coin_delay = enabled,
                    Fault::BillGlitch { enabled } => self.faults.bill_glitch = enabled,
                }
                Ok(Outcome::none())
            }
        }
    }

    fn back_to_registering_if_paying(&mut self, i: usize, side: Side) {
        let txn = &mut self.registers[i].txn;
        if side == Side::Staff && txn.phase == Phase::Paying {
            txn.phase = Phase::Registering;
            txn.method = None;
        }
    }

    /// 1 点取り消す。戻り値は商品名。
    fn remove_unit(&mut self, i: usize, line_id: u32) -> R<String> {
        let reg_id = self.registers[i].id;
        let txn_id = self.registers[i].txn.id;
        let txn = &mut self.registers[i].txn;
        let p = txn
            .lines
            .iter()
            .position(|l| l.id == line_id)
            .ok_or_else(|| deny("明細が見つかりません"))?;
        let line = &mut txn.lines[p];
        let name = line.name.clone();
        let sku = line.sku.clone();
        let ff = line.fast_food;
        line.qty -= 1;
        if line.qty == 0 {
            txn.lines.remove(p);
        }
        if ff {
            if let Some(s) = self.stock_mut(&sku) {
                *s += 1;
            }
            if let Some(c) = self.calls.find_mut_by(reg_id, CallKind::FastFood, txn_id) {
                if let Some(pos) = c.ff_items.iter().position(|it| it.line_id == line_id) {
                    c.ff_items[pos].qty -= 1;
                    if c.ff_items[pos].qty == 0 {
                        c.ff_items.remove(pos);
                    }
                }
                c.detail = ff_summary(&c.ff_items);
                self.calls.bump();
            }
        }
        Ok(name)
    }

    fn release_change(&mut self, i: usize, by: &str) {
        let reg_id = self.registers[i].id;
        let txn_id = self.registers[i].txn.id;
        self.calls.resolve_where(self.now, by, |c| {
            c.register == reg_id && c.txn_id == txn_id && c.kind == CallKind::ChangeNotTaken
        });
        self.finish_txn(i);
    }

    fn confirm_age(&mut self, i: usize, by: Operator) -> R<Outcome> {
        let name = self.operator(i, by, Action::ConfirmAge)?;
        let reg_id = self.registers[i].id;
        let txn_id = self.registers[i].txn.id;
        self.registers[i].txn.age_verified = true;
        if let Some(id) = self
            .calls
            .find(reg_id, CallKind::AgeCheck, txn_id)
            .map(|c| c.id)
        {
            self.calls.resolve(id, self.now, &name);
        }
        self.event(Some(reg_id), format!("年齢確認 OK — {name}"));
        self.customer_notice(i, "年齢確認が完了しました");
        Ok(Outcome::msg("年齢確認を完了しました"))
    }

    fn call_register(&self, call_id: u32) -> R<(Call, usize)> {
        let c = self
            .calls
            .get(call_id)
            .cloned()
            .ok_or_else(|| deny("呼び出しは完了済みです"))?;
        let i = self.idx(c.register)?;
        Ok((c, i))
    }

    /// 種類に応じた「完了」操作。MAT のボタン一つで適切な処理になる。
    fn resolve_call(&mut self, call_id: u32, by: Operator) -> R<Outcome> {
        let (c, i) = self.call_register(call_id)?;
        match c.kind {
            CallKind::AgeCheck => self.confirm_age(i, by),
            CallKind::FastFood => self.hand_over_ff(call_id, None, by),
            CallKind::ChangeNotTaken => {
                let name = self.operator(i, by, Action::CollectChange)?;
                self.event(
                    Some(c.register),
                    format!("取り忘れ釣銭を回収・保管 — {name}"),
                );
                self.release_change(i, &name);
                Ok(Outcome::msg("取り忘れ釣銭を回収しました"))
            }
            CallKind::CustomerHelp | CallKind::Hardware => {
                let name = self.any_operator(by)?;
                self.calls.resolve(call_id, self.now, &name);
                Ok(Outcome::none())
            }
        }
    }

    fn hand_over_ff(&mut self, call_id: u32, line_id: Option<u32>, by: Operator) -> R<Outcome> {
        let (c, i) = self.call_register(call_id)?;
        if c.kind != CallKind::FastFood {
            return Err(deny("FF の呼び出しではありません"));
        }
        let name = self.ff_operator(i, by)?;
        let call = self.calls.get_mut(call_id).expect("exists");
        let mut handed = vec![];
        for it in call
            .ff_items
            .iter_mut()
            .filter(|it| it.status == FfStatus::Waiting && line_id.is_none_or(|l| l == it.line_id))
        {
            it.status = FfStatus::Handed;
            handed.push(format!("{}×{}", it.name, it.qty));
        }
        if handed.is_empty() {
            return Err(deny("お渡し待ちの FF はありません"));
        }
        call.detail = ff_summary(&call.ff_items);
        let done = call.ff_done();
        self.calls.bump();
        if done {
            self.calls.resolve(call_id, self.now, &name);
        }
        self.event(
            Some(c.register),
            format!("FF お渡し {} — {name}", handed.join("、")),
        );
        Ok(Outcome::none())
    }

    /// FF 作業の実行者：対象レジのスタッフ画面か MAT（その場の作業）。
    fn ff_operator(&mut self, i: usize, by: Operator) -> R<String> {
        match by {
            Operator::Mat { .. } => self.any_operator(by),
            Operator::Register { register } => {
                if self.idx(register)? != i {
                    return Err(deny("この作業は対象レジか MAT で行ってください"));
                }
                self.any_operator(by)
            }
        }
    }

    /// 在庫切れ FF の取り消し（問題3）。スタッフの 1 操作で完結し、
    /// お客様側の確認や操作権の受け渡しを挟まない。
    fn mark_out_of_stock(&mut self, call_id: u32, line_id: u32, by: Operator) -> R<Outcome> {
        let (c, i) = self.call_register(call_id)?;
        if c.kind != CallKind::FastFood {
            return Err(deny("FF の呼び出しではありません"));
        }
        let name = self.ff_operator(i, by)?;
        let item = c
            .ff_items
            .iter()
            .find(|it| it.line_id == line_id && it.status == FfStatus::Waiting)
            .cloned()
            .ok_or_else(|| deny("お渡し待ちの明細ではありません"))?;
        let r = &self.registers[i];
        if r.txn.id != c.txn_id || r.txn.phase == Phase::Settling {
            return Err(deny("会計済みのため返品で対応してください"));
        }
        let txn = &mut self.registers[i].txn;
        if let Some(p) = txn.lines.iter().position(|l| l.id == line_id) {
            txn.lines.remove(p);
        }
        if let Some(s) = self.stock_mut(&item.sku) {
            *s = 0;
        }
        let call = self.calls.get_mut(call_id).expect("exists");
        if let Some(it) = call.ff_items.iter_mut().find(|it| it.line_id == line_id) {
            it.status = FfStatus::OutOfStock;
        }
        call.detail = ff_summary(&call.ff_items);
        self.calls.bump();
        self.on_lines_changed(i, &name);
        self.customer_notice(
            i,
            format!(
                "「{}」は品切れのため取り消しました（係員操作）。そのままお会計を続けられます",
                item.name
            ),
        );
        self.event(
            Some(c.register),
            format!("品切れ取消 {}×{} — {name}", item.name, item.qty),
        );
        self.try_settle_cash(i);
        Ok(Outcome::msg(format!(
            "{} を品切れで取り消しました",
            item.name
        )))
    }

    fn refund(&mut self, register: u8, receipt_id: u32, manager_id: &str, pin: &str) -> R<Outcome> {
        let i = self.idx(register)?;
        let op = format!("返品返金 #{receipt_id}");
        let acc = match self.auth.verify_manager(manager_id, pin, self.now) {
            Ok(a) => a,
            Err(e) => {
                let msg = e.message();
                self.audit(register, &op, manager_id, Err(msg.clone()));
                return Err(msg);
            }
        };
        let result = (|| {
            let r = &self.registers[i];
            if !r.txn.lines.is_empty()
                || r.txn.phase != Phase::Registering
                || !r.dispenser.is_idle()
            {
                return Err(deny("取引中は返金できません"));
            }
            let rc = self
                .receipts
                .iter()
                .find(|rc| rc.id == receipt_id)
                .ok_or_else(|| deny("レシートが見つかりません"))?;
            if rc.refunded {
                return Err(deny("返金済みのレシートです"));
            }
            Ok((rc.total, rc.method))
        })();
        match result {
            Ok((total, method)) => {
                if let Some(rc) = self.receipts.iter_mut().find(|rc| rc.id == receipt_id) {
                    rc.refunded = true;
                }
                let msg = format!("{} を{}で返金", yen(total), method.label());
                self.audit(register, &op, acc.name, Ok(msg.clone()));
                self.event(Some(register), msg.clone());
                if method == PaymentMethod::Cash {
                    self.pay_out(i, total, "返金");
                }
                Ok(Outcome::msg(msg))
            }
            Err(e) => {
                self.audit(register, &op, acc.name, Err(e.clone()));
                Err(e)
            }
        }
    }

    fn withdraw(&mut self, register: u8, amount: u32, manager_id: &str, pin: &str) -> R<Outcome> {
        let i = self.idx(register)?;
        let op = format!("緊急出金 {}", yen(amount));
        let acc = match self.auth.verify_manager(manager_id, pin, self.now) {
            Ok(a) => a,
            Err(e) => {
                let msg = e.message();
                self.audit(register, &op, manager_id, Err(msg.clone()));
                return Err(msg);
            }
        };
        let drawer = &mut self.registers[i].drawer;
        if amount == 0 || amount > drawer.cash.total() {
            let e = deny("出金額が在高を超えています");
            self.audit(register, &op, acc.name, Err(e.clone()));
            return Err(e);
        }
        // 金庫へ移すため、一万円札を含む大きい金種から取り出す
        let mut rest = amount;
        let mut out = Cash::default();
        for (d, n) in drawer.cash.0.iter_mut() {
            let k = (rest / *d).min(*n);
            *n -= k;
            rest -= k * *d;
            out.add(*d, k);
        }
        if rest > 0 {
            for (d, n) in &out.0 {
                drawer.cash.add(*d, *n);
            }
            let e = format!("金種の都合で {} ちょうどは出金できません", yen(amount));
            self.audit(register, &op, acc.name, Err(e.clone()));
            return Err(e);
        }
        let msg = format!("{} を金庫へ出金", yen(amount));
        self.audit(register, &op, acc.name, Ok(msg.clone()));
        self.event(Some(register), msg.clone());
        Ok(Outcome::msg(msg))
    }

    // ---- 時間経過 ----------------------------------------------------------

    pub fn tick(&mut self, now: u64) {
        if now < self.now {
            return;
        }
        self.now = now;
        for i in 0..self.registers.len() {
            let reg_id = self.registers[i].id;
            let mut glitch = self.faults.bill_glitch;
            let ev = self.registers[i].dispenser.tick(now, &mut glitch);
            self.faults.bill_glitch = glitch;
            match ev {
                Some(DispenserEvent::BillRefed) => {
                    self.event(
                        Some(reg_id),
                        "紙幣保持センサー異常を検知 → 放出せず回収して再排出",
                    );
                    self.staff_notice(i, "紙幣を回収して再排出しています（自動復旧）");
                }
                Some(DispenserEvent::Presented) => {
                    if let Dispenser::Presented {
                        bill_ready_ms,
                        coin_ready_ms,
                        bills,
                        coins,
                        ..
                    } = &self.registers[i].dispenser
                    {
                        let text = format!(
                            "釣銭を同時提示（紙幣{} 準備 {:.1}s / 硬貨{} 準備 {:.1}s）",
                            yen(bills.total()),
                            *bill_ready_ms as f64 / 1000.0,
                            yen(coins.total()),
                            *coin_ready_ms as f64 / 1000.0
                        );
                        self.event(Some(reg_id), text);
                    }
                }
                None => {}
            }
            if let Dispenser::Presented { presented_at, .. } = self.registers[i].dispenser {
                let txn_id = self.registers[i].txn.id;
                if now - presented_at >= CHANGE_CALL_MS
                    && self
                        .calls
                        .find(reg_id, CallKind::ChangeNotTaken, txn_id)
                        .is_none()
                {
                    self.calls.open_call(
                        reg_id,
                        CallKind::ChangeNotTaken,
                        txn_id,
                        now,
                        true,
                        "取り出し口に釣銭が残っています".into(),
                    );
                }
            }
            let r = &mut self.registers[i];
            if let Some(s) = r.session.as_mut() {
                if !s.locked && now.saturating_sub(s.last_activity) > IDLE_LOCK_MS {
                    s.locked = true;
                }
            }
            if r.customer_notice.as_ref().is_some_and(|n| n.until <= now) {
                r.customer_notice = None;
            }
            if r.staff_notice.as_ref().is_some_and(|n| n.until <= now) {
                r.staff_notice = None;
            }
        }
        if self.calls.seq != self.kicked_seq {
            self.kicked_seq = self.calls.seq;
            for m in &mut self.mats {
                m.kick();
            }
        }
        let calls = self.calls.sorted();
        let seq = self.calls.seq;
        let loss = self.faults.mat_loss_percent;
        for m in &mut self.mats {
            m.tick(now, seq, &calls, loss, &mut self.rng);
        }
    }

    // ---- スナップショット ---------------------------------------------------

    pub fn snapshot(&self) -> Snapshot {
        let now = self.now;
        Snapshot {
            now,
            registers: self
                .registers
                .iter()
                .map(|r| self.register_view(r))
                .collect(),
            calls: self.calls.sorted(),
            call_history: self.calls.history.iter().rev().take(10).cloned().collect(),
            call_seq: self.calls.seq,
            mats: self
                .mats
                .iter()
                .map(|m| MatView {
                    id: m.id.clone(),
                    staff_name: m.staff_name.clone(),
                    view: m.view.clone(),
                    view_seq: m.view_seq,
                    acked_seq: m.acked_seq,
                    link_ok: m.link_ok(now),
                    rx_ok: m.rx_ok(now),
                    buzzing: m.buzz_until > now,
                    sent: m.sent,
                    delivered: m.delivered,
                    retries: m.retries,
                    offline: m.offline,
                    last_rx_ago_ms: now.saturating_sub(m.last_rx_at),
                })
                .collect(),
            ff_menu: CATALOG
                .iter()
                .filter(|p| p.fast_food)
                .map(|p| MenuItem {
                    sku: p.sku,
                    name: p.name,
                    price: p.price,
                    age_restricted: p.age_restricted,
                    stock: self
                        .ff_stock
                        .iter()
                        .find(|(s, _)| *s == p.sku)
                        .map(|(_, n)| *n),
                })
                .collect(),
            items: CATALOG
                .iter()
                .filter(|p| !p.fast_food)
                .map(|p| MenuItem {
                    sku: p.sku,
                    name: p.name,
                    price: p.price,
                    age_restricted: p.age_restricted,
                    stock: None,
                })
                .collect(),
            receipts: self.receipts.iter().rev().take(10).cloned().collect(),
            audit: self.audit.iter().rev().take(20).cloned().collect(),
            events: self.events.iter().rev().take(40).cloned().collect(),
            faults: self.faults,
            mat_offline: self.mats.iter().map(|m| m.offline).collect(),
            accounts: ACCOUNTS
                .iter()
                .map(|a| AccountView {
                    id: a.id,
                    name: a.name,
                    role: a.role,
                })
                .collect(),
        }
    }

    fn register_view(&self, r: &Register) -> RegisterView {
        let now = self.now;
        let staff = Self::staff_state(r);
        let ctx = Self::context(r, staff);
        let actions = |side: Side, list: &[Action]| -> Vec<ActionView> {
            list.iter()
                .map(|&a| {
                    let res = permitted(side, a, &ctx);
                    ActionView {
                        action: a,
                        allowed: res.is_ok(),
                        reason: res.err(),
                    }
                })
                .collect()
        };
        let txn = &r.txn;
        let total = txn.total();
        let taxable = |rate: TaxRate| -> u32 {
            txn.lines
                .iter()
                .filter(|l| l.tax == rate)
                .map(Line::amount)
                .sum()
        };
        let thanks = r
            .last_receipt
            .filter(|(_, at)| now.saturating_sub(*at) < THANKS_MS && txn.lines.is_empty())
            .and_then(|(id, _)| self.receipts.iter().find(|rc| rc.id == id).cloned());
        let presented_at = match &r.dispenser {
            Dispenser::Presented { presented_at, .. } => Some(*presented_at),
            _ => None,
        };
        let age = if txn.age_verified {
            "verified"
        } else if txn.age_pending() {
            "pending"
        } else {
            "none"
        };
        let ff_call = self.calls.find(r.id, CallKind::FastFood, txn.id);
        RegisterView {
            id: r.id,
            mode: r.mode,
            phase: txn.phase,
            staff_state: staff,
            session: r.session.clone(),
            txn_id: txn.id,
            lines: txn
                .lines
                .iter()
                .map(|l| LineView {
                    line: l.clone(),
                    ff_status: ff_call
                        .and_then(|c| c.ff_items.iter().find(|it| it.line_id == l.id))
                        .map(|it| it.status),
                })
                .collect(),
            total,
            item_count: txn.lines.iter().map(|l| l.qty).sum(),
            taxable8: taxable(TaxRate::Reduced8),
            taxable10: taxable(TaxRate::Standard10),
            point_card: txn.point_card.clone(),
            age,
            method: txn.method,
            inserted: txn.inserted.total(),
            remaining: total.saturating_sub(txn.inserted.total()),
            dispenser: r.dispenser.clone(),
            change_reminder: presented_at.is_some_and(|t| now - t >= CHANGE_REMIND_MS),
            drawer_total: r.drawer.cash.total(),
            staff_actions: actions(Side::Staff, STAFF_ACTIONS),
            customer_actions: actions(Side::Customer, CUSTOMER_ACTIONS),
            customer_notice: r.customer_notice.as_ref().map(|n| n.text.clone()),
            staff_notice: r.staff_notice.as_ref().map(|n| n.text.clone()),
            customer_activity: r.customer_activity.iter().rev().cloned().collect(),
            thanks,
            alerting: self
                .calls
                .open
                .iter()
                .any(|c| c.register == r.id && c.alert),
            open_calls: self
                .calls
                .open
                .iter()
                .filter(|c| c.register == r.id)
                .count(),
        }
    }
}

fn ff_summary(items: &[FfItem]) -> String {
    items
        .iter()
        .filter(|it| it.status == FfStatus::Waiting)
        .map(|it| format!("{}×{}", it.name, it.qty))
        .collect::<Vec<_>>()
        .join("、")
}

// ---------------------------------------------------------------------------
// スナップショット（UI が読む唯一の状態）
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize)]
pub struct ActionView {
    pub action: Action,
    pub allowed: bool,
    pub reason: Option<&'static str>,
}

#[derive(Clone, Debug, Serialize)]
pub struct LineView {
    #[serde(flatten)]
    pub line: Line,
    pub ff_status: Option<FfStatus>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RegisterView {
    pub id: u8,
    pub mode: Mode,
    pub phase: Phase,
    pub staff_state: StaffState,
    pub session: Option<Session>,
    pub txn_id: u32,
    pub lines: Vec<LineView>,
    pub total: u32,
    pub item_count: u32,
    pub taxable8: u32,
    pub taxable10: u32,
    pub point_card: Option<String>,
    pub age: &'static str,
    pub method: Option<PaymentMethod>,
    pub inserted: u32,
    pub remaining: u32,
    pub dispenser: Dispenser,
    pub change_reminder: bool,
    pub drawer_total: u32,
    pub staff_actions: Vec<ActionView>,
    pub customer_actions: Vec<ActionView>,
    pub customer_notice: Option<String>,
    pub staff_notice: Option<String>,
    pub customer_activity: Vec<Activity>,
    pub thanks: Option<Receipt>,
    pub alerting: bool,
    pub open_calls: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct MatView {
    pub id: String,
    pub staff_name: String,
    pub view: Vec<Call>,
    pub view_seq: u64,
    pub acked_seq: u64,
    pub link_ok: bool,
    pub rx_ok: bool,
    pub buzzing: bool,
    pub sent: u64,
    pub delivered: u64,
    pub retries: u64,
    pub offline: bool,
    pub last_rx_ago_ms: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct MenuItem {
    pub sku: &'static str,
    pub name: &'static str,
    pub price: u32,
    pub age_restricted: bool,
    pub stock: Option<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct AccountView {
    pub id: &'static str,
    pub name: &'static str,
    pub role: Role,
}

#[derive(Clone, Debug, Serialize)]
pub struct Snapshot {
    pub now: u64,
    pub registers: Vec<RegisterView>,
    pub calls: Vec<Call>,
    pub call_history: Vec<ResolvedCall>,
    pub call_seq: u64,
    pub mats: Vec<MatView>,
    pub ff_menu: Vec<MenuItem>,
    pub items: Vec<MenuItem>,
    pub receipts: Vec<Receipt>,
    pub audit: Vec<AuditEntry>,
    pub events: Vec<Event>,
    pub faults: Faults,
    pub mat_offline: Vec<bool>,
    pub accounts: Vec<AccountView>,
}
