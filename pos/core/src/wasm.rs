//! wasm-bindgen 境界。状態はすべて Rust 側にあり、JS は JSON の
//! コマンドを送ってスナップショットを受け取るだけ。

use wasm_bindgen::prelude::*;

use crate::store::Store;

#[wasm_bindgen]
pub struct PosSimulator {
    store: Store,
}

#[wasm_bindgen]
impl PosSimulator {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32, registers: u8, mats: u8) -> PosSimulator {
        console_error_panic_hook::set_once();
        PosSimulator {
            store: Store::new(u64::from(seed), registers, mats),
        }
    }

    /// コマンド（JSON）を処理し、結果（JSON）を返す。
    pub fn dispatch(&mut self, command_json: &str) -> String {
        crate::dispatch_json(&mut self.store, command_json)
    }

    /// シミュレーション時刻（ms）を進める。
    pub fn tick(&mut self, now_ms: f64) {
        self.store.tick(now_ms.max(0.0) as u64);
    }

    pub fn snapshot(&self) -> String {
        crate::snapshot_json(&self.store)
    }
}
