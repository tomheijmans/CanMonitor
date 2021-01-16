use crate::models::CanLine;
use crate::models::ChartModel;

pub struct ViewState {
    pub chart_model: ChartModel,
    pub id_to_monitor: u32,
    pub byte_index_start: u8,
    pub byte_index_end: u8,
    can_lines: Vec<CanLine>,
}

impl ViewState {
    pub fn new(
        id_to_monitor: u32,
        byte_index_start: u8,
        byte_index_end: u8,
        chart_model: ChartModel,
    ) -> ViewState {
        ViewState {
            id_to_monitor: id_to_monitor,
            byte_index_start: byte_index_start,
            byte_index_end: byte_index_end,
            chart_model: chart_model,
            can_lines: Vec::new(),
        }
    }

    pub fn process_new_value(&mut self, can_line: CanLine) {
        if can_line.id == self.id_to_monitor {
            let value = can_line.get_value(self.byte_index_start, self.byte_index_end);
            self.chart_model.add_value(value);
            self.can_lines.push(can_line);
        }
    }

    pub fn get_last_can_lines(&self, count: usize) -> impl Iterator<Item = &CanLine> {
        self.can_lines.iter().rev().take(count)
    }
}
