use godot::{classes::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, PhysicsBody2D}, obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    speed: real,
    screen_size: Vector2,
    
    base: Base<Area2D>
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();

    pub fn on_esc_click(&mut self) {
        self.base_mut().hide();
        self.base_mut().emit_signal("hit", &[]);

        let mut collision = self.base().get_node_as::<CollisionShape2D>("CollisionShape2D");
        collision.set_deferred("disabled", &true.to_variant());
    }

    #[func]
    fn on_body_enter(&mut self, _body: Gd<PhysicsBody2D>) {
        self.base_mut().emit_signal("hit", &[]);

    }

    #[func]
    pub fn start(&mut self, pos: Vector2) {
        self.base_mut().set_global_position(pos);
        self.base_mut().show();
        let mut collision = self.base_mut().get_node_as::<CollisionShape2D>("CollisionShape2D");
        collision.set_disabled(false);
    }
}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            speed: 400.0,
            screen_size: Vector2::ZERO,
            base,
        }
    }

    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
        self.base_mut().hide();
    }

    fn process(&mut self, delta: f64) {
        let input = Input::singleton();

        let mut velocity = Vector2::ZERO;

        if input.is_action_pressed("move_right") {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("move_left") {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("move_up") {
            velocity += Vector2::UP;
        }
        if input.is_action_pressed("move_down") {
            velocity += Vector2::DOWN;
        }
        // if input.is_action_pressed("end_game") {
        //     self.on_esc_click();
        // }

        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        if velocity.length() > 0.0 {
            let animation;

            if velocity.x != 0.0 {
                animation = "walk";

                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0);
            } else {
                animation = "up";

                animated_sprite.set_flip_v(velocity.y > 0.0);
            }

            velocity = velocity.normalized() * self.speed;
            animated_sprite.play_ex().name(animation).done();
        } else {
            animated_sprite.stop();
        }

        let change = velocity * real::from_f64(delta);
        let position = self.base().get_global_position() + change;
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );
        self.base_mut().set_global_position(position);
    }
}















