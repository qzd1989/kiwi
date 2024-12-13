use super::find::{FindColorReply, FindColorRequest};
use super::FindServiceInstance;
use super::{init_client, CLIENT};
use crate::capture::FRAME;
use crate::grpc::RUN_TIME;
use crate::{find as system_find, grpc};
use tonic::{Request, Response, Status};
pub async fn find(
    _: &FindServiceInstance,
    request: Request<FindColorRequest>,
) -> Result<Response<FindColorReply>, Status> {
    if FRAME.lock().unwrap().is_none() {
        return Ok(FindColorReply::empty().response());
    }
    let request = request.into_inner();
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let (width, height) = (
        request.end_x - request.start_x,
        request.end_y - request.start_y,
    );
    if width <= 0 || height <= 0 {
        println!("width ({}) or height ({}) is zero", width, height);
        return Ok(FindColorReply::empty().response());
    }
    let colors: grpc::request::HexColors = serde_json::from_str(&request.hex_colors).unwrap();
    match system_find::color::find(
        frame,
        colors,
        request.start_x,
        request.start_y,
        width,
        height,
        request.offset_r as u8,
        request.offset_g as u8,
        request.offset_b as u8,
    ) {
        Ok(locating_colors) => {
            let locating_colors: grpc::response::LocatingColors = locating_colors
                .iter()
                .map(|lc| {
                    let locating_color: grpc::response::LocatingColor =
                        (lc.point.x as i32, lc.point.y as i32, Some(lc.hex.clone()));
                    locating_color
                })
                .collect();
            Ok(FindColorReply::new(locating_colors).response())
        }
        Err(_) => Ok(FindColorReply::empty().response()),
    }
}

impl FindColorReply {
    pub fn new(locating_colors: grpc::response::LocatingColors) -> Self {
        let json = serde_json::to_string(&locating_colors).unwrap();
        Self { json }
    }
    pub fn empty() -> Self {
        let json = serde_json::to_string(&grpc::response::EMPTY_LOCATING_COLORS).unwrap();
        Self { json }
    }
    pub fn response(self) -> Response<Self> {
        Response::new(self)
    }
    pub fn python_response(self) -> grpc::response::LocatingColors {
        let locating_colors: grpc::response::LocatingColors =
            serde_json::from_str(&self.json).unwrap();
        locating_colors
    }
}

impl FindColorRequest {
    pub fn new(
        hex_colors: grpc::request::HexColors,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
        offset_r: u32,
        offset_g: u32,
        offset_b: u32,
    ) -> Self {
        let hex_colors = serde_json::to_string(&hex_colors).unwrap();
        Self {
            hex_colors,
            start_x,
            start_y,
            end_x,
            end_y,
            offset_r,
            offset_g,
            offset_b,
        }
    }
    pub fn send(self) -> FindColorReply {
        let result = RUN_TIME.block_on(async move {
            if CLIENT.lock().unwrap().is_none() {
                init_client().await;
            }
            let mut client = CLIENT.lock().unwrap().clone().unwrap();
            client.find_color(self).await.unwrap()
        });
        result.into_inner()
    }
}
