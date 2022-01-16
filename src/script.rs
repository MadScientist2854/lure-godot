use gdnative::prelude::*;

#[derive(gdnative::NativeClass)]
#[inherit(Node2D)]
pub struct Script;

#[gdnative::methods]
impl Script {
    fn new(_owner: &Node2D) -> Self {
        Self {}
    }

    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        godot_print!("Scene Ready");
    }

    #[export]
    fn _process(&mut self, _owner: &Node2D, _delta: f64) {
        //
    }
}