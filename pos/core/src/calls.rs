//! 店員呼び出し（問題8・9への対策）。
//!
//! - 呼び出しは **発生したレジに紐づく独立したレコード** であり、
//!   単一の FIFO キューでも、後続をブロックするモーダルでもない。
//!   どの呼び出しも、他の呼び出しの状態と無関係に応答・完了できる。
//! - 鳴動（アラート）は発生したレジと MAT にだけ出す。他のレジには
//!   一覧に載るだけで割り込まない。
//! - FF の呼び出しは「何を何個渡すか」を明細として持つ（問題9）。

use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CallKind {
    AgeCheck,
    ChangeNotTaken,
    Hardware,
    CustomerHelp,
    FastFood,
}

impl CallKind {
    /// 小さいほど優先。表示順にのみ使い、処理順は強制しない。
    pub fn priority(self) -> u8 {
        match self {
            CallKind::AgeCheck => 0,
            CallKind::ChangeNotTaken => 1,
            CallKind::Hardware => 1,
            CallKind::CustomerHelp => 2,
            CallKind::FastFood => 3,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            CallKind::AgeCheck => "年齢確認",
            CallKind::ChangeNotTaken => "釣銭取り忘れ",
            CallKind::Hardware => "機器",
            CallKind::CustomerHelp => "お呼び出し",
            CallKind::FastFood => "FF お渡し",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FfStatus {
    Waiting,
    Handed,
    OutOfStock,
}

#[derive(Clone, Debug, Serialize)]
pub struct FfItem {
    pub line_id: u32,
    pub sku: String,
    pub name: String,
    pub qty: u32,
    pub status: FfStatus,
}

#[derive(Clone, Debug, Serialize)]
pub struct Call {
    pub id: u32,
    pub register: u8,
    pub kind: CallKind,
    pub label: &'static str,
    pub txn_id: u32,
    pub created_at: u64,
    /// true のとき、発生レジと MAT で鳴動させる
    pub alert: bool,
    /// 「向かいます」を押したスタッフ
    pub acked_by: Option<String>,
    pub detail: String,
    pub ff_items: Vec<FfItem>,
}

impl Call {
    pub fn ff_done(&self) -> bool {
        self.ff_items.iter().all(|i| i.status != FfStatus::Waiting)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ResolvedCall {
    pub id: u32,
    pub register: u8,
    pub label: &'static str,
    pub resolved_at: u64,
    pub resolved_by: String,
}

#[derive(Clone, Debug, Default)]
pub struct Calls {
    next_id: u32,
    pub open: Vec<Call>,
    pub history: Vec<ResolvedCall>,
    /// 呼び出し状態の版数。変化するたびに増え、MAT 同期の基準になる。
    pub seq: u64,
}

impl Calls {
    pub fn bump(&mut self) {
        self.seq += 1;
    }

    pub fn open_call(
        &mut self,
        register: u8,
        kind: CallKind,
        txn_id: u32,
        now: u64,
        alert: bool,
        detail: String,
    ) -> u32 {
        self.next_id += 1;
        let id = self.next_id;
        self.open.push(Call {
            id,
            register,
            kind,
            label: kind.label(),
            txn_id,
            created_at: now,
            alert,
            acked_by: None,
            detail,
            ff_items: vec![],
        });
        self.bump();
        id
    }

    pub fn find(&self, register: u8, kind: CallKind, txn_id: u32) -> Option<&Call> {
        self.open
            .iter()
            .find(|c| c.register == register && c.kind == kind && c.txn_id == txn_id)
    }

    pub fn find_mut_by(&mut self, register: u8, kind: CallKind, txn_id: u32) -> Option<&mut Call> {
        self.open
            .iter_mut()
            .find(|c| c.register == register && c.kind == kind && c.txn_id == txn_id)
    }

    pub fn get(&self, id: u32) -> Option<&Call> {
        self.open.iter().find(|c| c.id == id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Call> {
        self.open.iter_mut().find(|c| c.id == id)
    }

    pub fn resolve(&mut self, id: u32, now: u64, by: &str) -> bool {
        let Some(i) = self.open.iter().position(|c| c.id == id) else {
            return false;
        };
        let c = self.open.remove(i);
        self.history.push(ResolvedCall {
            id: c.id,
            register: c.register,
            label: c.label,
            resolved_at: now,
            resolved_by: by.to_string(),
        });
        if self.history.len() > 30 {
            self.history.remove(0);
        }
        self.bump();
        true
    }

    pub fn resolve_where(&mut self, now: u64, by: &str, pred: impl Fn(&Call) -> bool) {
        let ids: Vec<u32> = self.open.iter().filter(|c| pred(c)).map(|c| c.id).collect();
        for id in ids {
            self.resolve(id, now, by);
        }
    }

    /// 表示用：優先度 → 発生時刻の順。
    pub fn sorted(&self) -> Vec<Call> {
        let mut v = self.open.clone();
        v.sort_by_key(|c| (c.kind.priority(), c.created_at, c.id));
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calls_resolve_independently_of_order() {
        let mut calls = Calls::default();
        let a = calls.open_call(3, CallKind::FastFood, 1, 0, true, String::new());
        let b = calls.open_call(1, CallKind::CustomerHelp, 2, 10, true, String::new());
        // 後から来たレジ1の呼び出しを先に完了できる（FIFO ブロックなし）
        assert!(calls.resolve(b, 20, "MAT-A"));
        assert!(calls.get(a).is_some());
        assert!(calls.get(b).is_none());
    }
}
