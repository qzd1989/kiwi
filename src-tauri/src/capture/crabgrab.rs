use super::Frame;
use crabgrab::prelude::*;
use crossbeam_channel::bounded;
use std::sync::{Arc, Mutex};

pub async fn get_primary_display() -> CapturableDisplay {
    let filter = CapturableContentFilter::DISPLAYS;
    let content = CapturableContent::new(filter).await.unwrap();
    content.displays().next().unwrap()
}

pub async fn frame<F>(display: CapturableDisplay, frame_callback: F)
where
    F: Fn(Frame),
{
    let (sender, receiver) = bounded(100);
    let sender = Arc::new(Mutex::new(sender));
    let token = match CaptureStream::test_access(false) {
        Some(token) => token,
        None => CaptureStream::request_access(false).await.unwrap(),
    };
    let config = CaptureConfig::with_display(display, CapturePixelFormat::Bgra8888);
    let callback = move |stream_event: Result<StreamEvent, StreamError>| {
        if let Ok(StreamEvent::Video(frame)) = stream_event {
            if let Ok(bitmap) = frame.get_bitmap() {
                if let FrameBitmap::BgraUnorm8x4(data) = bitmap {
                    let (width, height) = (data.width as u32, data.height as u32);
                    let bgra_data = data.data;
                    let mut rgba_buffer = image::ImageBuffer::new(width, height);
                    for (x, y, pixel) in rgba_buffer.enumerate_pixels_mut() {
                        let index = (y * (width) + x) as usize;
                        let [b, g, r, a] = bgra_data[index];
                        *pixel = image::Rgba([r, g, b, a]);
                    }
                    let frame = Frame::new(width, height, rgba_buffer.to_vec());
                    sender.lock().unwrap().send(frame).unwrap();
                }
            }
        }
    };
    let mut _stream = CaptureStream::new(token, config, callback).unwrap();
    loop {
        if let Ok(data) = receiver.try_recv() {
            frame_callback(data);
        }
    }
}

pub async fn snapshot(display: CapturableDisplay) -> Frame {
    let (sender, receiver) = bounded(100);
    let sender = Arc::new(Mutex::new(sender));
    let token = match CaptureStream::test_access(false) {
        Some(token) => token,
        None => CaptureStream::request_access(false).await.unwrap(),
    };
    let config = CaptureConfig::with_display(display, CapturePixelFormat::Bgra8888);
    let callback = move |stream_event: Result<StreamEvent, StreamError>| {
        if let Ok(StreamEvent::Video(frame)) = stream_event {
            if let Ok(bitmap) = frame.get_bitmap() {
                if let FrameBitmap::BgraUnorm8x4(data) = bitmap {
                    let (width, height) = (data.width as u32, data.height as u32);
                    let bgra_data = data.data;
                    let mut rgba_buffer = image::ImageBuffer::new(width, height);
                    for (x, y, pixel) in rgba_buffer.enumerate_pixels_mut() {
                        let index = (y * (width) + x) as usize;
                        let [b, g, r, a] = bgra_data[index];
                        *pixel = image::Rgba([r, g, b, a]);
                    }
                    let frame = Frame::new(width, height, rgba_buffer.to_vec());
                    if let Ok(_) = sender.lock().unwrap().send(frame) {}
                }
            }
        }
    };
    let mut _stream = CaptureStream::new(token, config, callback).unwrap();
    if let Ok(frame) = receiver.recv() {
        frame
    } else {
        panic!("No frame received")
    }
}
