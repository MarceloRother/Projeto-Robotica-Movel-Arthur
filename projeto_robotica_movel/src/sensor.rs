use rand::Rng;

pub struct Sensor {
    pitch: f64,
    roll: f64,
}

impl Sensor {
    pub fn new() -> Self {
        Sensor { pitch: 0.0, roll: 0.0 }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        self.pitch += rng.gen_range(-1.0..1.0);
        self.roll += rng.gen_range(-1.0..1.0);
    }

    pub fn get_pitch(&self) -> f64 {
        self.pitch
    }

    pub fn get_roll(&self) -> f64 {
        self.roll
    }
}
