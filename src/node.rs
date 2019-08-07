use crate::csi::{
    server, NodeExpandVolumeRequest, NodeExpandVolumeResponse, NodeGetCapabilitiesRequest,
    NodeGetCapabilitiesResponse, NodeGetInfoRequest, NodeGetInfoResponse,
    NodeGetVolumeStatsRequest, NodeGetVolumeStatsResponse, NodePublishVolumeRequest,
    NodePublishVolumeResponse, NodeStageVolumeRequest, NodeStageVolumeResponse,
    NodeUnpublishVolumeRequest, NodeUnpublishVolumeResponse, NodeUnstageVolumeRequest,
    NodeUnstageVolumeResponse,
};
use futures::future::FutureResult;
use futures::Future;
use futures::future::{ok, err};
use tower_grpc::{Request, Response, Status, Code};
use libzfs_rs::zfs::{{LibZfs}};

/// our main structure
#[derive(Clone, Debug, Default)]
pub struct CsiNode {
    /// name of this node
    name: String,
    socket: String,
}

impl CsiNode {
    pub fn new() -> Self {
        Self::default()
    }
}

impl server::Node for CsiNode {
    type NodeStageVolumeFuture =
        Box<dyn Future<Item = Response<NodeStageVolumeResponse>, Error = Status> + Send>;
    type NodeUnstageVolumeFuture =
        Box<dyn Future<Item = Response<NodeUnstageVolumeResponse>, Error = Status> + Send>;
    type NodePublishVolumeFuture =
        Box<dyn Future<Item = Response<NodePublishVolumeResponse>, Error = Status> + Send>;
    type NodeUnpublishVolumeFuture =
        Box<dyn Future<Item = Response<NodeUnpublishVolumeResponse>, Error = Status> + Send>;
    type NodeGetVolumeStatsFuture =
        Box<dyn Future<Item = Response<NodeGetVolumeStatsResponse>, Error = Status> + Send>;
    type NodeExpandVolumeFuture = FutureResult<Response<NodeExpandVolumeResponse>, Status>;
    type NodeGetCapabilitiesFuture = FutureResult<Response<NodeGetCapabilitiesResponse>, Status>;
    type NodeGetInfoFuture = FutureResult<Response<NodeGetInfoResponse>, Status>;

    fn node_stage_volume(
        &mut self,
        _request: Request<NodeStageVolumeRequest>,
    ) -> Self::NodeStageVolumeFuture {
        unimplemented!()
    }

    fn node_unstage_volume(
        &mut self,
        _request: Request<NodeUnstageVolumeRequest>,
    ) -> Self::NodeUnstageVolumeFuture {
        unimplemented!()
    }

    fn node_publish_volume(
        &mut self,
        request: Request<NodePublishVolumeRequest>,
    ) -> Self::NodePublishVolumeFuture {
        let libzfs_handle = LibZfs::new().unwrap();

        let mut pool = "pool1/".to_string();
        pool.push_str(&request.get_ref().volume_id);

        println!("{}", pool);
//        dbg!(&pool);

        let result = libzfs_handle.create_filesystem(pool.as_str());
        let reply = NodePublishVolumeResponse{};
        let mut f = ok(Response::new(reply));
        if result.is_err() {
            f = err(Status::new(Code::Internal, result.err().unwrap().to_string()));
        }
        Box::new(f)
    }

    fn node_unpublish_volume(
        &mut self,
        _request: Request<NodeUnpublishVolumeRequest>,
    ) -> Self::NodeUnpublishVolumeFuture {
        unimplemented!()
    }

    fn node_get_volume_stats(
        &mut self,
        _request: Request<NodeGetVolumeStatsRequest>,
    ) -> Self::NodeGetVolumeStatsFuture {
        unimplemented!()
    }

    fn node_expand_volume(
        &mut self,
        _request: Request<NodeExpandVolumeRequest>,
    ) -> Self::NodeExpandVolumeFuture {
        unimplemented!()
    }

    fn node_get_capabilities(
        &mut self,
        _request: Request<NodeGetCapabilitiesRequest>,
    ) -> Self::NodeGetCapabilitiesFuture {
        unimplemented!()
    }

    fn node_get_info(&mut self, _request: Request<NodeGetInfoRequest>) -> Self::NodeGetInfoFuture {
        unimplemented!()
    }
}
