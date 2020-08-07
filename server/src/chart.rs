use base64;

use chrono::{NaiveDateTime, FixedOffset, DateTime};

fn parse_time(timestamp: i64) -> chrono::DateTime<FixedOffset> {
    let naive_dt = NaiveDateTime::from_timestamp(timestamp, 0);
    chrono::DateTime::from_utc(naive_dt, chrono::FixedOffset::east(60 * 60 * 3))
}

pub fn make_chart_encoded_base64(
    title: String,
    x: std::vec::Vec<i64>,
    y: std::vec::Vec<f64>
) -> String {
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
                 parse_time(x.first().unwrap().clone()) .. parse_time(x.last().unwrap().clone()),
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
                x.clone().into_iter().map(|a| parse_time(a)).zip(y.clone().into_iter().map(|a| a as f32)),
                 ShapeStyle {color: BLUE.to_rgba(), filled: true, stroke_width: 1}
            ))
            .unwrap();
    }
    let img = image::RgbImage::from_raw(width, height, buffer).unwrap();
    let img = image::DynamicImage::ImageRgb8(img);
    let mut output = vec![];
    img.write_to(&mut output, image::ImageOutputFormat::Png).unwrap();
    img.save(format!("{}.png", &title)).unwrap();
    return base64::encode(output);
}
