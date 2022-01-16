use gdnative::prelude::*;

#[derive(gdnative::NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    #[property(default = 25.0)]
    acceleration: f32,
    #[property(default = 250.0)]
    max_speed: f32,

    velocity: Vector2
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Self {
            acceleration: 25.0,
            max_speed: 250.0,
            velocity: Vector2::zero()
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &KinematicBody2D) {
        godot_print!("Player Ready");
    }

    #[export]
    fn _process(&mut self, _owner: &KinematicBody2D, _delta: f32) {
        let input: &Input = Input::godot_singleton();
        let mut stopping = true;
        if input.is_action_pressed("up") {
            self.velocity.y -= self.acceleration;
            stopping = false;
        }
        if input.is_action_pressed("down") {
            self.velocity.y += self.acceleration;
            stopping = false;
        }
        if input.is_action_pressed("left") {
            self.velocity.x -= self.acceleration;
            stopping = false;
        }
        if input.is_action_pressed("right") {
            self.velocity.x += self.acceleration;
            stopping = false;
        }
        if stopping {
            self.velocity = self.velocity.lerp(Vector2::zero(), self.acceleration/self.max_speed);
        }
        self.velocity = self.velocity.clamped(self.max_speed);
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f64) {
        owner.move_and_slide(self.velocity, Vector2::new(0.0, 0.0), false, 4, 0.785398, true);
    }
}