use super::find::{FindImagesReply, FindImagesRequest};
use super::FindServiceInstance;
use super::{init_client, CLIENT};
use crate::grpc::RUN_TIME;
use crate::{capture::FRAME, common::PROJECT_DIR, utils::fs::exists};
use crate::{find as system_find, grpc};
use std::path::PathBuf;
use tonic::{Request, Response, Status};
pub async fn find(
    _: &FindServiceInstance,
    request: Request<FindImagesRequest>,
) -> Result<Response<FindImagesReply>, Status> {
    let mut vecs = Vec::new();
    if PROJECT_DIR.lock().unwrap().is_none() {
        return Ok(FindImagesReply::empty().response());
    }
    if FRAME.lock().unwrap().is_none() {
        return Ok(FindImagesReply::empty().response());
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
        return Ok(FindImagesReply::empty().response());
    }
    if !exists(full_path.clone()).unwrap() {
        println!("file not exist");
        return Ok(FindImagesReply::empty().response());
    }
    let (width, height) = (
        request.end_x - request.start_x,
        request.end_y - request.start_y,
    );
    if width <= 0 || height <= 0 {
        println!("width ({}) or height ({}) is zero", width, height);
        return Ok(FindImagesReply::empty().response());
    }
    let template = image::open(full_path.clone()).unwrap().to_rgba8();
    match system_find::image::find_multiple(
        frame,
        template,
        request.start_x,
        request.start_y,
        width,
        height,
        request.threshold,
    ) {
        Ok(weight_points) => {
            for weight_point in weight_points {
                vecs.push((
                    weight_point.point.x as i32,
                    weight_point.point.y as i32,
                    weight_point.weight,
                ))
            }
            Ok(FindImagesReply::new(vecs).response())
        }
        Err(_) => Ok(FindImagesReply::empty().response()),
    }
}

impl FindImagesReply {
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

impl FindImagesRequest {
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
    pub fn send(self) -> FindImagesReply {
        let result = RUN_TIME.block_on(async move {
            if CLIENT.lock().unwrap().is_none() {
                init_client().await;
            }
            let mut client = CLIENT.lock().unwrap().clone().unwrap();
            client.find_images(self).await.unwrap()
        });
        result.into_inner()
    }
}
