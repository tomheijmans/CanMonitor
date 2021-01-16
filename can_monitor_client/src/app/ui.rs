use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    Frame,
};

use crate::models::{CanLine, ChartModel, ViewState};

pub fn draw<B>(f: &mut Frame<B>, view_state: &ViewState)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .direction(Direction::Horizontal)
        .split(f.size());

    let right_chunks = Layout::default()
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .direction(Direction::Vertical)
        .split(chunks[1]);

    draw_value(f, chunks[0], &view_state.chart_model);
    draw_header(f, right_chunks[0], &view_state);
    draw_charts(f, right_chunks[1], &view_state.chart_model);
}

fn draw_header<B>(f: &mut Frame<B>, area: Rect, view_state: &ViewState)
where
    B: Backend,
{
    let mut text = Vec::new();
    for can_line in view_state.get_last_can_lines(5) {
        text.push(Spans::from(vec![
            span_for_byte(
                can_line,
                0,
                view_state.byte_index_start,
                view_state.byte_index_end,
            ),
            span_for_byte(
                can_line,
                1,
                view_state.byte_index_start,
                view_state.byte_index_end,
            ),
            span_for_byte(
                can_line,
                2,
                view_state.byte_index_start,
                view_state.byte_index_end,
            ),
            span_for_byte(
                can_line,
                3,
                view_state.byte_index_start,
                view_state.byte_index_end,
            ),
            span_for_byte(
                can_line,
                4,
                view_state.byte_index_start,
                view_state.byte_index_end,
            ),
            span_for_byte(
                can_line,
                5,
                view_state.byte_index_start,
                view_state.byte_index_end,
            ),
            span_for_byte(
                can_line,
                6,
                view_state.byte_index_start,
                view_state.byte_index_end,
            ),
            span_for_byte(
                can_line,
                7,
                view_state.byte_index_start,
                view_state.byte_index_end,
            ),
        ]));
    }

    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Values",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn span_for_byte<'a>(
    can_line: &CanLine,
    index: u8,
    mark_start_index: u8,
    mark_end_index: u8,
) -> Span {
    let mut style: Style = Style::default();
    if index >= mark_start_index && index <= mark_end_index {
        style = Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD);
    }

    Span::styled(format!("{:08b} ", can_line.get_byte(index as usize)), style)
}

fn draw_value<B>(f: &mut Frame<B>, area: Rect, chart_model: &ChartModel)
where
    B: Backend,
{
    let value = &chart_model.get_last_value();
    let text = vec![Spans::from(vec![Span::styled(
        value.to_string(),
        Style::default(),
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

fn draw_charts<B>(f: &mut Frame<B>, area: Rect, chart_model: &ChartModel)
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
