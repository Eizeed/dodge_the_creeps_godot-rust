use godot::{classes::{animation, sprite_frames, AnimatedSprite2D, IRigidBody2D, RigidBody2D}, prelude::*};
use rand::seq::SliceRandom;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Enemy {
    pub min_speed: real,
    pub max_speed: real,

    base: Base<RigidBody2D>
}

#[godot_api]
impl Enemy {
    #[func]
    fn on_visiblity_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    fn on_start_game(&mut self) {
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IRigidBody2D for Enemy {
    fn init(base: Base<RigidBody2D>) -> Self {
        Self {
            min_speed: 150.0,
            max_speed: 250.0,
            base
        }
    }

    fn ready(&mut self) {
        let mut animation = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        animation.play();

        let animation_names = animation.get_sprite_frames().unwrap().get_animation_names();
        let animation_names = animation_names.to_vec();
        let mut rng = rand::thread_rng();
        let animation_name = animation_names.choose(&mut rng).unwrap();

        animation.set_animation(animation_name.arg());       
    }
}
