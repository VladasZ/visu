use plotters::{
    backend::BitMapBackend,
    chart::{ChartBuilder, ChartContext, LabelAreaPosition, SeriesLabelPosition},
    coord::types::RangedCoordu128,
    drawing::IntoDrawingArea,
    element::PathElement,
    prelude::{Cartesian2d, LineSeries, RGBAColor, RGBColor, BLACK, BLUE, GREEN, MAGENTA, WHITE},
    style::Color,
};

use crate::normalized_data::normalize_data;

const COLORS: [RGBColor; 3] = [GREEN, BLUE, MAGENTA];

pub fn render_chart<const SIZE: usize>(
    name: &str,
    data: [&[u128]; SIZE],
    output: &str,
) -> anyhow::Result<()> {
    let root = BitMapBackend::new(output, (2800, 1800)).into_drawing_area();

    root.fill(&WHITE)?;

    let data = normalize_data(data);

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 150)
        .set_label_area_size(LabelAreaPosition::Bottom, 100)
        .margin(10)
        .caption(name, ("sans-serif", 60))
        .build_cartesian_2d(data.x_range, data.y_range)?;

    #[allow(clippy::cast_precision_loss)]
    chart
        .configure_mesh()
        .y_label_style(("sans-serif", 40))
        .x_label_style(("sans-serif", 40))
        .x_desc("Days")
        .y_desc("$SWEAT / APY %")
        .y_label_formatter(&|value| format!("{:.1}", *value as f64))
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

    for (i, data) in data.data.iter().enumerate() {
        draw_graph(
            &mut chart,
            data.iter().enumerate().map(|(pos, val)| (pos as u128, *val)),
            &format!("{i}"),
            COLORS[i],
        )?;
    }

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
