#[cfg(target_arch = "wasm32-unknown-unknown")]
pub mod wasm_time {
  #[link(wasm_import_module = "time")]
  extern "C" {
    /// Returns the timestamp
    #[link_name = "now"]
    pub fn now() -> u64;
  }

  pub fn now_utc() -> u64 {
    unsafe { now() }
  }
}
