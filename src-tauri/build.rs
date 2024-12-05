fn main() {
    tonic_build::compile_protos("src/grpc/proto/common.proto").unwrap();
    tauri_build::build()
}
