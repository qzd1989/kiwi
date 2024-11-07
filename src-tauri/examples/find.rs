use kiwi_lib::capture::Frame;
use kiwi_lib::common::Point;
use kiwi_lib::find::common::crop_rgba;
use kiwi_lib::find::common::frame_to_rgba;
use kiwi_lib::find::common::rgba_to_mat;
use kiwi_lib::find::image::find_one;
use opencv::core::Point as CvPoint;
use opencv::core::Scalar;
use opencv::highgui::imshow;
use opencv::highgui::wait_key;
use opencv::imgproc::put_text_def;
use opencv::imgproc::rectangle_points_def;
pub fn main() {
    let origin = image::open("origin.png").unwrap().to_rgba8();
    let origin = Frame::new(origin.width(), origin.height(), origin.to_vec());
    let template: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::open("template2.png").unwrap().to_rgba8();
    let start_at = Point::new(0, 0);
    // let end_at = Point::new(origin.width, origin.height);
    let end_at = Point::new(1500, 1500);
    dbg!((origin.width, origin.height));
    dbg!((template.width(), template.height()));
    let result = find_one(origin.clone(), template.clone(), start_at, end_at, 0.99).unwrap();
    dbg!(result);
    //draw
    let origin = frame_to_rgba(origin).unwrap();
    let origin = crop_rgba(
        &origin,
        start_at.x as u32,
        start_at.y as u32,
        (end_at.x - start_at.x) as u32,
        (end_at.y - start_at.y) as u32,
    );
    let mut mat = rgba_to_mat(origin).unwrap();
    let pt1 = CvPoint::new(result.point.x as i32, result.point.y as i32);
    let pt2 = CvPoint::new(
        (template.width() + result.point.x as u32) as i32,
        (template.height() + result.point.y as u32) as i32,
    );
    rectangle_points_def(&mut mat, pt1, pt2, Scalar::new(0.0, 0.0, 255.0, 255.0)).unwrap();
    put_text_def(
        &mut mat,
        result.weight.to_string().as_str(),
        pt1,
        0,
        2.0,
        Scalar::new(0.0, 0.0, 255.0, 255.0),
    )
    .unwrap();
    imshow("undefined", &mat).unwrap();
    wait_key(100000).unwrap();
}
