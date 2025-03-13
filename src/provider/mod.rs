use std::future::Future;

use crate::context::{handler::Handler, sign_in::SignInHandler, sign_out::SignOutHandler};

pub trait Provider<State> {
    type Forbid: Handler<State>;
    type SignIn: SignInHandler<State>;
    type Signout: SignOutHandler<State>;
    type Challenge: Handler<State>;
    type Authenticate: Handler<State>;

    fn get_forbid<Request>(&self, req: Request) -> impl Future<Output = Self::Forbid> + Send;
    fn get_sign_in<Request>(&self, req: Request) -> impl Future<Output = Self::SignIn> + Send;
    fn get_sign_out<Request>(&self, req: Request) -> impl Future<Output = Self::Signout> + Send;
    fn get_challenge<Request>(&self, req: Request) -> impl Future<Output = Self::Challenge> + Send;
    fn get_authenticate<Request>(&self, req: Request) -> impl Future<Output = Self::Authenticate> + Send;
}
