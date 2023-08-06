use crate::common::{JsonRpcRequest, JsonRpcResponse};

pub trait RpcService {
    fn request(&self, request: &JsonRpcRequest, response: &mut JsonRpcResponse);
}
