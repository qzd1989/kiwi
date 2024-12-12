use super::find::{GlobalFindImageReply, GlobalFindImageRequest};
use super::FindServiceInstance;
use super::{init_client, CLIENT};
use crate::grpc::{EMPTY_WEIGHT_POINT, RUN_TIME};
use crate::{capture::FRAME, common::PROJECT_DIR, utils::fs::exists};
use crate::{find as system_find, grpc};
use std::path::PathBuf;
use tonic::{Request, Response, Status};

pub async fn find(
    _: &FindServiceInstance,
    request: Request<GlobalFindImageRequest>,
) -> Result<Response<GlobalFindImageReply>, Status> {
    if PROJECT_DIR.lock().unwrap().is_none() {
        return Ok(GlobalFindImageReply::empty().response());
    }
    if FRAME.lock().unwrap().is_none() {
        return Ok(GlobalFindImageReply::empty().response());
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
        return Ok(GlobalFindImageReply::empty().response());
    }
    if !exists(full_path.clone()).unwrap() {
        println!("file not exist");
        return Ok(GlobalFindImageReply::empty().response());
    }
    let (width, height) = (frame.width, frame.height);
    if width <= 0 || height <= 0 {
        println!("width ({}) or height ({}) is zero", width, height);
        return Ok(GlobalFindImageReply::empty().response());
    }
    let template = image::open(full_path.clone()).unwrap().to_rgba8();
    if let Ok(weight_point) = system_find::image::find_one(
        frame,
        template,
        0 as u32,
        0 as u32,
        width,
        height,
        request.threshold,
    ) {
        let (x, y, weight) = (
            weight_point.point.x,
            weight_point.point.y,
            weight_point.weight,
        );
        return Ok(GlobalFindImageReply::new(x, y, weight).response());
    }
    Ok(GlobalFindImageReply::empty().response())
}

impl GlobalFindImageReply {
    pub fn new(x: f64, y: f64, threshold: f64) -> Self {
        let weight_point: grpc::WeightPoint = (x, y, threshold);
        let json = serde_json::to_string(&weight_point).unwrap();
        Self { json }
    }
    pub fn empty() -> Self {
        let json = serde_json::to_string(&EMPTY_WEIGHT_POINT).unwrap();
        Self { json }
    }
    pub fn response(self) -> Response<Self> {
        Response::new(self)
    }
    pub fn python_response(self) -> Option<grpc::WeightPoint> {
        let weight_point: grpc::WeightPoint = serde_json::from_str(&self.json).unwrap();
        if weight_point.0 == -1.0 {
            return None;
        }
        Some(weight_point)
    }
}

impl GlobalFindImageRequest {
    pub fn new(path: String, threshold: f64) -> Self {
        Self { path, threshold }
    }
    pub fn send(self) -> GlobalFindImageReply {
        let result = RUN_TIME.block_on(async move {
            if CLIENT.lock().unwrap().is_none() {
                init_client().await;
            }
            let mut client = CLIENT.lock().unwrap().clone().unwrap();
            client.global_find_image(self).await.unwrap()
        });
        result.into_inner()
    }
}
