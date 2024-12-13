use crate::grpc;
use pyo3::prelude::*;

#[pyfunction]
pub fn find_image(
    path: String,
    start: (f64, f64),
    end: (f64, f64),
    threshold: f64,
) -> Option<grpc::response::WeightPoint> {
    let (start_x, start_y, end_x, end_y) = (start.0, start.1, end.0, end.1);
    grpc::FindImageRequest::new(
        path,
        start_x as u32,
        start_y as u32,
        end_x as u32,
        end_y as u32,
        threshold,
    )
    .send()
    .python_response()
}

#[pyfunction]
pub fn find_images(
    path: String,
    start: (f64, f64),
    end: (f64, f64),
    threshold: f64,
) -> grpc::response::WeightPoints {
    let (start_x, start_y, end_x, end_y) = (start.0, start.1, end.0, end.1);
    grpc::FindImagesRequest::new(
        path,
        start_x as u32,
        start_y as u32,
        end_x as u32,
        end_y as u32,
        threshold,
    )
    .send()
    .python_response()
}

#[pyfunction]
pub fn global_find_image(path: String, threshold: f64) -> Option<grpc::response::WeightPoint> {
    grpc::GlobalFindImageRequest::new(path, threshold)
        .send()
        .python_response()
}

#[pyfunction]
pub fn global_find_images(path: String, threshold: f64) -> grpc::response::WeightPoints {
    grpc::GlobalFindImagesRequest::new(path, threshold)
        .send()
        .python_response()
}

#[pyfunction]
pub fn find_locating_color(
    locating_colors: grpc::request::LocatingColors,
    start: (f64, f64),
    end: (f64, f64),
    offsets: (f64, f64, f64),
) -> Option<grpc::response::LocatingColor> {
    let (start_x, start_y, end_x, end_y) = (start.0, start.1, end.0, end.1);
    let (offset_r, offset_g, offset_b) = (offsets.0, offsets.1, offsets.2);
    grpc::FindLocatingColorRequest::new(
        locating_colors,
        start_x as u32,
        start_y as u32,
        end_x as u32,
        end_y as u32,
        offset_r as u32,
        offset_g as u32,
        offset_b as u32,
    )
    .send()
    .python_response()
}

#[pyfunction]
pub fn global_find_locating_color(
    locating_colors: grpc::request::LocatingColors,
    offsets: (f64, f64, f64),
) -> Option<grpc::response::LocatingColor> {
    let (offset_r, offset_g, offset_b) = (offsets.0, offsets.1, offsets.2);
    grpc::GlobalFindLocatingColorRequest::new(
        locating_colors,
        offset_r as u32,
        offset_g as u32,
        offset_b as u32,
    )
    .send()
    .python_response()
}

#[pyfunction]
pub fn find_color(
    hex_colors: grpc::request::HexColors,
    start: (f64, f64),
    end: (f64, f64),
    offsets: (f64, f64, f64),
) -> grpc::response::LocatingColors {
    let (start_x, start_y, end_x, end_y) = (start.0, start.1, end.0, end.1);
    let (offset_r, offset_g, offset_b) = (offsets.0, offsets.1, offsets.2);
    grpc::FindColorRequest::new(
        hex_colors,
        start_x as u32,
        start_y as u32,
        end_x as u32,
        end_y as u32,
        offset_r as u32,
        offset_g as u32,
        offset_b as u32,
    )
    .send()
    .python_response()
}

#[pyfunction]
pub fn global_find_color(
    hex_colors: grpc::request::HexColors,
    offsets: (f64, f64, f64),
) -> grpc::response::LocatingColors {
    let (offset_r, offset_g, offset_b) = (offsets.0, offsets.1, offsets.2);
    grpc::GlobalFindColorRequest::new(
        hex_colors,
        offset_r as u32,
        offset_g as u32,
        offset_b as u32,
    )
    .send()
    .python_response()
}

#[pyfunction]
pub fn find_text(start: (f64, f64), end: (f64, f64), langs: grpc::response::Langs) -> String {
    let (start_x, start_y, end_x, end_y) = (start.0, start.1, end.0, end.1);
    grpc::FindTextRequest::new(
        langs,
        start_x as u32,
        start_y as u32,
        end_x as u32,
        end_y as u32,
    )
    .send()
    .python_response()
}
