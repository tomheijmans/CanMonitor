use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::{Span},
    symbols,
    widgets::{
        Axis, Block, Borders, Chart, Dataset
    },
    Frame,
};

use crate::models::ChartModel;

pub fn draw_charts<'a, B>(f: &mut Frame<B>, chart_model: &ChartModel)
where
    B: Backend,
{   
    let datasets = vec![
        Dataset::default()
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&chart_model.values)
    ];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Chart",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds(chart_model.x_window),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds(chart_model.y_window)
                .labels(vec![
                    Span::styled(format!("{}", chart_model.y_window[0]), Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{}",(chart_model.y_window[0] + chart_model.y_window[1]) / 2.0)),
                    Span::styled(format!("{}", chart_model.y_window[1]), Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    f.render_widget(chart, f.size());
}