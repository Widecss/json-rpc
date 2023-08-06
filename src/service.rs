use crate::common::{JsonRpcRequest, JsonRpcResponse};

pub trait RpcHandler {
    fn request(&self, req: JsonRpcRequest) -> JsonRpcResponse;
}
