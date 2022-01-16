use gdnative::prelude::*;

#[derive(gdnative::NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Shooter {
    #[property(default = 25.0)]
    bullet_acceleration: f32,
    #[property(default = 250.0)]
    bullet_max_speed: f32,
    #[property]
    player_path: NodePath,
    #[property]
    enemy_path: NodePath,

    player: Option<Ref<Node2D, Shared>>,
    enemy: Option<Ref<Node2D, Shared>>,
    bullet: Option<Ref<KinematicBody2D, Shared>>,
    tex: Option<Ref<Node2D, Shared>>,
    velocity: Vector2,
    bullet_velocity: Vector2
}

#[gdnative::methods]
impl Shooter {
    fn new(_owner: &KinematicBody2D) -> Self {
        Self {
            bullet_acceleration: 25.0,
            bullet_max_speed: 250.0,
            player_path: "../Player".into(),
            enemy_path: "../Enemy".into(),
            player: None,
            enemy: None,
            tex: None,
            velocity: Vector2::zero(),
            bullet: None,
            bullet_velocity: Vector2::new(501.0, 501.0)
        }
    }

    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        self.bullet = unsafe { owner.get_node_as::<KinematicBody2D>("Bullet") }
            .map(|bullet| bullet.claim());
        self.player = unsafe { owner.get_node_as::<Node2D>(&self.player_path.to_string()) }
            .map(|player| player.claim());
        self.enemy = unsafe { owner.get_node_as::<Node2D>(&self.enemy_path.to_string()) }
            .map(|enemy| enemy.claim());
        self.tex = unsafe { owner.get_node_as::<Node2D>("cannon_tex") }
            .map(|tex| tex.claim());
        godot_print!("Shooter Ready");
    }

    #[export]
    fn _process(&mut self, owner: &KinematicBody2D, _delta: f32) {
        // get nodes
        let bullet = unsafe { self.bullet.unwrap().assume_safe() };
        let target = unsafe { self.player.unwrap().assume_safe() };
        let tex = unsafe { self.tex.unwrap().assume_safe() };

        // get properties
        let bullet_pos = bullet.global_position();
        let pos = owner.global_position();
        let target_pos = target.global_position();
        let target_dir = target_pos-pos;

        // rotate tex
        let target_angle = target_dir.angle_from_x_axis() + (Angle::pi()/2.0);
        tex.set_rotation(target_angle.get().into());

        // move bullet
        if (bullet_pos-pos).length() > 500.0 {
            bullet.set_global_position(pos);
            self.bullet_velocity = (target_dir*self.bullet_acceleration).clamped(self.bullet_max_speed);
        }

        // rotate bullet
        let bullet_angle = self.bullet_velocity.angle_from_x_axis() + (Angle::pi()/2.0);
        bullet.set_rotation(bullet_angle.get().into());
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f64) {
        let bullet = unsafe { self.bullet.unwrap().assume_safe() };
        bullet.move_and_slide(self.bullet_velocity, Vector2::zero(), false, 4, 0.785398, true);
        owner.move_and_slide(self.velocity, Vector2::zero(), false, 4, 0.785398, true);

        // detect collision with target
        for i in 0..bullet.get_slide_count() {
            let collision_unsafe = bullet.get_slide_collision(i).unwrap();
            let collision = unsafe {collision_unsafe.assume_safe()};
            let player = unsafe { self.player.unwrap().assume_safe() };
            let enemy = unsafe { self.enemy.unwrap().assume_safe() };
            if collision.collider_id() == player.get_instance_id() {
                let tree = unsafe { owner.get_tree().unwrap().assume_safe() };
                tree.change_scene("res://GameLoss.tscn").unwrap();
            } else if collision.collider_id() == enemy.get_instance_id() {
                let tree = unsafe { owner.get_tree().unwrap().assume_safe() };
                tree.change_scene("res://GameWin.tscn").unwrap();
            } else {
                let pos = owner.global_position();
                bullet.set_global_position(pos);
                let target_pos = player.global_position();
                self.bullet_velocity = ((target_pos-pos)*self.bullet_acceleration).clamped(self.bullet_max_speed);
            }
        }
    }
}
