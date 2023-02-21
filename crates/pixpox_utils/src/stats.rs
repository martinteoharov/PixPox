use std::{time::{self, Instant}, collections::HashMap, fmt::format};

pub struct Stats {
    last_update: time::Instant,
    fps: f32,
    acc: Vec<f32>,
    sectors: HashMap<&'static str, f32>
}

impl Stats {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            fps: 0.0,
            acc: Vec::with_capacity(100000),
            sectors: HashMap::new()
        }
    }

    pub fn new_tick(&mut self) {
        let curr_diff = Instant::now() - self.last_update;

        self.fps = 1.0 / curr_diff.as_secs_f32();
        self.acc.push(self.fps);
        self.last_update = Instant::now();
    }

    pub fn get_fps(&self) -> f32 {
        self.fps
    }

    pub fn get_fps_as_string(&self) -> String {
        self.fps.to_string()
    }

    pub fn get_average_fps(&self) -> f32 {
        self.acc.iter().sum::<f32>() as f32 / self.acc.len() as f32
    }

    pub fn get_mean_fps(&self) -> f32 {
        // self.acc.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        // self.acc[self.acc.len() / 2]
        0.0
    }

    pub fn update_sector(&mut self, label: &'static str, value: f32) {
        self.sectors.insert(label, value);
    }

    pub fn get_formatted_stats(&self) -> Vec<String> {
        let mut ret = Vec::new();

        ret.push("fps: ".to_owned() + &self.get_fps().to_string());
        ret.push("avg fps: ".to_owned() + &self.get_average_fps().to_string());
        ret.push("mean fps: ".to_owned() + &self.get_mean_fps().to_string());
    
        for (k, v) in self.sectors.iter() {
            let s = format!("{}: {}", k, v);
            ret.push(s);
        };

        ret
    }
}
