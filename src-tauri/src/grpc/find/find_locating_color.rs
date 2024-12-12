use super::find::{FindLocatingColorReply, FindLocatingColorRequest};
use super::FindServiceInstance;
use super::{init_client, CLIENT};
use crate::capture::FRAME;
use crate::common::{LocatingColor, Point};
use crate::grpc::{EMPTY_LOCATING_COLOR, RUN_TIME};
use crate::{find as system_find, grpc};
use tonic::{Request, Response, Status};

pub async fn find(
    _: &FindServiceInstance,
    request: Request<FindLocatingColorRequest>,
) -> Result<Response<FindLocatingColorReply>, Status> {
    if FRAME.lock().unwrap().is_none() {
        return Ok(FindLocatingColorReply::empty().response());
    }
    let request = request.into_inner();
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let (width, height) = (
        request.end_x - request.start_x,
        request.end_y - request.start_y,
    );
    if width <= 0 || height <= 0 {
        println!("width ({}) or height ({}) is zero", width, height);
        return Ok(FindLocatingColorReply::empty().response());
    }
    let locating_colors: grpc::LocatingColors =
        serde_json::from_str(&request.locating_colors).unwrap();
    let locating_colors: Vec<LocatingColor> = locating_colors
        .iter()
        .map(|lc| LocatingColor::new(Point::new(lc.0, lc.1), lc.2.clone().unwrap()))
        .collect();
    match system_find::locating_color::find(
        frame,
        locating_colors,
        request.start_x,
        request.start_y,
        width,
        height,
        request.offset_r as u8,
        request.offset_g as u8,
        request.offset_b as u8,
    ) {
        Some(point) => Ok(FindLocatingColorReply::new(point).response()),
        None => Ok(FindLocatingColorReply::empty().response()),
    }
}

impl FindLocatingColorReply {
    pub fn new(locating_color: LocatingColor) -> Self {
        let locating_color: grpc::LocatingColor = (
            locating_color.point.x,
            locating_color.point.y,
            Some(locating_color.hex),
        );
        let json = serde_json::to_string(&locating_color).unwrap();
        Self { json }
    }
    pub fn empty() -> Self {
        let json = serde_json::to_string(&EMPTY_LOCATING_COLOR).unwrap();
        Self { json }
    }
    pub fn response(self) -> Response<Self> {
        Response::new(self)
    }
    pub fn python_response(self) -> Option<grpc::LocatingColor> {
        let locating_color: grpc::LocatingColor = serde_json::from_str(&self.json).unwrap();
        if locating_color.0 == -1.0 {
            return None;
        }
        Some(locating_color)
    }
}

impl FindLocatingColorRequest {
    pub fn new(
        locating_colors: grpc::LocatingColors,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
        offset_r: u32,
        offset_g: u32,
        offset_b: u32,
    ) -> Self {
        let locating_colors = serde_json::to_string(&locating_colors).unwrap();
        Self {
            locating_colors,
            start_x,
            start_y,
            end_x,
            end_y,
            offset_r,
            offset_g,
            offset_b,
        }
    }
    pub fn send(self) -> FindLocatingColorReply {
        let result = RUN_TIME.block_on(async move {
            if CLIENT.lock().unwrap().is_none() {
                init_client().await;
            }
            let mut client = CLIENT.lock().unwrap().clone().unwrap();
            client.find_locating_color(self).await.unwrap()
        });
        result.into_inner()
    }
}
