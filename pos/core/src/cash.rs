//! 現金ハードウェアのシミュレーション（問題11・12への対策）。
//!
//! - 紙幣ユニットと硬貨ユニットは並列に動くが、どちらも「取り出し口の手前
//!   （ステージング）」までしか払い出さない。両方の準備が完了した時点で
//!   はじめてシャッターを開け、紙幣と硬貨を **同時に** 提示する。
//! - 紙幣は取り出し口のクランプで保持したまま提示する。保持センサーが紙幣を
//!   検知できない場合は放出せず回収して再排出する（まき散らしの防止）。

use serde::Serialize;

pub const BILLS: &[u32] = &[10_000, 5_000, 2_000, 1_000];
pub const COINS: &[u32] = &[500, 100, 50, 10, 5, 1];

pub fn is_bill(denomination: u32) -> bool {
    BILLS.contains(&denomination)
}

pub fn is_valid_denomination(denomination: u32) -> bool {
    is_bill(denomination) || COINS.contains(&denomination)
}

/// 金種別の枚数。金額の大きい順に並べて保持する。
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Cash(pub Vec<(u32, u32)>);

impl Cash {
    pub fn total(&self) -> u32 {
        self.0.iter().map(|(d, n)| d * n).sum()
    }

    pub fn count(&self) -> u32 {
        self.0.iter().map(|(_, n)| n).sum()
    }

    pub fn add(&mut self, denomination: u32, n: u32) {
        if n == 0 {
            return;
        }
        if let Some(e) = self.0.iter_mut().find(|(d, _)| *d == denomination) {
            e.1 += n;
        } else {
            self.0.push((denomination, n));
            self.0.sort_by_key(|e| std::cmp::Reverse(e.0));
        }
    }

    pub fn of(&self, denomination: u32) -> u32 {
        self.0
            .iter()
            .find(|(d, _)| *d == denomination)
            .map_or(0, |(_, n)| *n)
    }

    pub fn bills(&self) -> Cash {
        Cash(
            self.0
                .iter()
                .copied()
                .filter(|(d, _)| is_bill(*d))
                .collect(),
        )
    }

    pub fn coins(&self) -> Cash {
        Cash(
            self.0
                .iter()
                .copied()
                .filter(|(d, _)| !is_bill(*d))
                .collect(),
        )
    }
}

/// 釣銭機の在高。
#[derive(Clone, Debug, Serialize)]
pub struct Drawer {
    pub cash: Cash,
}

impl Default for Drawer {
    fn default() -> Self {
        let mut cash = Cash::default();
        for (d, n) in [
            (5_000, 10),
            (1_000, 30),
            (500, 20),
            (100, 50),
            (50, 30),
            (10, 50),
            (5, 30),
            (1, 50),
        ] {
            cash.add(d, n);
        }
        Drawer { cash }
    }
}

impl Drawer {
    /// 在高の範囲内で `amount` を金種に分解して取り出す。
    /// 足りない分は `Err(取り出せた分, 不足額)` で返す（取り出した分は在高から減る）。
    pub fn take(&mut self, amount: u32) -> Result<Cash, (Cash, u32)> {
        let mut rest = amount;
        let mut out = Cash::default();
        for &d in BILLS.iter().chain(COINS) {
            if d == 10_000 || d == 2_000 {
                // 一万円札・二千円札は釣銭に使わない
                continue;
            }
            let have = self.cash.of(d);
            let n = (rest / d).min(have);
            if n > 0 {
                out.add(d, n);
                rest -= d * n;
            }
        }
        for (d, n) in &out.0 {
            if let Some(e) = self.cash.0.iter_mut().find(|(x, _)| x == d) {
                e.1 -= n;
            }
        }
        if rest == 0 {
            Ok(out)
        } else {
            Err((out, rest))
        }
    }
}

