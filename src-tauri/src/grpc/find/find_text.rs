use super::find::{FindTextReply, FindTextRequest};
use super::FindServiceInstance;
use super::{init_client, CLIENT};
use crate::capture::FRAME;
use crate::grpc::RUN_TIME;
use crate::{find as system_find, grpc};
use tonic::{Request, Response, Status};
pub async fn find(
    _: &FindServiceInstance,
    request: Request<FindTextRequest>,
) -> Result<Response<FindTextReply>, Status> {
    if FRAME.lock().unwrap().is_none() {
        return Ok(FindTextReply::empty().response());
    }
    let request = request.into_inner();
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let (width, height) = (
        request.end_x - request.start_x,
        request.end_y - request.start_y,
    );
    if width <= 0 || height <= 0 {
        println!("width ({}) or height ({}) is zero", width, height);
        return Ok(FindTextReply::empty().response());
    }
    let langs: Vec<String> = serde_json::from_str(&request.langs).unwrap();
    match system_find::text::recognize(
        frame,
        langs,
        request.start_x,
        request.start_y,
        width,
        height,
    ) {
        Ok(text) => Ok(FindTextReply::new(text).response()),
        Err(_) => Ok(FindTextReply::empty().response()),
    }
}

impl FindTextReply {
    pub fn new(text: String) -> Self {
        Self { text }
    }
    pub fn empty() -> Self {
        Self {
            text: "".to_string(),
        }
    }
    pub fn response(self) -> Response<Self> {
        Response::new(self)
    }

    pub fn python_response(self) -> String {
        self.text
    }
}

impl FindTextRequest {
    pub fn new(
        langs: grpc::request::Langs,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
    ) -> Self {
        let langs = serde_json::to_string(&langs).unwrap();
        Self {
            langs,
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }
    pub fn send(self) -> FindTextReply {
        let result = RUN_TIME.block_on(async move {
            if CLIENT.lock().unwrap().is_none() {
                init_client().await;
            }
            let mut client = CLIENT.lock().unwrap().clone().unwrap();
            client.find_text(self).await.unwrap()
        });
        result.into_inner()
    }
}
