use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    Frame,
};

use crate::models::ChartModel;

pub fn draw<'a, B>(f: &mut Frame<B>, chart_model: &ChartModel)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .direction(Direction::Horizontal)
        .split(f.size());

    draw_value(f, chunks[0], chart_model);
    draw_charts(f, chunks[1], chart_model);
}

fn draw_value<'a, B>(f: &mut Frame<B>, area: Rect, chart_model: &ChartModel)
where
    B: Backend,
{
    let value = &chart_model.get_last_value();
    let text = vec![Spans::from(vec![Span::styled(
        value.to_string(),
        Style::default().add_modifier(Modifier::REVERSED),
    )])];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Values",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block); 
    f.render_widget(paragraph, area);
}

fn draw_charts<'a, B>(f: &mut Frame<B>, area: Rect, chart_model: &ChartModel)
where
    B: Backend,
{
    let datasets = vec![Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&chart_model.values)];
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
                    Span::styled(
                        format!("{}", chart_model.y_window[0]),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!(
                        "{}",
                        (chart_model.y_window[0] + chart_model.y_window[1]) / 2.0
                    )),
                    Span::styled(
                        format!("{}", chart_model.y_window[1]),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                ]),
        );
    f.render_widget(chart, area);
}
