use godot::{classes::{AnimatedSprite2D, IRigidBody2D, RigidBody2D}, prelude::*};
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
        // queue_free set object to be deleted
        // on the next frame
        self.base_mut().queue_free();
    }

    #[func]
    // same works here but for start game
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
        // getting node and playing its animation
        // it won't play anything because
        // I didn't set any default animation
        let mut animation = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        animation.play();

        // get_sprite_frames is returning some kind of storage
        // for animations and all related to them things
        // for this node ONLY
        let animation_names = animation.get_sprite_frames().unwrap().get_animation_names();
        let animation_names = animation_names.to_vec();
        let mut rng = rand::thread_rng();
        let animation_name = animation_names.choose(&mut rng).unwrap();

        animation.set_animation(animation_name.arg());       
    }
}










