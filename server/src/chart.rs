use base64;

pub fn make_chart_encoded_base64(
    title: String,
    x: std::vec::Vec<u32>,
    y: std::vec::Vec<f64>
) -> String {
    use plotters::prelude::*;
    let first = x.first().unwrap();
    let x : std::vec::Vec<f32> = x.iter().map(|it| (it - first) as f32).collect();

    let width = 1000;
    let height = 800;

    let mut buffer: std::vec::Vec<u8> = vec![0; (width * height * 3) as usize];
    {
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
        root.fill(&WHITE).expect("Filled white");

        let mut sorted_y: std::vec::Vec<f32> = y.clone().into_iter().map(|x| x as f32).collect();
        sorted_y.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut chart1 = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_ranged(
                x.first().unwrap().clone() as f32 .. x.last().unwrap().clone() as f32,
                sorted_y.first().unwrap().clone() as f32..sorted_y.last().unwrap().clone() as f32
            )
            .expect("NO");

        chart1.configure_mesh().draw().expect("Drawing mesh");

        chart1
            .draw_series(LineSeries::new(
                x.clone().into_iter().map(|a| a as f32).zip(y.clone().into_iter().map(|a| a as f32)),
                &RED,
            )).expect("No3")
            .label("real graph");
    }
    let img = image::RgbImage::from_raw(width, height, buffer).unwrap();
    let img = image::DynamicImage::ImageRgb8(img);
    let mut output = vec![];
    img.write_to(&mut output, image::ImageOutputFormat::Png).unwrap();

    return base64::encode(output);
}
