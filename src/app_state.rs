use druid::{Data, Lens, Color};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
pub struct Particle {
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub size: f64,
    pub rotation: f64,
    pub color: Color,
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub particles: Arc<Vec<Particle>>,
    pub initial_size: f64,
    pub shrink_speed: f64,
    pub direction: f64,
    pub speed: f64,
    pub gravity: f64,
    pub rotation_speed: f64,
    pub start_color_r: f64,
    pub start_color_g: f64,
    pub start_color_b: f64,
    pub end_color_r: f64,
    pub end_color_g: f64,
    pub end_color_b: f64,
    pub random_deviation: f64,
    pub is_spawning_paused: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            particles: Arc::new(Vec::new()),
            initial_size: 10.0,
            shrink_speed: 0.1,
            direction: 0.0,
            speed: 5.0,
            gravity: 0.5,
            rotation_speed: 0.1,
            start_color_r: 1.0,
            start_color_g: 0.0,
            start_color_b: 0.0,
            end_color_r: 0.0,
            end_color_g: 0.0,
            end_color_b: 1.0,
            random_deviation: 5.0,
            is_spawning_paused: false,
        }
    }
}

impl AppState {
    pub fn start_color(&self) -> Color {
        Color::rgb(self.start_color_r, self.start_color_g, self.start_color_b)
    }

    pub fn end_color(&self) -> Color {
        Color::rgb(self.end_color_r, self.end_color_g, self.end_color_b)
    }
}