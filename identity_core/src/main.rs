// src/main.rs

use identity_core::common::fake_wasm_mod::now_utc;

fn main() {
  let current_time = now_utc();
  println!("Current time (UTC): {}", current_time);
}
