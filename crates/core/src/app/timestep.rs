use std::time::Instant;

#[derive(Debug)]
pub struct TimeStep {
    last_time: Instant,
    delta_time: f32,
    frame_count: u32,
    frame_time: f32,
}

impl TimeStep {
    pub fn new() -> TimeStep {
        TimeStep {
            last_time: Instant::now(),
            delta_time: 0.0,
            frame_count: 0,
            frame_time: 0.0,
        }
    }

    pub fn delta(&mut self) -> f32 {
        let current_time = Instant::now();
        let delta = current_time.duration_since(self.last_time).as_micros() as f32 * 0.001;
        self.last_time = current_time;
        self.delta_time = delta;
        delta
    }

    // provides the framerate in FPS
    pub fn frame_rate(&mut self) -> Option<u32> {
        self.frame_count += 1;
        self.frame_time += self.delta_time;
        let tmp;
        // per second
        if self.frame_time >= 1000.0 {
            tmp = self.frame_count;
            self.frame_count = 0;
            self.frame_time = 0.0;
            return Some(tmp);
        }
        None
    }
}

/*
let mut timestep = TimeStep::new();
let mut lag = 0.0;

'running: loop {
    if !game.running() {
        break 'running;
    }
    game.handle_events();

    lag += timestep.delta();

    while lag >= MS_PER_UPDATE {
        game.update(MS_PER_UPDATE * 0.01);
        lag -= MS_PER_UPDATE;
    }

    game.render(lag);
}
*/
