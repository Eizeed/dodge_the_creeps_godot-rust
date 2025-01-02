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

    #[func]
    // _body is something that collided with Player
    // here we don't use it
    fn on_body_enter(&mut self, _body: Gd<PhysicsBody2D>) {
        // Signal call here
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
        // This vieport is global for all scenes
        // It is set from project settings in godot editor
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
        self.base_mut().hide();
    }


    // Delta is difference between current frame and previous one
    // For example player moves 100 px per frame. On 100 fps
    // he will be faster and on 30 slower
    // So to fix this delta is used
    // it grants smooth changes
    fn process(&mut self, delta: f64) {
        let input = Input::singleton();

        // To track direction of movement
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

        // Getting NODE for animations
        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        // triggers if move
        // set animation and direction of sprite
        if velocity.length() > 0.0 {
            let animation;
            
            // horizontal movement has prio
            if velocity.x != 0.0 {
                animation = "walk";

                animated_sprite.set_flip_v(false);
                // if going left flip sprite
                animated_sprite.set_flip_h(velocity.x < 0.0);
            } else {
                animation = "up";
                
                // same but for vertical
                animated_sprite.set_flip_v(velocity.y > 0.0);
            }

            // Getting speed of player and playing set animation
            // normalized makes shape of vector from square to circle
            // imagine you pressed down and right
            // range from 0 to your point will be 1^2 + 1^2 = 2^1/2
            // and roughly equals 1.414
            // So moving diagonally will be faster
            // normalized() makes it so all points of vector are
            // exact 1 far away from 0
            velocity = velocity.normalized() * self.speed;
            animated_sprite.play_ex().name(animation).done();
        } else {
            animated_sprite.stop();
        }

        // finding speed with delta
        let change = velocity * real::from_f64(delta);
        let position = self.base().get_global_position() + change;

        // limiting position with screen_size and clamp()
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );
        self.base_mut().set_global_position(position);
    }
}















