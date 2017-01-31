use tokio_service::Service;
use std::fmt;
use futures::{Future, Then, future};
use futures::future::FutureResult;

#[derive(Clone)]
pub struct Log<S> {
    upstream: S,
}

impl<S> Log<S> {
    pub fn new(upstream: S) -> Log<S> {
        Log { upstream: upstream }
    }
}

impl<S> Service for Log<S>
    where S: Service,
          S::Request: fmt::Debug,
          S::Response: fmt::Debug,
          S::Error: fmt::Debug
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = Then<S::Future,
         FutureResult<S::Response, S::Error>,
         fn(Result<S::Response, S::Error>) -> FutureResult<Self::Response, Self::Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        println!("[REQUEST] {:?}", request);

        fn log_result<R: fmt::Debug, E: fmt::Debug>(result: Result<R, E>) -> FutureResult<R, E> {
            match result {
                Ok(ref response) => println!("[RESPONSE] {:?}", response),
                Err(ref error) => println!("[ERROR] {:?}", error),
            }
            future::result(result)
        }

        self.upstream.call(request).then(log_result)
    }
}
