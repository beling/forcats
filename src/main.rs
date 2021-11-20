use macroquad::prelude::*;
use macroquad::rand::gen_range;

#[derive(Copy, Clone)]
enum RunnerState {
    LinearRun, Wait
}

struct Runner {
    position: Vec2,
    source_position: Vec2,
    destination_position: Vec2,
    current_time: f32,
    destination_time: f32,
    radius: f32,
    state: RunnerState,
}

impl Runner {
    fn random_destination() -> Vec2 {
        Vec2::new(gen_range(0.0, screen_width()), gen_range(0.0, screen_height()))
    }

    fn random_initial_position() -> Vec2 {
        let center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        polar_to_cartesian(center.length() * 2.0, gen_range(0.0, 2.0 * std::f32::consts::PI)) + center
    }

    fn update(&mut self, frame_time: f32) {
        self.current_time += frame_time;
        if self.current_time > self.destination_time {  // action finished, set new state:
            self.position = self.destination_position;
            self.current_time = 0.0;
            match self.state {
                RunnerState::LinearRun => {
                    self.destination_time = gen_range(0.0, 0.5);   // wait upto 0.5sec
                    self.state = RunnerState::Wait;
                }
                RunnerState::Wait => {
                    self.source_position = self.position;
                    self.destination_position = Self::random_destination();
                    self.destination_time = self.source_position.distance(self.destination_position) * gen_range(0.0001, 0.001);
                    self.state = RunnerState::LinearRun;
                }
            }
        } else {    // move:
            match self.state {
                RunnerState::LinearRun => {
                    self.position = self.source_position.lerp(self.destination_position,
                                                              (self.current_time / self.destination_time) as f32);
                }
                RunnerState::Wait => {}
            }
        }
    }

    fn draw(&self) {
        for i in 1..16 {
            draw_circle(self.position.x, self.position.y, self.radius-i as f32 * 3.0,
                        Color::new(i as f32 / 15.0, 0.0, 0.0, 1.0));
            /*let c = (15 - i) as f32 / 15.0;
            draw_circle(self.position.x, self.position.y, self.radius-i as f32 * 3.0,
                        Color::new(1.0, c, c, 1.0));*/
        }
    }

    fn collide(&self, point: Vec2) -> bool {
        self.position.distance_squared(point) <= self.radius * self.radius
    }

    fn is_touched(&self) -> bool {
        (is_mouse_button_down(MouseButton::Left) &&
            self.collide(mouse_position().into())) ||
            touches().iter().any(|t| t.phase != TouchPhase::Cancelled && self.collide(t.position))
    }

    fn respawn_on_touch(&mut self) -> bool {
        if self.is_touched() {
            *self = Self::default();
            true
        } else {
            false
        }
    }
}

impl Default for Runner {
    fn default() -> Self {
        let p = Self::random_initial_position();
        Self {
            source_position: p,
            destination_position: p,
            current_time: 0.0,
            destination_time: 0.0,
            state: RunnerState::Wait,
            position: p,
            radius: 50.0
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "forcats".to_owned(),
        //fullscreen: true,
        high_dpi: true,
        ..Default::default()
    }
}

//#[macroquad::main("forcats")]
#[macroquad::main(window_conf)]
async fn main() {
    simulate_mouse_with_touch(false);
    let mut score = 0u32;
    let mut dot = Runner::default();
    loop {
        let frame_time = get_frame_time();
        dot.update(frame_time);
        if dot.respawn_on_touch() { score += 1; }

        clear_background(BLACK);
        draw_text(&format!("{:>5}", score), 2.0, 20.0, 30.0, GRAY);
        dot.draw();

        next_frame().await
    }
}