use std::cmp;

pub struct ChartModel {
    values_on_x: usize,
    next_x: i64,
    pub x_window: [f64; 2],
    pub y_window: [f64; 2],
    pub values: Vec<(f64, f64)>,
}

impl ChartModel {
    pub fn new(values_on_x: usize) -> ChartModel {
        ChartModel {
            values_on_x: values_on_x,
            next_x: 1,
            x_window: [0.0, values_on_x as f64],
            y_window: [0.0, 100.0],
            values: vec![],
        }
    }

    pub fn add_value(&mut self, value: f64) {
        if self.values.len() == self.values_on_x {
            self.values.remove(0);
        }
        self.values.push((self.next_x as f64, value));
        self.next_x += 1;

        if value + 100.0 > self.y_window[1] {
            self.y_window = [0.0, value + 100.0];
        }

        self.x_window = [
            cmp::max(0, self.next_x - self.values_on_x as i64) as f64,
            cmp::max(self.next_x, self.values_on_x as i64) as f64,
        ];
    }

    pub fn get_last_value(&self) -> u64 {
        match self.values.last() {
            Some(value) => value.1 as u64,
            _ => 0,
        }
    }
}
