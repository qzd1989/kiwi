use super::Frame;
use crate::extensions::ImageBufferRgbaExt as _;
use crate::types::Point;
use crate::types::Size;
use crate::types::WeightPoint;
use anyhow::{anyhow, Result};
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
impl Frame {
    pub fn find_image(
        &self,
        template: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        point: Point,
        size: Size,
        threshold: impl Into<f64>, //建议0.99以上
    ) -> Result<Option<WeightPoint>> {
        let (template_width, template_height) = {
            let (w, h) = template.dimensions();
            (w as usize, h as usize)
        };
        if size.width < template_width as u32 || size.height < template_height as u32 {
            return Err(anyhow!("invalid area"));
        }
        let image = self.to_buffer()?.crop(point, size).to_mat()?;
        let mask = template.mask()?;
        let template = template.to_mat()?;
        let mut matched = Mat::default();
        imgproc::match_template(
            &image,
            &template,
            &mut matched,
            TemplateMatchModes::TM_CCORR_NORMED.into(),
            &mask,
        )?;
        let mut one = FindResult::new(template, matched).one()?;
        if one.weight < threshold.into() {
            println!("threshold is too big, the max is {:?}", one.weight);
            return Ok(None);
        }
        one.point.x += point.x;
        one.point.y += point.y;
        Ok(Some(one))
    }
    pub fn find_images(
        &self,
        template: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        point: Point,
        size: Size,
        threshold: impl Into<f64>, //建议0.99以上
    ) -> Result<Vec<WeightPoint>> {
        let (template_width, template_height) = {
            let (w, h) = template.dimensions();
            (w as usize, h as usize)
        };
        if size.width < template_width as u32 || size.height < template_height as u32 {
            return Err(anyhow!("invalid area"));
        }
        let image = self.to_buffer()?.crop(point, size).to_mat()?;
        let mask = template.mask()?;
        let template = template.to_mat()?;
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
        for weight_point in weight_points.iter_mut() {
            weight_point.point.x += point.x;
            weight_point.point.y += point.y;
        }
        Ok(weight_points)
    }
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
            data: HashMap<(i32, i32), Vec<Value>>,
        }

        impl Buckets {
            // 创建一个新的桶
            fn new() -> Self {
                Self {
                    data: HashMap::new(),
                }
            }

            // 向桶中添加一个值
            fn add(&mut self, key: (i32, i32), element: Value) {
                self.data.entry(key).or_insert_with(Vec::new).push(element);
            }

            // 获取桶中所有的值
            // fn all(&mut self) -> Vec<Value> {
            //     let mut points = Vec::new();
            //     for el in &mut self.data {
            //         let bucket = el.1;
            //         bucket.truncate(1);
            //         points.push(bucket.pop().unwrap());
            //     }
            //     points
            // }
            fn all(&self) -> Vec<Value> {
                self.data
                    .values()
                    .filter_map(|bucket| bucket.first().cloned())
                    .collect()
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
            println!("threshold is too big, the max is {:?}", _weight_max);
            return Ok(Vec::new());
        }

        // 创建一个新的桶
        let mut buckets = Buckets::new();

        // 将匹配添加到桶中
        for element in matches.iter() {
            let key = (element.point.x / cols, element.point.y / rows);
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
