// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::any::TypeId;
use std::marker::PhantomData;

use futures::Future;

use crate::traits::AnyFuture;
use crate::traits::RequestHandler;
use crate::Actor;
use crate::ActorRequest;
use crate::RemoteSendError;
use crate::RequestContext;

#[derive(Clone)]
pub struct Handler<OBJ, REQ, FUT, FUN>
where
  OBJ: 'static,
  REQ: ActorRequest,
  FUT: Future<Output = REQ::Response>,
  FUN: Fn(OBJ, Actor, RequestContext<REQ>) -> FUT,
{
  func: FUN,
  // Need to use the types that appear in the closure's arguments here,
  // as it is otherwise considered unused.
  // Since this type does not actually own any of these types, we use a reference.
  // See also the drop check section in the PhantomData doc.
  _marker_obj: PhantomData<&'static OBJ>,
  _marker_req: PhantomData<&'static REQ>,
}

impl<OBJ, REQ, FUT, FUN> Handler<OBJ, REQ, FUT, FUN>
where
  OBJ: 'static,
  REQ: ActorRequest,
  FUT: Future<Output = REQ::Response>,
  FUN: Fn(OBJ, Actor, RequestContext<REQ>) -> FUT,
{
  pub fn new(func: FUN) -> Self {
    Self {
      func,
      _marker_obj: PhantomData,
      _marker_req: PhantomData,
    }
  }
}

impl<OBJ, REQ, FUT, FUN> RequestHandler for Handler<OBJ, REQ, FUT, FUN>
where
  OBJ: Clone + Send + Sync + 'static,
  REQ: ActorRequest + Send + Sync,
  REQ::Response: Send,
  FUT: Future<Output = REQ::Response> + Send,
  FUN: Send + Sync + Fn(OBJ, Actor, RequestContext<REQ>) -> FUT,
{
  fn invoke(
    &self,
    actor: Actor,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    request: Box<dyn Any + Send>,
  ) -> Result<AnyFuture<'_>, RemoteSendError> {
    let input: Box<REQ> = request.downcast().map_err(|_| {
      RemoteSendError::HandlerInvocationError(format!(
        "{}: could not downcast request to: {}",
        context.endpoint,
        std::any::type_name::<REQ>()
      ))
    })?;

    let request: RequestContext<REQ> = context.convert(*input);

    let boxed_object: Box<OBJ> = object.downcast().map_err(|_| {
      RemoteSendError::HandlerInvocationError(format!(
        "{}: could not downcast state object to: {}",
        request.endpoint,
        std::any::type_name::<OBJ>()
      ))
    })?;
    let future = async move {
      let response: REQ::Response = (self.func)(*boxed_object, actor, request).await;
      let type_erased: Box<dyn Any + Send> = Box::new(response);
      type_erased
    };
    Ok(Box::pin(future))
  }

  fn serialize_response(&self, input: Box<dyn Any>) -> Result<Vec<u8>, RemoteSendError> {
    crate::traits::request_handler_serialize_response::<REQ>(input)
  }

  fn deserialize_request(&self, input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError> {
    crate::traits::request_handler_deserialize_request::<REQ>(input)
  }

  fn object_type_id(&self) -> TypeId {
    crate::traits::request_handler_object_type_id::<OBJ>()
  }

  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Box<dyn Any + Send + Sync> {
    crate::traits::request_handler_clone_object::<OBJ>(object)
  }
}