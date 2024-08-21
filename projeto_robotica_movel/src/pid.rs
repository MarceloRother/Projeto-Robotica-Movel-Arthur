// src/pid.rs
pub struct PID {
    kp: f64, // Ganho Proporcional
    ki: f64, // Ganho Integral
    kd: f64, // Ganho Derivativo
    previous_error: f64,
    integral: f64,
}

impl PID {
}
