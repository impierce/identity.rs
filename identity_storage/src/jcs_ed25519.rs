// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Ed25519SignatureAlgorithm;
use crate::KeyId;
use crate::KeyStorage;
use crate::Signable;
use crate::SignatureHandler;
use crate::SignatureMethodType;
use crate::StorageResult;
use async_trait::async_trait;
use identity_core::convert::ToJson;
use identity_core::crypto::ProofValue;
use identity_core::utils::BaseEncoding;
use identity_did::verification::MethodType;

pub struct JcsEd25519;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl<K> SignatureHandler<K> for JcsEd25519
where
  K: KeyStorage,
  K::SigningAlgorithm: From<Ed25519SignatureAlgorithm>,
{
  fn signature_name(&self) -> String {
    "JcsEd25519Signature2020".to_owned()
  }

  async fn sign(&self, data: Signable, private_key: KeyId, key_storage: &K) -> StorageResult<ProofValue> {
    let data: Vec<u8> = data.to_jcs().expect("TODO");
    let raw_signature: Vec<u8> = key_storage.sign(&private_key, Ed25519SignatureAlgorithm, data).await?.0;

    let signature: String = BaseEncoding::encode_base58(&raw_signature);

    Ok(ProofValue::Signature(signature))
  }
}

impl SignatureMethodType for JcsEd25519 {
  /// Returns the method type of a signature handler.
  fn method_type() -> MethodType {
    MethodType::ED25519_VERIFICATION_KEY_2018
  }
}