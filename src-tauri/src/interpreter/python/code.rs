use crate::interpreter::Code;
use crate::types::{HexColor, Point, RelativePoint, RgbOffset};

pub struct PythonCode {}

impl Default for PythonCode {
    fn default() -> Self {
        Self {}
    }
}

impl Code for PythonCode {
    fn predefined(&self) -> String {
        format!(
            r#"from kiwi import ScreenClient, Point, RelativePoint, RgbOffset, System, Key
client = ScreenClient()
"#
        )
    }

    fn find_image(
        &self,
        subpath: &str,
        start_point: &Point,
        end_point: &Point,
        threshold: f64,
    ) -> String {
        let Point { x: sx, y: sy } = start_point;
        let Point { x: ex, y: ey } = end_point;
        format!(
            r#"client.find_image(
    subpath="{subpath}",
    start_point=Point(x={sx},y={sy}),
    end_point=Point(x={ex},y={ey}),
    threshold={threshold},
)"#
        )
    }

    fn find_images(
        &self,
        subpath: &str,
        start_point: &Point,
        end_point: &Point,
        threshold: f64,
    ) -> String {
        let Point { x: sx, y: sy } = start_point;
        let Point { x: ex, y: ey } = end_point;
        format!(
            r#"client.find_images(
    subpath="{subpath}",
    start_point=Point(x={sx},y={sy}),
    end_point=Point(x={ex},y={ey}),
    threshold={threshold},
)"#
        )
    }

    fn find_relative_colors(
        &self,
        vertex_hex: &str,
        relative_points: &[RelativePoint],
        start_point: &Point,
        end_point: &Point,
        rgb_offset: &RgbOffset,
    ) -> String {
        let Point { x: sx, y: sy } = start_point;
        let Point { x: ex, y: ey } = end_point;
        let RgbOffset { r, g, b } = rgb_offset;
        let rows_str = relative_points
            .iter()
            .map(|p| {
                let Point { x, y } = p.point;
                format!("        RelativePoint(point=Point(x={x}, y={y})),")
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            r#"client.find_relative_colors(
    vertex_color="{vertex_hex}",
    relative_points=[
{rows_str}
    ],
    start_point=Point(x={sx},y={sy}),
    end_point=Point(x={ex},y={ey}),
    rgb_offset=RgbOffset(r={r}, g={g}, b={b}),
)"#
        )
    }

    fn find_colors(
        &self,
        hex_colors: &[HexColor],
        start_point: &Point,
        end_point: &Point,
        rgb_offset: &RgbOffset,
    ) -> String {
        let Point { x: sx, y: sy } = start_point;
        let Point { x: ex, y: ey } = end_point;
        let RgbOffset { r, g, b } = rgb_offset;
        let rows_str = hex_colors
            .iter()
            .map(|hex| format!("        \"{hex}\","))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            r#"client.find_colors(
    hex_colors=[
{rows_str}
    ],
    start_point=Point(x={sx},y={sy}),
    end_point=Point(x={ex},y={ey}),
    rgb_offset=RgbOffset(r={r}, g={g}, b={b}),
)"#
        )
    }

    fn recognize_text(&self, start_point: &Point, end_point: &Point) -> String {
        let Point { x: sx, y: sy } = start_point;
        let Point { x: ex, y: ey } = end_point;
        format!(
            r#"client.recognize_text(
    start_point=Point(x={sx},y={sy}),
    end_point=Point(x={ex},y={ey}),
)"#
        )
    }
}
