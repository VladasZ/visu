use anyhow::Result;
use plotters::{
    backend::BitMapBackend,
    chart::{ChartBuilder, ChartContext, LabelAreaPosition, SeriesLabelPosition},
    coord::types::RangedCoordu128,
    drawing::IntoDrawingArea,
    element::PathElement,
    prelude::{
        Cartesian2d, LineSeries, RGBAColor, RGBColor, BLACK, BLUE, CYAN, GREEN, MAGENTA, WHITE, YELLOW,
    },
    style::Color,
};

use crate::normalized_data::normalize_data;

const COLORS: [RGBColor; 5] = [GREEN, BLUE, MAGENTA, CYAN, YELLOW];

pub struct Graph<'a, const SIZE: usize> {
    pub title:       &'a str,
    pub x_title:     &'a str,
    pub y_title:     &'a str,
    pub data:        [&'a [u128]; SIZE],
    pub legend:      [&'a str; SIZE],
    pub output_file: &'a str,
}

impl<'a, const SIZE: usize> Default for Graph<'a, SIZE> {
    fn default() -> Self {
        Self {
            title:       "",
            x_title:     "",
            y_title:     "",
            data:        [&[]; SIZE],
            legend:      [""; SIZE],
            output_file: "",
        }
    }
}

pub fn render_chart<const SIZE: usize>(graph: Graph<SIZE>) -> Result<()> {
    let root = BitMapBackend::new(graph.output_file, (2800, 1800)).into_drawing_area();

    root.fill(&WHITE)?;

    let data = normalize_data(graph.data);

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 100)
        .set_label_area_size(LabelAreaPosition::Bottom, 100)
        .margin(10)
        .caption(graph.title, ("sans-serif", 60))
        .build_cartesian_2d(data.x_range, data.y_range)?;

    #[allow(clippy::cast_precision_loss)]
    chart
        .configure_mesh()
        .y_label_style(("sans-serif", 40))
        .x_label_style(("sans-serif", 40))
        .x_desc(graph.x_title)
        .y_desc(graph.y_title)
        .y_label_formatter(&|value| {
            format!(
                "{:.1}",
                if data.y_divider > 0 {
                    value / data.y_divider
                } else {
                    *value
                }
            )
        })
        .draw()?;

    for (i, data) in data.data.iter().enumerate() {
        draw_graph(
            &mut chart,
            data.iter().enumerate().map(|(pos, val)| (pos as u128, *val)),
            graph.legend[i],
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
) -> Result<()> {
    let color: RGBAColor = color.into();
    let series = LineSeries::new(data, color.stroke_width(8));
    chart
        .draw_series(series)?
        .label(label)
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 100, y)], color.stroke_width(8)));

    Ok(())
}
