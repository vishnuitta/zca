use crate::csi::{
    server, GetPluginCapabilitiesRequest, GetPluginCapabilitiesResponse, GetPluginInfoRequest,
    GetPluginInfoResponse, ProbeRequest, ProbeResponse,
};
use futures::future;
use futures::future::ok;
use tower_grpc::{Request, Response, Status};

#[derive(Debug, Clone, Default)]
pub struct Identity {
    pub socket: String,
}

impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}

impl server::Identity for Identity {
    type GetPluginInfoFuture = future::FutureResult<Response<GetPluginInfoResponse>, Status>;
    type GetPluginCapabilitiesFuture =
        future::FutureResult<Response<GetPluginCapabilitiesResponse>, Status>;
    type ProbeFuture =
        Box<dyn future::Future<Item = Response<ProbeResponse>, Error = Status> + Send>;

    fn get_plugin_info(
        &mut self,
        _request: Request<GetPluginInfoRequest>,
    ) -> Self::GetPluginInfoFuture {
        ok(Response::new(GetPluginInfoResponse {
            name: "zfs csi provisioner".to_string(),
            vendor_version: "19.10".to_string(),
            manifest: Default::default(),
        }))
    }

    fn get_plugin_capabilities(
        &mut self,
        _request: Request<GetPluginCapabilitiesRequest>,
    ) -> Self::GetPluginCapabilitiesFuture {
        unimplemented!()
    }

    fn probe(&mut self, _request: Request<ProbeRequest>) -> Self::ProbeFuture {
        unimplemented!()
    }
}
