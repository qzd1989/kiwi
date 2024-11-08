use super::common::frame_to_rgba;
use crate::{
    capture::Frame,
    common::{Point, WeightPoint},
    find::common::{crop_rgba, mask, rgba_to_mat},
};
use anyhow::{anyhow, Ok, Result};
use image::ImageBuffer;
use opencv::core::min_max_loc;
use opencv::core::no_array;
use opencv::core::Point as OpencvCorePoint;
use opencv::core::Rect;
use opencv::core::Scalar;
use opencv::prelude::*;
use opencv::{
    core::Mat,
    imgproc::{self, TemplateMatchModes},
};
use std::collections::HashMap;

pub fn find_one(
    frame: Frame,
    template: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    x: impl Into<u32>,
    y: impl Into<u32>,
    width: impl Into<u32>,
    height: impl Into<u32>,
    threshold: impl Into<f64>, //建议0.99以上
) -> Result<WeightPoint> {
    let (width, height) = (width.into(), height.into());
    let (template_width, template_height) = template.dimensions();
    if width < template_width || height < template_height {
        return Err(anyhow!(format!(
            "Invalid search area:(x: {:?} - {:?}, y: {:?} - {:?})",
            width, template_width, height, template_height
        ))
        .into());
    }
    let buffer = frame_to_rgba(frame)?;
    let buffer = crop_rgba(&buffer, x, y, width, height);
    let image = rgba_to_mat(buffer)?;
    let mask = mask(template.clone())?;
    let template = rgba_to_mat(template)?;
    let mut matched = Mat::default();
    imgproc::match_template(
        &image,
        &template,
        &mut matched,
        TemplateMatchModes::TM_CCORR_NORMED.into(),
        &mask,
    )?;
    let one = FindResult::new(template, matched).one()?;
    if one.weight < threshold.into() {
        return Err(anyhow!(format!(
            "The lowest threshold is : {:?}",
            one.weight
        )));
    }
    Ok(one)
}

pub fn find_multiple(
    frame: Frame,
    template: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    x: impl Into<u32>,
    y: impl Into<u32>,
    width: impl Into<u32>,
    height: impl Into<u32>,
    threshold: impl Into<f64>, //建议0.99以上
) -> Result<Vec<WeightPoint>> {
    let (width, height) = (width.into(), height.into());
    let (template_width, template_height) = template.dimensions();
    if width < template_width || height < template_height {
        return Err(anyhow!(format!(
            "Invalid search area:(x: {:?} - {:?}, y: {:?} - {:?})",
            width, template_width, height, template_height
        ))
        .into());
    }
    let buffer = frame_to_rgba(frame)?;
    let buffer = crop_rgba(&buffer, x, y, width, height);
    let image = rgba_to_mat(buffer)?;
    let mask = mask(template.clone())?;
    let template = rgba_to_mat(template)?;
    let mut matched = Mat::default();
    imgproc::match_template(
        &image,
        &template,
        &mut matched,
        TemplateMatchModes::TM_CCORR_NORMED.into(),
        &mask,
    )?;
    let mut weight_points = FindResult::new(template, matched).multiple(threshold.into())?;
    weight_points.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());
    Ok(weight_points)
}

struct FindResult {
    pub template: Mat,
    pub matched: Mat,
}

