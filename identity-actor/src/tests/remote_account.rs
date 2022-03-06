// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account::identity::IdentitySetup;
use identity_iota::document::IotaDocument;

use crate::remote_account::IdentityCreate;
use crate::remote_account::IdentityGet;
use crate::remote_account::IdentityList;
use crate::remote_account::RemoteAccount;
use crate::tests::default_listening_actor;
use crate::tests::default_sending_actor;
use crate::tests::try_init_logger;

#[tokio::test]
async fn test_remote_account() -> crate::Result<()> {
  try_init_logger();

  let (receiver, receiver_addr, receiver_peer_id) = default_listening_actor(|builder| {
    builder
      .add_state(RemoteAccount::new().unwrap())
      .add_handler("remote_account/create", RemoteAccount::create)
      .unwrap()
      .add_handler("remote_account/list", RemoteAccount::list)
      .unwrap()
      .add_handler("remote_account/get", RemoteAccount::get)
      .unwrap();
  })
  .await;
  let mut sender = default_sending_actor(|_| {}).await;

  sender.add_address(receiver_peer_id, receiver_addr).await;

  let doc: IotaDocument = sender
    .send_request(receiver_peer_id, IdentityCreate(IdentitySetup::new()))
    .await?
    .unwrap();

  assert_eq!(sender.send_request(receiver_peer_id, IdentityList).await?.len(), 1);

  let doc2: IotaDocument = sender
    .send_request(receiver_peer_id, IdentityGet(doc.id().clone()))
    .await?
    .unwrap();

  assert_eq!(doc, doc2);

  sender.shutdown().await.unwrap();
  receiver.shutdown().await.unwrap();

  Ok(())
}
