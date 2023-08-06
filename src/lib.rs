pub mod common;
pub mod server;
pub mod service;


#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::str::FromStr;

    use serde_json::Value;

    use crate::common::{JsonRpcRequest, JsonRpcResponse};
    use crate::server::JsonRpcServer;
    use crate::service::RpcService;

    pub struct ExampleService {}

    impl ExampleService {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl RpcService for ExampleService {
        fn request(&self, request: &JsonRpcRequest, response: &mut JsonRpcResponse) {
            let ret = format!(
                "{} + {}",
                request.get_arg_value(&"a".to_string()).unwrap().to_string(),
                request.get_arg_value(&"b".to_string()).unwrap().as_str().unwrap()
            );
            response.set_result(Value::String(ret));
        }
    }


    #[test]
    fn it_works() {
        let addr = SocketAddr::from_str("0.0.0.0:11122");
        assert!(addr.is_ok(), "!!!!");

        let task = JsonRpcServer::new(Box::new(ExampleService::new()))
            .start(addr.unwrap());

        task.join().unwrap();
    }
}
