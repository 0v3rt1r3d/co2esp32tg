use chrono::{FixedOffset, DateTime};

use super::utils;


pub fn save_chart(
    title: &'static str,
    filename: &'static str,
    x: &std::vec::Vec<i64>,
    y: &std::vec::Vec<f64>
) {
    use plotters::prelude::*;

    let width = 1400;
    let height = 800;

    let mut buffer: std::vec::Vec<u8> = vec![0; (width * height * 3) as usize];
    {
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut sorted_y: std::vec::Vec<f32> = y.clone().into_iter().map(|x| x as f32).collect();
        sorted_y.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut chart = ChartBuilder::on(&root)
            .caption(&title, ("sans-serif", 50).into_font())
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .margin_right(60)
            .build_ranged(
                 utils::parse_time(x.first().unwrap().clone())..utils::parse_time(x.last().unwrap().clone()),
                sorted_y.first().unwrap().clone()..sorted_y.last().unwrap().clone()
            )
            .unwrap();

        chart.configure_mesh()
            .x_labels(6)
            .x_label_style(("sans-serif", 18).into_font())
            .x_label_formatter(&|t: &DateTime<FixedOffset>| { t.format("%d %h %Y %H:%M").to_string()})
            .y_labels(10)
            .y_label_style(("sans-serif", 18).into_font())
            .draw()
            .unwrap();
        chart
            .draw_series(LineSeries::new(
                x.clone().into_iter().map(|t| utils::parse_time(t)).zip(y.clone().into_iter().map(|a| a as f32)),
                 ShapeStyle {color: BLUE.to_rgba(), filled: true, stroke_width: 1}
            ))
            .unwrap();
    }
    let img = image::RgbImage::from_raw(width, height, buffer).unwrap();
    let img = image::DynamicImage::ImageRgb8(img);
    // let mut output = vec![];
    // img.write_to(&mut output, image::ImageOutputFormat::Png).unwrap();
    // return base64::encode(output);
    img.save(&filename).unwrap();
}
