//! 決定的な疑似乱数（通信ロスなどの障害注入用）。
//! シードが同じなら同じシナリオが再現でき、テストで障害を固定できる。

#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.max(1))
    }

    pub fn next_u64(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// `percent` % の確率で true
    pub fn chance(&mut self, percent: u8) -> bool {
        (self.next_u64() % 100) < u64::from(percent)
    }

    pub fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.next_u64() % (hi - lo + 1)
    }
}
