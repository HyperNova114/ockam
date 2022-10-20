use crate::nodes::models::policy::Policy;
use either::Either;
use minicbor::Decoder;
use ockam_abac::{Action, PolicyStorage, Resource};
use ockam_core::api::{Id, Request, Response, ResponseBuilder};
use ockam_core::Result;

use super::NodeManager;

impl NodeManager {
    pub(super) async fn add_policy(
        &self,
        resource: &str,
        action: &str,
        req: &Request<'_>,
        dec: &mut Decoder<'_>,
    ) -> Result<ResponseBuilder<()>> {
        let p: Policy = dec.decode()?;
        let r = Resource::new(resource);
        let a = Action::new(action);
        self.policies.set_policy(&r, &a, p.expression()).await?;
        Ok(Response::ok(req.id()))
    }

    pub(super) async fn get_policy(
        &self,
        id: Id,
        resource: &str,
        action: &str,
    ) -> Result<Either<ResponseBuilder<()>, ResponseBuilder<Policy>>> {
        let r = Resource::new(resource);
        let a = Action::new(action);
        if let Some(e) = self.policies.get_policy(&r, &a).await? {
            Ok(Either::Right(Response::ok(id).body(Policy::new(e))))
        } else {
            Ok(Either::Left(Response::not_found(id)))
        }
    }
}
