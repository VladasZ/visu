mod normalized_data;

use plotters::{
    backend::BitMapBackend,
    coord::types::RangedCoordu128,
    drawing::IntoDrawingArea,
    prelude::{
        Cartesian2d, ChartBuilder, ChartContext, LabelAreaPosition, LineSeries, PathElement, RGBAColor,
        SeriesLabelPosition, BLACK, BLUE, MAGENTA, WHITE,
    },
    style::Color,
};

struct AccrualsData {
    day:    u128,
    steps:  u128,
    simple: u128,
}

#[test]
#[ignore]
fn plot() -> anyhow::Result<()> {
    // render_chart("Step jars interest", get_data(), "walk.png")?;

    Ok(())
}

fn render_chart(name: &str, data: Vec<AccrualsData>, output: &str) -> anyhow::Result<()> {
    let root = BitMapBackend::new(output, (2800, 1800)).into_drawing_area();

    root.fill(&WHITE)?;

    let min_x: u128 = data.iter().map(|data| data.day).min().unwrap();
    let max_x: u128 = data.iter().map(|data| data.day).max().unwrap();

    let min_y: u128 = data.iter().map(|data| data.steps).min().unwrap();
    let max_y: u128 = data.iter().map(|data| data.simple).max().unwrap(); // + NearToken::from_near(12).as_yoctonear();

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 150)
        .set_label_area_size(LabelAreaPosition::Bottom, 100)
        .margin(10)
        .caption(name, ("sans-serif", 60))
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart
        .configure_mesh()
        .y_label_style(("sans-serif", 40))
        .x_label_style(("sans-serif", 40))
        .x_desc("Days")
        .y_desc("$SWEAT / APY %")
        .y_label_formatter(&|value| {
            format!(
                "{:.1}",
                *value as f64 /* NearToken::from_near(1).as_yoctonear() as f64 */
            )
        })
        .draw()?;

    // draw_graph(
    //     &mut chart,
    //     data.iter().map(|data| {
    //         (
    //             data.day,
    //             (data.apy * 10000.0) as u128 *
    // NearToken::from_near(1).as_yoctonear() / 100,         )
    //     }),
    //     "APY",
    //     GREEN,
    // )?;

    draw_graph(
        &mut chart,
        data.iter().map(|data| (data.day, data.steps)),
        "1% - 1000 steps",
        BLUE,
    )?;

    draw_graph(
        &mut chart,
        data.iter().map(|data| (data.day, data.simple)),
        "12% jar",
        MAGENTA.mix(0.5),
    )?;

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .legend_area_size(140)
        .label_font(("sans-serif", 60))
        .border_style(BLACK.stroke_width(4))
        .draw()?;

    root.present().expect("Unable to write result to file");
    Ok(())
}

fn draw_graph(
    chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordu128, RangedCoordu128>>,
    data: impl IntoIterator<Item = (u128, u128)>,
    label: &str,
    color: impl Into<RGBAColor>,
) -> anyhow::Result<()> {
    let color: RGBAColor = color.into();
    let series = LineSeries::new(data, color.stroke_width(8));
    chart
        .draw_series(series)?
        .label(label)
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 100, y)], color.stroke_width(8)));

    Ok(())
}
