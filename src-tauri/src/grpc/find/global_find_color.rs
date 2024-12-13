use super::find::{GlobalFindColorReply, GlobalFindColorRequest};
use super::FindServiceInstance;
use super::{init_client, CLIENT};
use crate::capture::FRAME;
use crate::grpc::RUN_TIME;
use crate::{find as system_find, grpc};
use tonic::{Request, Response, Status};
pub async fn find(
    _: &FindServiceInstance,
    request: Request<GlobalFindColorRequest>,
) -> Result<Response<GlobalFindColorReply>, Status> {
    if FRAME.lock().unwrap().is_none() {
        return Ok(GlobalFindColorReply::empty().response());
    }
    let request = request.into_inner();
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let (width, height) = (frame.width, frame.height);
    if width <= 0 || height <= 0 {
        println!("width ({}) or height ({}) is zero", width, height);
        return Ok(GlobalFindColorReply::empty().response());
    }
    let colors: grpc::request::HexColors = serde_json::from_str(&request.hex_colors).unwrap();
    match system_find::color::find(
        frame,
        colors,
        0 as u32,
        0 as u32,
        width,
        height,
        request.offset_r as u8,
        request.offset_g as u8,
        request.offset_b as u8,
    ) {
        Ok(locating_colors) => {
            let locating_colors: grpc::response::LocatingColors = locating_colors
                .into_iter()
                .map(|lc| {
                    let locating_color: grpc::response::LocatingColor =
                        (lc.point.x as i32, lc.point.y as i32, Some(lc.hex));
                    locating_color
                })
                .collect();
            Ok(GlobalFindColorReply::new(locating_colors).response())
        }
        Err(_) => Ok(GlobalFindColorReply::empty().response()),
    }
}

impl GlobalFindColorReply {
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

impl GlobalFindColorRequest {
    pub fn new(
        hex_colors: grpc::request::HexColors,
        offset_r: u32,
        offset_g: u32,
        offset_b: u32,
    ) -> Self {
        let hex_colors = serde_json::to_string(&hex_colors).unwrap();
        Self {
            hex_colors,

            offset_r,
            offset_g,
            offset_b,
        }
    }
    pub fn send(self) -> GlobalFindColorReply {
        let result = RUN_TIME.block_on(async move {
            if CLIENT.lock().unwrap().is_none() {
                init_client().await;
            }
            let mut client = CLIENT.lock().unwrap().clone().unwrap();
            client.global_find_color(self).await.unwrap()
        });
        result.into_inner()
    }
}
