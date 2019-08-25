//! Implements support for the srml_balances module.
use crate::{
    // codec::compact,
    error::Error,
    srml::system::System,
    // Client,
    XtBuilder,
};
use futures::future::{
    self,
    Future,
};
use parity_scale_codec::Codec;
use runtime_primitives::traits::{
//    MaybeSerializeDebug,
//    Member,
//    SimpleArithmetic,
    StaticLookup,
};
//use runtime_support::Parameter;
use substrate_primitives::Pair;

pub trait Litentry: System {

}

pub trait LitentryCalls {

    type Litentry: Litentry;

    fn register_identity(
        &mut self,
    ) -> Box<dyn Future<Item = <Self::Litentry as System>::Hash, Error = Error> + Send>;
}

impl<T: Litentry + 'static, P> LitentryCalls for XtBuilder<T, P>
    where
    P: Pair,
    P::Public: Into<<<T as System>::Lookup as StaticLookup>::Source>,
    P::Signature: Codec,
{

    type Litentry = T;
    fn register_identity(
        &mut self,
    ) -> Box<dyn Future<Item = <Self::Litentry as System>::Hash, Error = Error> + Send>
    {
        let transfer_call = || {
            Ok(self
                .metadata()
                .module("Litentry")?
                .call("register_identity", ())?)
        };
        let call = match transfer_call() {
            Ok(call) => call,
            Err(err) => return Box::new(future::err(err)),
        };
        Box::new(self.submit(call))
    }
}
