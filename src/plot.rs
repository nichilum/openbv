use plotly::{common::Title, layout::Axis, Bar, ImageFormat, Layout, Plot};

pub fn plot_histogram<T>(histogram: &[T], filename: &str, y_title: &str)
where
    T: Clone + serde::ser::Serialize + 'static,
{
    let mut plot = Plot::new();
    plot.set_layout(
        Layout::new()
            // .title(Title::new("Histogram"))
            .x_axis(Axis::new().title(Title::new("Value")))
            .y_axis(Axis::new().title(Title::new(y_title))),
    );

    let x = (0..256).collect::<Vec<_>>();
    let bar = Bar::new(x, histogram.to_vec());

    plot.add_trace(bar);
    plot.write_image(
        filename,
        ImageFormat::SVG,
        1024,
        680,
        1.0,
    );
}
