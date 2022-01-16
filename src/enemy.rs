use gdnative::prelude::*;

#[derive(gdnative::NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Enemy {
    #[property(default = 25.0)]
    acceleration: f32,
    #[property(default = 225.0)]
    max_speed: f32,
    #[property(default = 50.0)]
    max_see_ahead: f32,
    #[property]
    target_path: NodePath,

    target: Option<Ref<Node2D, Shared>>,
    velocity: Vector2
}

#[gdnative::methods]
impl Enemy {

    fn new(_owner: &KinematicBody2D) -> Self {
        Self {
            acceleration: 25.0,
            max_speed: 225.0,
            max_see_ahead: 50.0,
            target_path: "../Player".into(),
            target: None,
            velocity: Vector2::zero()
        }
    }

    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        self.target = unsafe { owner.get_node_as::<Node2D>(&self.target_path.to_string()) }
            .map(|target| target.claim());
        godot_print!("Enemy Ready");
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f64) {
        // get positions
        let pos = owner.global_position();
        let target = unsafe { self.target.unwrap().assume_safe() };
        let target_pos = target.global_position();
        // seek target
        let desired_velocity = (target_pos-pos)*self.max_speed;
        let steering = (desired_velocity - self.velocity).normalize();

        // avoid obstacles
        let ahead = pos + self.velocity.normalize()*self.max_see_ahead;
        let space_state = unsafe {owner.get_world_2d().unwrap().assume_safe().direct_space_state().unwrap().assume_safe()};
        let cast = space_state.intersect_ray(pos, ahead, VariantArray::new_shared(), 0x7FFFFFFE, true, false);
        if !cast.is_empty() {
            let avoidance = cast.get("normal").to_vector2();
            self.velocity = (self.velocity + ((avoidance+steering).normalize()*self.acceleration)).clamped(self.max_speed);
        } else {
            self.velocity = (self.velocity + (steering*self.acceleration)).clamped(self.max_speed);
        }

        // rotate according to velocity
        let angle_90: Angle = Angle::pi()/2.0;
        let angle = desired_velocity.angle_from_x_axis() + angle_90;
        owner.set_rotation(angle.get().into());
        if angle > angle_90 * 2.0 && angle < angle_90 * 4.0
            { owner.set_scale(Vector2::new(-0.75, 0.75)); }
        else { owner.set_scale(Vector2::new(0.75, 0.75)); }

        // apply velocity
        self.velocity = owner.move_and_slide(self.velocity, Vector2::new(0.0, 0.0), false, 4, 0.785398, true);

        // detect collision with target
        for i in 0..owner.get_slide_count() {
            let collision_unsafe = owner.get_slide_collision(i).unwrap();
            let collision = unsafe {collision_unsafe.assume_safe()};
            if collision.collider_id() == target.get_instance_id() {
                let tree = unsafe { owner.get_tree().unwrap().assume_safe() };
                tree.change_scene("res://GameLoss.tscn").unwrap();
            }
        }
    }
}