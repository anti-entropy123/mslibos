use std::{fs::File, io::Read};

use charming::{
    component::{Axis, Grid, Legend, Title},
    datatype::{Dataset, Transform},
    element::{AxisLabel, AxisType, Color, NameLocation, SplitArea, SplitLine},
    series::Boxplot,
    Chart, ImageRenderer,
};
use rand::Rng;

pub fn chart() -> Chart {
    let data0 = vec![{
        let mut file = File::open("./media_service_trace.log").expect("no tracing file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("read file failed");
        serde_json::from_str::<Vec<f64>>(&content).expect("wrong file content")
    }];
    let data1 = vec![{
        let mut file = File::open("./media_service_no_libos_trace.log").expect("no tracing file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("read file failed");
        serde_json::from_str::<Vec<f64>>(&content).expect("wrong file content")
    }];
    let data2 = vec![{
        let mut file =
            File::open("./media_service_no_libos_trace_docker.log").expect("no tracing file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("read file failed");
        serde_json::from_str::<Vec<f64>>(&content).expect("wrong file content")
    }];

    Chart::new()
        .title(Title::new().text("Media Service").left("center"))
        .background_color(Color::Value("white".to_string()))
        .dataset(
            Dataset::new()
                .source(data0)
                .source(data1)
                .source(data2)
                .transform(
                    Transform::new()
                        .from_dataset_index(0)
                        .transform(r#"{"type": "boxplot"}"#),
                )
                .transform(
                    Transform::new()
                        .from_dataset_index(1)
                        .transform(r#"{"type": "boxplot"}"#),
                )
                .transform(
                    Transform::new()
                        .from_dataset_index(2)
                        .transform(r#"{"type": "boxplot"}"#),
                ),
            // .transform(
            //     Transform::new()
            //         .from_dataset_index(2)
            //         .transform(r#"{"type": "boxplot"}"#),
            // ),
        )
        .legend(Legend::new().top("10%"))
        // .tooltip(
        //     Tooltip::new()
        //         .trigger(Trigger::Axis)
        //         .axis_pointer(AxisPointer::new().type_(AxisPointerType::Shadow)),
        // )
        .grid(
            Grid::new()
                .left("10%")
                .top("20%")
                .right("10%")
                .bottom("15%"),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(true)
                .name_gap(15)
                .name("media service")
                .name_location(NameLocation::Center)
                .axis_label(AxisLabel::new().show(false))
                .split_area(SplitArea::new().show(true))
                .split_line(SplitLine::new().show(false)),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Workflow Execute Time(ms)")
                .name_location(NameLocation::Center)
                .name_gap(25)
                .min(2)
                .max(4)
                .split_area(SplitArea::new().show(false)),
        )
        // .data_zoom(DataZoom::new().type_(DataZoomType::Inside).start(0).end(20))
        // .data_zoom(
        //     DataZoom::new()
        //         .type_(DataZoomType::Slider)
        //         .start(0)
        //         .end(20)
        //         .top("90%")
        //         .x_axis_index(0),
        // )
        .series(Boxplot::new().name("mslibos").dataset_index(3))
        .series(Boxplot::new().name("faaslane-rs").dataset_index(4))
        .series(Boxplot::new().name("faaslane-rs-docker").dataset_index(5))
}

fn _make_data() -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    let mut data = vec![];
    for _ in 0..18 {
        let mut data0 = vec![];
        for _ in 0..100 {
            data0.push(rng.gen::<f64>() * 200.0);
        }
        data.push(data0);
    }
    data
}

fn main() {
    let chart = chart();
    let mut renderer = ImageRenderer::new(500, 400);
    renderer
        .save(&chart, "./media_service.svg")
        .expect("save img failed");
}