pub const BILL_BASE_MS: u64 = 900;
pub const BILL_PER_NOTE_MS: u64 = 350;
pub const COIN_BASE_MS: u64 = 400;
pub const COIN_PER_PIECE_MS: u64 = 60;
pub const COIN_DELAY_FAULT_MS: u64 = 3_000;
pub const BILL_REFEED_MS: u64 = 1_800;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum Dispenser {
    Idle,
    /// 両ユニットがステージングへ払い出し中。シャッターは閉じている。
    Staging {
        bills: Cash,
        coins: Cash,
        started_at: u64,
        bill_ready_at: u64,
        coin_ready_at: u64,
        bill_refed: bool,
    },
    /// 紙幣と硬貨を同時に提示中（紙幣はクランプで保持）。
    Presented {
        bills: Cash,
        coins: Cash,
        presented_at: u64,
        /// 各ユニットがステージングを完了した時刻（開始からの経過 ms）
        bill_ready_ms: u64,
        coin_ready_ms: u64,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum DispenserEvent {
    BillRefed,
    Presented,
}

impl Dispenser {
    pub fn start(change: Cash, now: u64, coin_delay: bool) -> Dispenser {
        let bills = change.bills();
        let coins = change.coins();
        let bill_ms = if bills.count() == 0 {
            0
        } else {
            BILL_BASE_MS + BILL_PER_NOTE_MS * u64::from(bills.count())
        };
        let coin_ms = if coins.count() == 0 {
            0
        } else {
            COIN_BASE_MS
                + COIN_PER_PIECE_MS * u64::from(coins.count())
                + if coin_delay { COIN_DELAY_FAULT_MS } else { 0 }
        };
        Dispenser::Staging {
            bills,
            coins,
            started_at: now,
            bill_ready_at: now + bill_ms,
            coin_ready_at: now + coin_ms,
            bill_refed: false,
        }
    }

    /// `bill_glitch` が true の場合、紙幣保持センサーの異常を一度だけ発生させる。
    pub fn tick(&mut self, now: u64, bill_glitch: &mut bool) -> Option<DispenserEvent> {
        let Dispenser::Staging {
            bills,
            coins,
            started_at,
            bill_ready_at,
            coin_ready_at,
            bill_refed,
        } = self
        else {
            return None;
        };
        if *bill_glitch && bills.count() > 0 && now >= *bill_ready_at && !*bill_refed {
            // 保持センサーが紙幣を検知できない → 放出せず回収・再排出
            *bill_glitch = false;
            *bill_refed = true;
            *bill_ready_at = now + BILL_REFEED_MS;
            return Some(DispenserEvent::BillRefed);
        }
        if now >= *bill_ready_at && now >= *coin_ready_at {
            *self = Dispenser::Presented {
                bills: bills.clone(),
                coins: coins.clone(),
                presented_at: now,
                bill_ready_ms: *bill_ready_at - *started_at,
                coin_ready_ms: *coin_ready_at - *started_at,
            };
            return Some(DispenserEvent::Presented);
        }
        None
    }

    pub fn is_idle(&self) -> bool {
        matches!(self, Dispenser::Idle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_is_split_greedily_without_10k_or_2k() {
        let mut drawer = Drawer::default();
        let change = drawer.take(8_766).unwrap();
        assert_eq!(change.of(5_000), 1);
        assert_eq!(change.of(1_000), 3);
        assert_eq!(change.of(500), 1);
        assert_eq!(change.of(100), 2);
        assert_eq!(change.of(50), 1);
        assert_eq!(change.of(10), 1);
        assert_eq!(change.of(5), 1);
        assert_eq!(change.of(1), 1);
        assert_eq!(change.total(), 8_766);
    }

    #[test]
    fn bills_and_coins_are_presented_at_the_same_instant() {
        let mut drawer = Drawer::default();
        let change = drawer.take(1_234).unwrap();
        let mut d = Dispenser::start(change, 0, true);
        let mut glitch = false;
        let mut t = 0;
        while !matches!(d, Dispenser::Presented { .. }) {
            t += 10;
            let ev = d.tick(t, &mut glitch);
            if let Dispenser::Staging { .. } = d {
                assert!(ev.is_none());
            }
        }
        let Dispenser::Presented {
            bills,
            coins,
            presented_at,
            bill_ready_ms,
            coin_ready_ms,
        } = d
        else {
            unreachable!()
        };
        assert_eq!(bills.total(), 1_000);
        assert_eq!(coins.total(), 234);
        // 硬貨ユニットが大幅に遅れても、提示は両方そろった一度きり
        assert!(coin_ready_ms > bill_ready_ms + 2_000);
        assert!(presented_at >= coin_ready_ms);
    }

    #[test]
    fn bill_sensor_glitch_refeeds_instead_of_ejecting() {
        let mut drawer = Drawer::default();
        let mut d = Dispenser::start(drawer.take(1_000).unwrap(), 0, false);
        let mut glitch = true;
        let mut events = vec![];
        let mut t = 0;
        while !matches!(d, Dispenser::Presented { .. }) {
            t += 10;
            if let Some(e) = d.tick(t, &mut glitch) {
                events.push(e);
            }
        }
        assert_eq!(
            events,
            vec![DispenserEvent::BillRefed, DispenserEvent::Presented]
        );
        assert!(!glitch);
    }
}
