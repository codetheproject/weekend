use crate::security::Provider;

pub struct Context<S, P>
where
    P: Provider<S>,
{
    state: S,
    provider: P,
    // This will basically hold a context we don't know if we should support multiple context tho
    // context: Arc<Box<dyn >>
}

pub mod handler {
    use std::future::Future;

    pub trait Handler<State> {
        type Error;

        /// The name of the handler
        /// This is used to identify the handler in logs and other contexts
        const NAME: &'static str;

        /// Authenticate the current request
        ///
        /// This method is called to authenticate the current request
        fn authenticate(&self) -> impl Future<Output = Result<(), Self::Error>> + Send;

        /// Forbid the current request
        ///
        /// This method is called to forbid the current request
        ///
        /// # Arguments
        /// `state` - The current state of the request `S`
        fn forbid(&self, state: &State) -> impl Future<Output = Result<(), Self::Error>> + Send;

        /// Challenge the current request
        ///
        /// This method is called to challenge the current request
        ///
        /// # Arguments
        /// `state` - The current state of the request `Self::State`
        fn challenge(&self, state: &State) -> impl Future<Output = Result<(), Self::Error>> + Send;
    }
}

pub mod sign_out {
    use super::handler::Handler;
    use std::future::Future;

    pub trait SignOutHandler<State>: Handler<State> {
        type Claim;

        fn sign_out(&self, state: &State, claim: &Self::Claim) -> impl Future<Output = Result<(), Self::Error>> + Send;
    }

    pub struct SignOutContext<State, Handler: SignOutHandler<State>> {
        handler: Handler,
        before_signout: Box<dyn Fn(&State, &Handler::Claim)>,
        after_signout: Box<dyn Fn(&State, &Handler::Claim)>,
    }
}

pub mod sign_in {
    use super::sign_out::SignOutHandler;
    use std::future::Future;

    pub trait SignInHandler<State>: SignOutHandler<State> {
        fn sign_in(&self, state: &State, claim: &Self::Claim) -> impl Future<Output = Result<(), Self::Error>> + Send;
    }

    pub struct SignInContext<State, Handler: SignOutHandler<State>> {
        handler: Handler,
        before_signout: Box<dyn Fn(&State, &Handler::Claim)>,
        after_signout: Box<dyn Fn(&State, &Handler::Claim)>,
    }
}

#[cfg(test)]
mod test {}
