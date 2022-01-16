use gdnative::prelude::*;

mod script;
mod player;
mod enemy;
mod shooter;

fn init(handle: InitHandle) {
    handle.add_class::<script::Script>();
    handle.add_class::<player::Player>();
    handle.add_class::<enemy::Enemy>();
    handle.add_class::<shooter::Shooter>();
}

godot_init!(init);