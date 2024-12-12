pub use find::find_service_server::FindServiceServer;
use find::{
    find_service_client::FindServiceClient, find_service_server::FindService, FindColorReply,
    FindImageReply, FindImagesReply, FindLocatingColorReply, FindTextReply, GlobalFindColorReply,
    GlobalFindImageReply, GlobalFindImagesReply, GlobalFindLocatingColorReply,
};
pub use find::{
    FindColorRequest, FindImageRequest, FindImagesRequest, FindLocatingColorRequest,
    FindTextRequest, GlobalFindColorRequest, GlobalFindImageRequest, GlobalFindImagesRequest,
    GlobalFindLocatingColorRequest,
};
use std::sync::LazyLock;
use std::sync::Mutex;
use tonic::transport::channel::Channel;
use tonic::{Request, Response, Status};
mod find_color;
mod find_locating_color;
mod find_text;
mod global_find_color;
mod global_find_image;
mod global_find_images;
mod global_find_locating_color;
mod image;
mod images;
mod find {
    tonic::include_proto!("find_kiwi");
}
#[derive(Debug, Default)]
pub struct FindServiceInstance {}

async fn init_client() {
    let client = FindServiceClient::connect(format!("http://{}", super::ADDR))
        .await
        .unwrap();
    *CLIENT.lock().unwrap() = Some(client);
}

#[tonic::async_trait]
impl FindService for FindServiceInstance {
    async fn find_image(
        &self,
        request: Request<FindImageRequest>,
    ) -> Result<Response<FindImageReply>, Status> {
        image::find(self, request).await
    }
    async fn find_images(
        &self,
        request: Request<FindImagesRequest>,
    ) -> Result<Response<FindImagesReply>, Status> {
        images::find(self, request).await
    }
    async fn find_locating_color(
        &self,
        request: Request<FindLocatingColorRequest>,
    ) -> Result<Response<FindLocatingColorReply>, Status> {
        find_locating_color::find(&self, request).await
    }
    async fn find_color(
        &self,
        request: Request<FindColorRequest>,
    ) -> Result<Response<FindColorReply>, Status> {
        find_color::find(&self, request).await
    }
    async fn find_text(
        &self,
        request: Request<FindTextRequest>,
    ) -> Result<Response<FindTextReply>, Status> {
        find_text::find(&self, request).await
    }
    async fn global_find_image(
        &self,
        request: Request<GlobalFindImageRequest>,
    ) -> Result<Response<GlobalFindImageReply>, Status> {
        global_find_image::find(&self, request).await
    }
    async fn global_find_images(
        &self,
        request: Request<GlobalFindImagesRequest>,
    ) -> Result<Response<GlobalFindImagesReply>, Status> {
        global_find_images::find(&self, request).await
    }
    async fn global_find_locating_color(
        &self,
        request: Request<GlobalFindLocatingColorRequest>,
    ) -> Result<Response<GlobalFindLocatingColorReply>, Status> {
        global_find_locating_color::find(&self, request).await
    }
    async fn global_find_color(
        &self,
        request: Request<GlobalFindColorRequest>,
    ) -> Result<Response<GlobalFindColorReply>, Status> {
        global_find_color::find(&self, request).await
    }
}

static CLIENT: LazyLock<Mutex<Option<FindServiceClient<Channel>>>> =
    LazyLock::new(|| Mutex::new(None));
