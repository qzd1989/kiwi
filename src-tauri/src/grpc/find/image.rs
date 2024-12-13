use super::{
    find::{FindImageReply, FindImageRequest},
    FindServiceInstance,
};
use super::{init_client, CLIENT};
use crate::grpc::RUN_TIME;
use crate::{capture::FRAME, common::PROJECT_DIR, utils::fs::exists};
use crate::{find as system_find, grpc};
use std::path::PathBuf;
use tonic::{Request, Response, Status};

pub async fn find(
    _: &FindServiceInstance,
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
        let weight_point = (
            weight_point.point.x as i32,
            weight_point.point.y as i32,
            weight_point.weight,
        );
        return Ok(FindImageReply::new(weight_point).response());
    }
    Ok(FindImageReply::empty().response())
}

impl FindImageReply {
    pub fn new(weight_point: grpc::response::WeightPoint) -> Self {
        let json = serde_json::to_string(&weight_point).unwrap();
        Self { json }
    }
    pub fn empty() -> Self {
        let json = serde_json::to_string(&grpc::response::EMPTY_WEIGHT_POINT).unwrap();
        Self { json }
    }
    pub fn response(self) -> Response<Self> {
        Response::new(self)
    }
    pub fn python_response(self) -> Option<grpc::response::WeightPoint> {
        let weight_point: grpc::response::WeightPoint = serde_json::from_str(&self.json).unwrap();
        if weight_point.0 == -1 {
            return None;
        }
        Some(weight_point)
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
