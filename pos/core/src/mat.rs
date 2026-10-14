//! MAT（スタッフ携行端末）への通知配信（問題10への対策）。
//!
//! 投げっぱなしのイベント通知ではなく、**状態同期 + ACK + 再送 + 死活監視**
//! で配信する。
//!
//! - レジ側は呼び出し一覧の版数 (`Calls::seq`) を持つ。MAT は受け取った
//!   一覧の版数を ACK で返す。ACK された版数が最新に追いつくまで、
//!   一定間隔で **一覧全体** を再送する（冪等なので重複しても壊れない）。
//! - 変化がなくてもハートビートとして定期送信する。ACK が一定時間
//!   返らない MAT は「不通」として全スタッフ画面に警告を出す。
//!   「届いていないかもしれない」状態を黙って放置しない。

use serde::Serialize;

use crate::calls::Call;
use crate::rng::Rng;

pub const RETRY_MS: u64 = 600;
pub const HEARTBEAT_MS: u64 = 2_000;
pub const LINK_TIMEOUT_MS: u64 = 5_000;
pub const BUZZ_MS: u64 = 1_500;

#[derive(Clone, Debug)]
struct Packet {
    seq: u64,
    deliver_at: u64,
    calls: Vec<Call>,
    lost_down: bool,
    lost_up: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct Mat {
    pub id: String,
    pub staff_name: String,
    /// MAT 側が表示している呼び出し一覧（最後に受信できたもの）
    pub view: Vec<Call>,
    pub view_seq: u64,
    /// レジ側が把握している、この MAT の ACK 済み版数
    pub acked_seq: u64,
    pub last_rx_at: u64,
    pub last_ack_at: u64,
    #[serde(skip)]
    last_sent_at: Option<u64>,
    #[serde(skip)]
    last_sent_seq: u64,
    #[serde(skip)]
    in_flight: Vec<Packet>,
    pub sent: u64,
    pub delivered: u64,
    pub retries: u64,
    /// 障害注入：電源断・圏外
    pub offline: bool,
    pub buzz_until: u64,
}

impl Mat {
    pub fn new(id: &str, staff_name: &str) -> Mat {
        Mat {
            id: id.into(),
            staff_name: staff_name.into(),
            view: vec![],
            view_seq: 0,
            acked_seq: 0,
            last_rx_at: 0,
            last_ack_at: 0,
            last_sent_at: None,
            last_sent_seq: 0,
            in_flight: vec![],
            sent: 0,
            delivered: 0,
            retries: 0,
            offline: false,
            buzz_until: 0,
        }
    }

    /// レジ側から見たリンク状態
    pub fn link_ok(&self, now: u64) -> bool {
        now.saturating_sub(self.last_ack_at) <= LINK_TIMEOUT_MS
    }

    /// MAT 自身から見た受信状態
    pub fn rx_ok(&self, now: u64) -> bool {
        now.saturating_sub(self.last_rx_at) <= LINK_TIMEOUT_MS
    }

    pub fn tick(&mut self, now: u64, seq: u64, calls: &[Call], loss_percent: u8, rng: &mut Rng) {
        // 到着処理
        let (arrived, pending): (Vec<Packet>, Vec<Packet>) = std::mem::take(&mut self.in_flight)
            .into_iter()
            .partition(|p| p.deliver_at <= now);
        self.in_flight = pending;
        for p in arrived {
            if p.lost_down {
                continue;
            }
            if p.seq >= self.view_seq {
                let had: Vec<u32> = self.view.iter().map(|c| c.id).collect();
                let new_alert = p.calls.iter().any(|c| c.alert && !had.contains(&c.id));
                self.view = p.calls;
                self.view_seq = p.seq;
                if new_alert {
                    self.buzz_until = now + BUZZ_MS;
                }
            }
            self.last_rx_at = now;
            self.delivered += 1;
            if !p.lost_up {
                self.acked_seq = self.acked_seq.max(p.seq);
                self.last_ack_at = now;
            }
        }

        // 送信判定：未 ACK の版があれば RETRY_MS ごとに再送、なければハートビート
        let since = self.last_sent_at.map(|t| now.saturating_sub(t));
        let behind = self.acked_seq < seq;
        let due = match since {
            None => true,
            Some(s) => (behind && s >= RETRY_MS) || s >= HEARTBEAT_MS,
        };
        if due {
            if behind && self.last_sent_at.is_some() && self.last_sent_seq == seq {
                self.retries += 1;
            }
            let lost_down = self.offline || rng.chance(loss_percent);
            let lost_up = self.offline || rng.chance(loss_percent);
            self.in_flight.push(Packet {
                seq,
                deliver_at: now + rng.range(60, 200),
                calls: calls.to_vec(),
                lost_down,
                lost_up,
            });
            self.sent += 1;
            self.last_sent_at = Some(now);
            self.last_sent_seq = seq;
        }
    }

    /// 呼び出しが変化したら次の tick で即送信させる
    pub fn kick(&mut self) {
        self.last_sent_at = None;
    }
}
