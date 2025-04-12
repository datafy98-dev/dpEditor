use druid::{
    kurbo::Circle, piet::RenderContext, Color, Data, Env, PaintCtx, UpdateCtx, Widget,
    EventCtx, TimerToken, LifeCycleCtx, LifeCycle, BoxConstraints, Size, Event,
};
use crate::app_state::{AppState, Particle};
use std::sync::Arc;
use rand::Rng;

pub struct ParticlePreview {
    timer_id: TimerToken,
}

impl ParticlePreview {
    pub fn new() -> Self {
        Self {
            timer_id: TimerToken::INVALID,
        }
    }
}

impl Widget<AppState> for ParticlePreview {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::Timer(id) if *id == self.timer_id => {
                // Update particles
                let mut updated_particles = Vec::new();
                for mut particle in (*data.particles).iter().cloned() {
                    particle.position.0 += particle.velocity.0;
                    particle.position.1 += particle.velocity.1;
                    particle.velocity.1 += data.gravity; // Gravity
                    particle.size -= data.shrink_speed;
                    if particle.size > 0.0 {
                        updated_particles.push(particle);
                    }
                }
                data.particles = Arc::new(updated_particles);

                // Spawn new particles if not paused
                if !data.is_spawning_paused {
                    let new_particle = spawn_particle(data);
                    let mut particles = (*data.particles).clone();
                    particles.push(new_particle);
                    data.particles = Arc::new(particles);
                }

                // Request another timer tick
                self.timer_id = ctx.request_timer(std::time::Duration::from_millis(16));
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &AppState, _env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.timer_id = ctx.request_timer(std::time::Duration::from_millis(16));
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, bc: &BoxConstraints, _data: &AppState, _env: &Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();

        // Draw background
        ctx.fill(rect, &Color::BLACK);

        // Draw particles
        for particle in data.particles.iter() {
            let circle = Circle::new(
                druid::Point::new(particle.position.0, particle.position.1),
                particle.size,
            );
            ctx.fill(circle, &particle.color);
        }
    }
}

pub fn spawn_particle(data: &AppState) -> Particle {
    let mut rng = rand::rng();
    let angle = data.direction.to_radians();
    let deviation = data.random_deviation;
    let random_angle = rng.random_range(-deviation..deviation).to_radians();
    let velocity = (
        data.speed * (angle + random_angle).cos(),
        data.speed * (angle + random_angle).sin(),
    );
    let random_x = rng.random_range(-deviation..deviation);
    let random_y = rng.random_range(-deviation..deviation);
    Particle {
        position: (400.0 + random_x, 300.0 + random_y),
        velocity,
        size: data.initial_size,
        rotation: 0.0,
        color: data.start_color(),
    }
}