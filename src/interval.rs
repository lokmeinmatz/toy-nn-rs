struct Countdown {
    id: usize,
    start_time: f32,
    time_remaining: f32,
    is_interval: bool
}

pub struct Timer {
    countdowns: Vec<Countdown>
}

impl Timer {
    pub fn new() -> Timer {
        Timer { countdowns: Vec::new() }
    }

    pub fn update(&mut self, delta: f32) -> Vec<usize> {
        let mut res = Vec::new();

        for cd in &mut self.countdowns {
            cd.time_remaining -= delta;

            if cd.time_remaining <= 0.0 {
                res.push(cd.id);
                if cd.is_interval {
                    cd.time_remaining = cd.start_time + cd.time_remaining;
                }
            }
        }
        self.countdowns.drain_filter(|e| e.time_remaining <= 0.0);
        res
    }

    pub fn add(&mut self, id: usize, time_secs: f32, repeat: bool) {
        self.countdowns.push(Countdown{
            id,
            start_time: time_secs,
            time_remaining: time_secs,
            is_interval: repeat
        });
    }
}