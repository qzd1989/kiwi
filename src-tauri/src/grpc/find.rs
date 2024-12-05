use super::EMPTY_WEIGHT_POINT;
use super::RUN_TIME;
use crate::capture::FRAME;
use crate::common::PROJECT_DIR;
use crate::find as system_find;
use crate::utils::fs::exists;
use anyhow::Result;
use find::{
    find_service_client::FindServiceClient, find_service_server::FindService, FindImageReply,
    FindImageRequest,
};
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::Mutex;
use tonic::transport::channel::Channel;
use tonic::{Request, Response, Status};

pub mod find {
    tonic::include_proto!("find_kiwi");
}
#[derive(Debug, Default)]
pub struct FindServiceInstance {}
#[tonic::async_trait]
impl FindService for FindServiceInstance {
    async fn find_image(
        &self,
        request: Request<FindImageRequest>,
    ) -> Result<Response<FindImageReply>, Status> {
        if PROJECT_DIR.lock().unwrap().is_none() {
            return Ok(FindImageReply::empty().response());
        }
        if FRAME.lock().unwrap().is_none() {
            return Ok(FindImageReply::empty().response());
        }
        let request = request.into_inner();
        let frame = FRAME.lock().unwrap().clone().unwrap();
        let project_path = PROJECT_DIR.lock().unwrap().clone().unwrap();
        let project_pathbuf = PathBuf::from(project_path);
        let full_path = format!(
            "{}{}",
            project_pathbuf
                .join("resources")
                .join(request.path)
                .to_str()
                .unwrap()
                .to_string(),
            ".png"
        );
        println!("full path is {:?}", full_path.clone());
        if let Err(error) = exists(full_path.clone()) {
            println!("exists caused error: {}", error.to_string());
            return Ok(FindImageReply::empty().response());
        }
        if !exists(full_path.clone()).unwrap() {
            println!("file not exist");
            return Ok(FindImageReply::empty().response());
        }
        let (width, height) = (
            request.end_x - request.start_x,
            request.end_y - request.start_y,
        );
        if width <= 0 || height <= 0 {
            println!("width ({}) or height ({}) is zero", width, height);
            return Ok(FindImageReply::empty().response());
        }
        let template = image::open(full_path.clone()).unwrap().to_rgba8();
        if let Ok(weight_point) = system_find::image::find_one(
            frame,
            template,
            request.start_x,
            request.start_y,
            width,
            height,
            request.threshold,
        ) {
            let (x, y, weight) = (
                weight_point.point.x as i32,
                weight_point.point.y as i32,
                weight_point.weight,
            );
            return Ok(FindImageReply::new(x, y, weight).response());
        }
        Ok(FindImageReply::empty().response())
    }
}

impl FindImageReply {
    pub fn new(x: i32, y: i32, threshold: f64) -> Self {
        Self { x, y, threshold }
    }
    pub fn empty() -> Self {
        let (x, y, threshold) = EMPTY_WEIGHT_POINT;
        Self { x, y, threshold }
    }
    pub fn response(self) -> Response<Self> {
        Response::new(self)
    }

    pub fn to_tuple(self) -> (i32, i32, f64) {
        (self.x, self.y, self.threshold)
    }
}

impl FindImageRequest {
    pub fn new(
        path: String,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
        threshold: f64,
    ) -> Self {
        Self {
            path,
            start_x,
            start_y,
            end_x,
            end_y,
            threshold,
        }
    }
    pub fn send(self) -> FindImageReply {
        let result = RUN_TIME.block_on(async move {
            if CLIENT.lock().unwrap().is_none() {
                init_client().await;
            }
            let mut client = CLIENT.lock().unwrap().clone().unwrap();
            client.find_image(self).await.unwrap()
        });
        result.into_inner()
    }
}

async fn init_client() {
    let client = FindServiceClient::connect(format!("http://{}", super::ADDR))
        .await
        .unwrap();
    *CLIENT.lock().unwrap() = Some(client);
}

pub static CLIENT: LazyLock<Mutex<Option<FindServiceClient<Channel>>>> =
    LazyLock::new(|| Mutex::new(None));
