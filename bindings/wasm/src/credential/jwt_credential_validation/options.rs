// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

// Note This type from `identity_iota::credential::vc_jwt_validation` and are different from those in
// `identity_iota::credential`. In the future these new types for credential validation should either replace the old,
// or the old types should be updated.
use identity_iota::credential::vc_jwt_validation::CredentialValidationOptions as JwtCredentialValidationOptions;

/// Options to declare validation criteria when validating credentials.
// TODO: Perhaps rename this to CredentialValidationOptions (in which case the old `CredentialValidationOptions` should
// be updated or removed)-
#[wasm_bindgen(js_name = JwtCredentialValidationOptions)]
pub struct WasmJwtCredentialValidationOptions(pub(crate) JwtCredentialValidationOptions);

#[wasm_bindgen(js_class = JwtCredentialValidationOptions)]
impl WasmJwtCredentialValidationOptions {
  // TODO: Create a constructor where users can specify values (similar to how it was done with the old
  // WasmCredentialValidationOptions).

  /// Creates a new `JwtCredentialValidationOptions` with defaults.
  #[allow(clippy::should_implement_trait)]
  #[wasm_bindgen]
  pub fn default() -> WasmJwtCredentialValidationOptions {
    WasmJwtCredentialValidationOptions::from(JwtCredentialValidationOptions::default())
  }
}

impl_wasm_json!(WasmJwtCredentialValidationOptions, JwtCredentialValidationOptions);
impl_wasm_clone!(WasmJwtCredentialValidationOptions, JwtCredentialValidationOptions);

impl From<JwtCredentialValidationOptions> for WasmJwtCredentialValidationOptions {
  fn from(options: JwtCredentialValidationOptions) -> Self {
    Self(options)
  }
}

impl From<WasmJwtCredentialValidationOptions> for JwtCredentialValidationOptions {
  fn from(options: WasmJwtCredentialValidationOptions) -> Self {
    options.0
  }
}
