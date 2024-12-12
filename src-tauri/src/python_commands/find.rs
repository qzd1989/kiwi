use crate::grpc;
use pyo3::prelude::*;

#[pyfunction]
pub fn find_image(
    path: String, //such as "subfolder/imageFileName"
    start: (u32, u32),
    end: (u32, u32),
    threshold: f64,
) -> Option<grpc::WeightPoint> {
    let (start_x, start_y, end_x, end_y) = (start.0, start.1, end.0, end.1);
    grpc::FindImageRequest::new(path, start_x, start_y, end_x, end_y, threshold)
        .send()
        .python_response()
}

#[pyfunction]
pub fn find_images(
    path: String, //such as "subfolder/imageFileName"
    start: (u32, u32),
    end: (u32, u32),
    threshold: f64,
) -> grpc::WeightPoints {
    let (start_x, start_y, end_x, end_y) = (start.0, start.1, end.0, end.1);
    grpc::FindImagesRequest::new(path, start_x, start_y, end_x, end_y, threshold)
        .send()
        .python_response()
}

#[pyfunction]
pub fn global_find_image(
    path: String, //such as "subfolder/imageFileName"
    threshold: f64,
) -> Option<grpc::WeightPoint> {
    grpc::GlobalFindImageRequest::new(path, threshold)
        .send()
        .python_response()
}

#[pyfunction]
pub fn global_find_images(
    path: String, //such as "subfolder/imageFileName"
    threshold: f64,
) -> grpc::WeightPoints {
    grpc::GlobalFindImagesRequest::new(path, threshold)
        .send()
        .python_response()
}

#[pyfunction]
pub fn find_locating_color(
    locating_colors: grpc::LocatingColors,
    start: (u32, u32),
    end: (u32, u32),
    offsets: (u32, u32, u32),
) -> Option<grpc::Point> {
    let (start_x, start_y, end_x, end_y) = (start.0, start.1, end.0, end.1);
    let (offset_r, offset_g, offset_b) = (offsets.0, offsets.1, offsets.2);
    grpc::FindLocatingColorRequest::new(
        locating_colors,
        start_x,
        start_y,
        end_x,
        end_y,
        offset_r,
        offset_g,
        offset_b,
    )
    .send()
    .python_response()
}

#[pyfunction]
pub fn global_find_locating_color(
    locating_colors: grpc::LocatingColors,
    offsets: (u32, u32, u32),
) -> Option<grpc::Point> {
    let (offset_r, offset_g, offset_b) = (offsets.0, offsets.1, offsets.2);
    grpc::GlobalFindLocatingColorRequest::new(locating_colors, offset_r, offset_g, offset_b)
        .send()
        .python_response()
}

#[pyfunction]
pub fn find_color(
    hex_colors: grpc::HexColors,
    start: (u32, u32),
    end: (u32, u32),
    offsets: (u32, u32, u32),
) -> grpc::LocatingColors {
    let (start_x, start_y, end_x, end_y) = (start.0, start.1, end.0, end.1);
    let (offset_r, offset_g, offset_b) = (offsets.0, offsets.1, offsets.2);
    grpc::FindColorRequest::new(
        hex_colors, start_x, start_y, end_x, end_y, offset_r, offset_g, offset_b,
    )
    .send()
    .python_response()
}

#[pyfunction]
pub fn global_find_color(
    hex_colors: grpc::HexColors,
    offsets: (u32, u32, u32),
) -> grpc::LocatingColors {
    let (offset_r, offset_g, offset_b) = (offsets.0, offsets.1, offsets.2);
    grpc::GlobalFindColorRequest::new(hex_colors, offset_r, offset_g, offset_b)
        .send()
        .python_response()
}

#[pyfunction]
pub fn find_text(start: (u32, u32), end: (u32, u32), langs: grpc::Langs) -> String {
    let (start_x, start_y, end_x, end_y) = (start.0, start.1, end.0, end.1);
    grpc::FindTextRequest::new(langs, start_x, start_y, end_x, end_y)
        .send()
        .python_response()
}
