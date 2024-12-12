use super::find::{GlobalFindImagesReply, GlobalFindImagesRequest};
use super::FindServiceInstance;
use super::{init_client, CLIENT};
use crate::grpc::RUN_TIME;
use crate::{capture::FRAME, common::PROJECT_DIR, utils::fs::exists};
use crate::{find as system_find, grpc};
use std::path::PathBuf;
use tonic::{Request, Response, Status};
pub async fn find(
    _: &FindServiceInstance,
    request: Request<GlobalFindImagesRequest>,
) -> Result<Response<GlobalFindImagesReply>, Status> {
    let mut vecs = Vec::new();
    if PROJECT_DIR.lock().unwrap().is_none() {
        return Ok(GlobalFindImagesReply::empty().response());
    }
    if FRAME.lock().unwrap().is_none() {
        return Ok(GlobalFindImagesReply::empty().response());
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
    println!("full_path: {}", full_path.clone());
    if let Err(error) = exists(full_path.clone()) {
        println!("exists caused error: {}", error.to_string());
        return Ok(GlobalFindImagesReply::empty().response());
    }
    if !exists(full_path.clone()).unwrap() {
        println!("file not exist");
        return Ok(GlobalFindImagesReply::empty().response());
    }
    let (width, height) = (frame.width, frame.height);
    if width <= 0 || height <= 0 {
        println!("width ({}) or height ({}) is zero", width, height);
        return Ok(GlobalFindImagesReply::empty().response());
    }
    let template = image::open(full_path.clone()).unwrap().to_rgba8();
    match system_find::image::find_multiple(
        frame,
        template,
        0 as u32,
        0 as u32,
        width,
        height,
        request.threshold,
    ) {
        Ok(weight_points) => {
            for weight_point in weight_points {
                vecs.push((
                    weight_point.point.x,
                    weight_point.point.y,
                    weight_point.weight,
                ))
            }
            Ok(GlobalFindImagesReply::new(vecs).response())
        }
        Err(_) => Ok(GlobalFindImagesReply::empty().response()),
    }
}

impl GlobalFindImagesReply {
    pub fn new(weight_points: Vec<grpc::WeightPoint>) -> Self {
        let json = serde_json::to_string(&weight_points).unwrap();
        Self { json }
    }
    pub fn empty() -> Self {
        let json = serde_json::to_string(&grpc::EMPTY_WEIGHT_POINTS).unwrap();
        Self { json }
    }
    pub fn response(self) -> Response<Self> {
        Response::new(self)
    }
    pub fn python_response(self) -> grpc::WeightPoints {
        let weight_points: grpc::WeightPoints = serde_json::from_str(&self.json).unwrap();
        weight_points
    }
}

impl GlobalFindImagesRequest {
    pub fn new(path: String, threshold: f64) -> Self {
        Self { path, threshold }
    }
    pub fn send(self) -> GlobalFindImagesReply {
        let result = RUN_TIME.block_on(async move {
            if CLIENT.lock().unwrap().is_none() {
                init_client().await;
            }
            let mut client = CLIENT.lock().unwrap().clone().unwrap();
            client.global_find_images(self).await.unwrap()
        });
        result.into_inner()
    }
}
