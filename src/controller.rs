use crate::csi::{
    server, ControllerExpandVolumeRequest, ControllerExpandVolumeResponse,
    ControllerGetCapabilitiesRequest, ControllerGetCapabilitiesResponse,
    ControllerPublishVolumeRequest, ControllerPublishVolumeResponse,
    ControllerUnpublishVolumeRequest, ControllerUnpublishVolumeResponse, CreateSnapshotRequest,
    CreateSnapshotResponse, CreateVolumeRequest, CreateVolumeResponse, DeleteSnapshotRequest,
    DeleteSnapshotResponse, DeleteVolumeRequest, DeleteVolumeResponse, GetCapacityRequest,
    GetCapacityResponse, ListSnapshotsRequest, ListSnapshotsResponse, ListVolumesRequest,
    ListVolumesResponse, ValidateVolumeCapabilitiesRequest, ValidateVolumeCapabilitiesResponse,
};
use futures::Future;
use tower_grpc::{Request, Response, Status};

#[derive(Debug, Clone, Default)]
pub struct Controller {
    socket: String,
}

impl Controller {
    pub fn new() -> Self {
        Self::default()
    }
}

impl server::Controller for Controller {
    type CreateVolumeFuture =
        Box<dyn Future<Item = Response<CreateVolumeResponse>, Error = Status> + Send>;
    type DeleteVolumeFuture =
        Box<dyn Future<Item = Response<DeleteVolumeResponse>, Error = Status> + Send>;
    type ControllerPublishVolumeFuture =
        Box<dyn Future<Item = Response<ControllerPublishVolumeResponse>, Error = Status> + Send>;
    type ControllerUnpublishVolumeFuture =
        Box<dyn Future<Item = Response<ControllerUnpublishVolumeResponse>, Error = Status> + Send>;
    type ValidateVolumeCapabilitiesFuture =
        Box<dyn Future<Item = Response<ValidateVolumeCapabilitiesResponse>, Error = Status> + Send>;
    type ListVolumesFuture =
        Box<dyn Future<Item = Response<ListVolumesResponse>, Error = Status> + Send>;
    type GetCapacityFuture =
        Box<dyn Future<Item = Response<GetCapacityResponse>, Error = Status> + Send>;
    type ControllerGetCapabilitiesFuture =
        Box<dyn Future<Item = Response<ControllerGetCapabilitiesResponse>, Error = Status> + Send>;
    type CreateSnapshotFuture =
        Box<dyn Future<Item = Response<CreateSnapshotResponse>, Error = Status> + Send>;
    type DeleteSnapshotFuture =
        Box<dyn Future<Item = Response<DeleteSnapshotResponse>, Error = Status> + Send>;
    type ListSnapshotsFuture =
        Box<dyn Future<Item = Response<ListSnapshotsResponse>, Error = Status> + Send>;
    type ControllerExpandVolumeFuture =
        Box<dyn Future<Item = Response<ControllerExpandVolumeResponse>, Error = Status> + Send>;

    fn create_volume(
        &mut self,
        _request: Request<CreateVolumeRequest>,
    ) -> Self::CreateVolumeFuture {
        unimplemented!()
    }

    fn delete_volume(
        &mut self,
        _request: Request<DeleteVolumeRequest>,
    ) -> Self::DeleteVolumeFuture {
        unimplemented!()
    }

    fn controller_publish_volume(
        &mut self,
        _request: Request<ControllerPublishVolumeRequest>,
    ) -> Self::ControllerPublishVolumeFuture {
        unimplemented!()
    }

    fn controller_unpublish_volume(
        &mut self,
        _request: Request<ControllerUnpublishVolumeRequest>,
    ) -> Self::ControllerUnpublishVolumeFuture {
        unimplemented!()
    }

    fn validate_volume_capabilities(
        &mut self,
        _request: Request<ValidateVolumeCapabilitiesRequest>,
    ) -> Self::ValidateVolumeCapabilitiesFuture {
        unimplemented!()
    }

    fn list_volumes(&mut self, _request: Request<ListVolumesRequest>) -> Self::ListVolumesFuture {
        unimplemented!()
    }

    fn get_capacity(&mut self, _request: Request<GetCapacityRequest>) -> Self::GetCapacityFuture {
        unimplemented!()
    }

    fn controller_get_capabilities(
        &mut self,
        _request: Request<ControllerGetCapabilitiesRequest>,
    ) -> Self::ControllerGetCapabilitiesFuture {
        unimplemented!()
    }

    fn create_snapshot(
        &mut self,
        _request: Request<CreateSnapshotRequest>,
    ) -> Self::CreateSnapshotFuture {
        unimplemented!()
    }

    fn delete_snapshot(
        &mut self,
        _request: Request<DeleteSnapshotRequest>,
    ) -> Self::DeleteSnapshotFuture {
        unimplemented!()
    }

    fn list_snapshots(
        &mut self,
        _request: Request<ListSnapshotsRequest>,
    ) -> Self::ListSnapshotsFuture {
        unimplemented!()
    }

    fn controller_expand_volume(
        &mut self,
        _request: Request<ControllerExpandVolumeRequest>,
    ) -> Self::ControllerExpandVolumeFuture {
        unimplemented!()
    }
}