impl FindResult {
    /// 创建一个新的ImageProcessor实例
    ///
    /// # 参数
    /// - `template`: 匹配模板，用于在输入图像中寻找匹配区域
    /// - `matched`: 用于展示匹配结果的图像
    ///
    /// # 返回
    /// 返回一个包含输入图像、匹配模板和匹配结果图像的ImageProcessor实例
    pub fn new(template: Mat, matched: Mat) -> Self {
        Self { template, matched }
    }
    /// 根据模板匹配结果，找出最佳匹配位置并返回一个位于图像中的矩形区域。
    ///
    /// # 返回
    /// - `Ok(WeightPoint)`：包含最佳匹配位置和大小的矩形区域。
    /// - `Err(OpencvError)`：操作失败时的错误信息。
    pub fn one(&mut self) -> Result<WeightPoint> {
        // 初始化最小值和最大值变量。
        let mut min_val = 0.0;
        let mut max_val = 0.0;
        // 初始化最小和最大位置变量，默认为零。
        let mut min_loc = OpencvCorePoint::default();
        let mut max_loc = OpencvCorePoint::default();

        // 调用min_max_loc函数，找出匹配结果中的最小值、最大值及其位置。
        min_max_loc(
            &self.matched,
            Some(&mut min_val),
            Some(&mut max_val),
            Some(&mut min_loc),
            Some(&mut max_loc),
            &no_array(),
        )?;

        // 最大值位置即为最佳匹配位置的左上角点。
        let top_left = max_loc;
        // 构造并返回WeightPoint对象，包含最佳匹配位置和大小，以及匹配值。
        Ok(WeightPoint::new(
            Point::new(top_left.x, top_left.y),
            max_val,
        ))
    }
    /// 处理多个模板匹配的函数
    ///
    /// # 参数
    /// - `threshold`: 匹配的阈值,通常设为0.9
    ///
    /// # 返回值
    /// - `Result<Vec<WeightPoint>, OpencvError>`: 返回一个结果，其中包含一个位于矩形的向量或OpenCV错误
    ///
    /// # 错误
    /// - 当最大权重小于阈值时，返回错误，提示用户应改变阈值
    pub fn multiple(&mut self, threshold: f64) -> Result<Vec<WeightPoint>> {
        // 定义一个值结构体，包含权重和点
        #[derive(Debug, Clone, Copy)]
        struct Value {
            weight: f64,            //权重
            point: OpencvCorePoint, //左上角坐标
        }

        // 定义一个桶结构体，包含一组值的哈希图
        #[derive(Debug)]
        struct Buckets {
            data: HashMap<String, Vec<Value>>,
        }

        impl Buckets {
            // 创建一个新的桶
            fn new() -> Self {
                Self {
                    data: HashMap::new(),
                }
            }

            // 向桶中添加一个值
            fn add(&mut self, key: String, element: Value) {
                self.data.entry(key).or_insert_with(Vec::new).push(element);
            }

            // 获取桶中所有的值
            fn all(&mut self) -> Vec<Value> {
                let mut points = Vec::new();
                for el in &mut self.data {
                    let bucket = el.1;
                    bucket.truncate(1);
                    points.push(bucket.pop().unwrap());
                }
                points
            }
        }

        // 获取模板的列和行
        let (cols, rows) = (self.template.cols(), self.template.rows());

        // 找到所有匹配位置
        let mut min_val = 0.0;
        let mut max_val = 0.0;
        let mut min_loc = OpencvCorePoint::default();
        let mut max_loc = OpencvCorePoint::default();
        let mut matches = Vec::new();
        let mut _weight_max = 0.0;

        loop {
            // 寻找最小和最大值及其位置
            min_max_loc(
                &self.matched,
                Some(&mut min_val),
                Some(&mut max_val),
                Some(&mut min_loc),
                Some(&mut max_loc),
                &no_array(),
            )?;

            // 如果最大值大于等于阈值，则将其添加到匹配列表中
            if max_val >= threshold {
                matches.push(Value {
                    weight: max_val,
                    point: max_loc,
                });

                // 将已匹配到的位置涂黑，避免重复匹配
                imgproc::rectangle(
                    &mut self.matched,
                    Rect::new(
                        max_loc.x,
                        max_loc.y,
                        self.template.cols(),
                        self.template.rows(),
                    ),
                    Scalar::all(0.0),
                    -1,
                    imgproc::LINE_8,
                    0,
                )?;
            } else {
                // 记录最大权重后结束循环
                _weight_max = max_val;
                break;
            }
        }

        // 如果匹配列表为空，则返回错误
        if matches.is_empty() {
            return Err(anyhow!(format!(
                "The max weight is {:?}, you should change the threshold",
                _weight_max
            )));
        }

        // 创建一个新的桶
        let mut buckets = Buckets::new();

        // 将匹配添加到桶中
        for element in matches.iter() {
            let key = format!(
                "{}_{}",
                (element.point.x / cols) as u32,
                (element.point.y / rows) as u32
            );
            buckets.add(key, element.clone());
        }

        // 获取桶中所有的值
        let values = buckets.all();

        // 创建一个位于矩形的向量
        let mut weight_points = Vec::new();
        for value in values {
            let weight_point =
                WeightPoint::new(Point::new(value.point.x, value.point.y), value.weight);
            weight_points.push(weight_point);
        }
        // 返回位于矩形的向量
        Ok(weight_points)
    }
}
