//! 従業員アカウントと認証（問題6への対策）。
//!
//! - 権限は「画面に表示されているか」ではなく、コア側で操作ごとに判定する。
//! - 返品返金・緊急出金は「責任者ロール」かつ「その場での PIN 再入力」が必須。
//!   ログイン中であっても、メニューが開いていても、この判定は省略されない。
//! - PIN の連続失敗はアカウント単位でロックアウトする。

use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// 一般クルー
    Crew,
    /// 責任者（店長・時間帯責任者）
    Manager,
}

#[derive(Clone, Copy, Debug)]
pub struct StaffAccount {
    pub id: &'static str,
    pub name: &'static str,
    pub role: Role,
    /// シミュレーター用の固定 PIN。実機ではソルト付きハッシュで保持する。
    pin: &'static str,
}

pub const ACCOUNTS: &[StaffAccount] = &[
    StaffAccount {
        id: "1001",
        name: "山田",
        role: Role::Crew,
        pin: "1111",
    },
    StaffAccount {
        id: "1002",
        name: "鈴木",
        role: Role::Crew,
        pin: "2222",
    },
    StaffAccount {
        id: "9001",
        name: "佐藤",
        role: Role::Manager,
        pin: "9999",
    },
];

pub const MAX_PIN_FAILURES: u32 = 3;
pub const LOCKOUT_MS: u64 = 30_000;

#[derive(Debug, PartialEq, Eq)]
pub enum AuthError {
    UnknownStaff,
    WrongPin { remaining: u32 },
    LockedOut { retry_in_ms: u64 },
    NotManager,
}

impl AuthError {
    pub fn message(&self) -> String {
        match self {
            AuthError::UnknownStaff => "従業員番号が見つかりません".into(),
            AuthError::WrongPin { remaining } => {
                format!("PIN が違います（あと {remaining} 回で一時ロック）")
            }
            AuthError::LockedOut { retry_in_ms } => format!(
                "PIN の連続失敗によりロック中です（あと {} 秒）",
                retry_in_ms.div_ceil(1000)
            ),
            AuthError::NotManager => "この操作には責任者の認証が必要です".into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct FailureState {
    count: u32,
    locked_until: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Authenticator {
    failures: Vec<(&'static str, FailureState)>,
}

impl Authenticator {
    pub fn account(id: &str) -> Option<&'static StaffAccount> {
        ACCOUNTS.iter().find(|a| a.id == id)
    }

    fn state_mut(&mut self, id: &'static str) -> &mut FailureState {
        if let Some(i) = self.failures.iter().position(|(k, _)| *k == id) {
            &mut self.failures[i].1
        } else {
            self.failures.push((id, FailureState::default()));
            &mut self.failures.last_mut().expect("just pushed").1
        }
    }

    /// PIN を検証する。成功時は失敗回数をリセットする。
    pub fn verify(
        &mut self,
        id: &str,
        pin: &str,
        now: u64,
    ) -> Result<&'static StaffAccount, AuthError> {
        let account = Self::account(id).ok_or(AuthError::UnknownStaff)?;
        let state = self.state_mut(account.id);
        if now < state.locked_until {
            return Err(AuthError::LockedOut {
                retry_in_ms: state.locked_until - now,
            });
        }
        if constant_time_eq(account.pin.as_bytes(), pin.as_bytes()) {
            state.count = 0;
            Ok(account)
        } else {
            state.count += 1;
            if state.count >= MAX_PIN_FAILURES {
                state.count = 0;
                state.locked_until = now + LOCKOUT_MS;
                Err(AuthError::LockedOut {
                    retry_in_ms: LOCKOUT_MS,
                })
            } else {
                Err(AuthError::WrongPin {
                    remaining: MAX_PIN_FAILURES - state.count,
                })
            }
        }
    }

    /// 責任者操作のためのステップアップ認証。ログイン状態とは無関係に毎回要求する。
    pub fn verify_manager(
        &mut self,
        id: &str,
        pin: &str,
        now: u64,
    ) -> Result<&'static StaffAccount, AuthError> {
        let account = self.verify(id, pin, now)?;
        if account.role == Role::Manager {
            Ok(account)
        } else {
            Err(AuthError::NotManager)
        }
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crew_cannot_pass_manager_step_up() {
        let mut auth = Authenticator::default();
        assert_eq!(
            auth.verify_manager("1001", "1111", 0).unwrap_err(),
            AuthError::NotManager
        );
        assert!(auth.verify_manager("9001", "9999", 0).is_ok());
    }

    #[test]
    fn repeated_failures_lock_the_account() {
        let mut auth = Authenticator::default();
        assert!(matches!(
            auth.verify("9001", "0000", 0),
            Err(AuthError::WrongPin { remaining: 2 })
        ));
        assert!(matches!(
            auth.verify("9001", "0000", 1),
            Err(AuthError::WrongPin { remaining: 1 })
        ));
        assert!(matches!(
            auth.verify("9001", "0000", 2),
            Err(AuthError::LockedOut { .. })
        ));
        // 正しい PIN でもロック中は通らない
        assert!(matches!(
            auth.verify("9001", "9999", 3),
            Err(AuthError::LockedOut { .. })
        ));
        assert!(auth.verify("9001", "9999", 2 + LOCKOUT_MS).is_ok());
    }
}
