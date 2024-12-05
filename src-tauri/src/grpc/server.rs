use super::find::{find::find_service_server::FindServiceServer, FindServiceInstance};
use super::RUN_TIME;
use anyhow::Result;
use tonic::transport::Server;

pub async fn run() -> Result<()> {
    let addr = super::ADDR.parse()?;
    let instance = FindServiceInstance::default();
    Server::builder()
        .add_service(FindServiceServer::new(instance))
        .serve(addr)
        .await?;
    Ok(())
}

pub fn run_spawn() {
    RUN_TIME.spawn(async {
        run().await.unwrap();
    });
}
//grpcurl -plaintext -import-path /Users/kiwi/Documents/Rust/kiwi/src-tauri/src/grpc/proto -proto common.proto -d '{"path": "payment", "startX": 457, "startY": 88, "endX": 711, "endY": 946, "threshold": 590.2428442764207}' '[::1]:50555' find_kiwi.FindService/FindImage
