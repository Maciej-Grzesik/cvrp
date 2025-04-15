fn plot_best_path(best_path: &Vec<Node>, name: &str) {
    let root = BitMapBackend::new(name, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Best Path", ("sans-serif", 50))
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let points: Vec<(i32, i32)> = best_path.iter().map(|node| (node.x, node.y)).collect();
    chart
        .draw_series(points.iter().map(|&(x, y)| {
            Circle::new(
                (x, y),
                5,
                ShapeStyle {
                    color: BLUE.to_rgba(),
                    filled: true,
                    stroke_width: 1,
                },
            )
        }))
        .unwrap();

    let mut prev_node: Option<&Node> = None;
    let line_colors = [&RED, &MAGENTA, &GREEN, &BLUE, &BROWN, &YELLOW, &PINK];
    let mut color_idx = 0;
    for node in best_path {
        if let Some(prev) = prev_node {
            chart
                .draw_series(LineSeries::new(
                    vec![(prev.x, prev.y), (node.x, node.y)],
                    line_colors[color_idx].to_owned(),
                ))
                .unwrap();
        }
        if node.id == 1 {
            color_idx += 1;
        }
        prev_node = Some(node);
    }

    root.present().unwrap();
}

