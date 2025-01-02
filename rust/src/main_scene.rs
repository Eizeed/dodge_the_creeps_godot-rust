use std::f32::consts::PI;

use godot::{classes::{CollisionShape2D, Marker2D, PathFollow2D, RigidBody2D, Timer}, obj::{NewGd, WithBaseField}, prelude::*};
use rand::Rng;

use crate::{enemy, hud, player};

#[derive(GodotClass)]
#[class(base=Node)]
struct MainScene {
    // mob_scene isn't child node of main scene because
    // here it is a template to create a lot of them
    // it also allows us to take control over mob_scene
    // in main scene
    mob_scene: Gd<PackedScene>,
    score: u32,
    music: Option<Gd<AudioStreamPlayer>>,
    death_sound: Option<Gd<AudioStreamPlayer>>,

    base: Base<Node>
}

#[godot_api]
impl MainScene {
    #[func]
    // I moved some functionality from on_body_enter() from Player
    // to be able to call on_body_enter() function on ESC click
    // I couldn't use self if i used input, so now i'm just calling
    // hit signal from there, and all login is here in game_over()
    fn game_over(&mut self) {
        // getting node with name Player and trying to cast it
        // to our Player struct
        let mut player = self.base().get_node_as::<player::Player>("Player");

        // here we cast it as GD class with all methods
        let mut player = player.bind_mut();

        player.base_mut().hide();
        let mut collision = player.base().get_node_as::<CollisionShape2D>("CollisionShape2D");
        collision.set_deferred("disabled", &true.to_variant());

        let mut score_timer = self.base().get_node_as::<Timer>("ScoreTimer");
        let mut mob_timer = self.base().get_node_as::<Timer>("MobTimer");
        score_timer.stop();
        mob_timer.stop();

        let mut hud = self.base().get_node_as::<hud::Hud>("Hud");
        hud.bind_mut().show_gameover();

        self.music().stop();
        self.death_sound().play();
    }

    #[func]
    fn new_game(&mut self) {
        self.score = 0;

        let position = self.base().get_node_as::<Marker2D>("StartPos");
        let mut player = self.base().get_node_as::<player::Player>("Player");

        // start() is called from Player implementation
        player.bind_mut().start(position.get_position());
    
        // starting only this timer becuase its timeout
        // signal triggers other timers
        let mut start_timer = self.base_mut().get_node_as::<Timer>("StartTimer");
        start_timer.start();

        let mut hud = self.base().get_node_as::<hud::Hud>("Hud");
        let mut hud = hud.bind_mut();
        hud.update_score(self.score);
        hud.show_message("Get Ready".into());

        self.music().play();
    }

    #[func]
    // In godot one shot is set to true
    // it makes timer run only once without repeating
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;
        let mut hud = self.base().get_node_as::<hud::Hud>("Hud");

        // Every timeout signal we update score
        hud.bind_mut().update_score(self.score);
    }

    #[func]
    fn on_start_timer_timeout(&mut self) {
        // After timeout of StartTimer we start 2 timers
        let mut score_timer = self.base_mut().get_node_as::<Timer>("ScoreTimer");
        let mut mob_timer = self.base_mut().get_node_as::<Timer>("MobTimer");
        score_timer.start();
        mob_timer.start();
    }

    #[func]
    fn on_mob_timer_timeout(&mut self) {
        // getting our struct field as scene where we can
        // use all its methods
        let mut mob_scene = self.mob_scene.instantiate_as::<RigidBody2D>();
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");

        let mut rng = rand::thread_rng();
        let progress = rng.gen_range(u32::MIN..u32::MAX);

        // Progress is value for PathFollow2D
        mob_spawn_location.set_progress(progress as f32);
        mob_scene.set_position(mob_spawn_location.get_position());

        // Making direction perpendicular to mob_spawn_location
        let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        // Makeing it a bit random from -45 to +45 from its
        // perpendicular line
        // PI is 180 deg, 
        // so PI / 2 is perpendicular or 90 deg
        // and PI / 4 is 45 deg
        direction += rng.gen_range(-PI / 4.0..PI / 4.0);

        mob_scene.set_rotation(direction);

        // Adding initialized mob_scene with all config
        // into our MainScene as its child node
        self.base_mut().add_child(&mob_scene);

        let mut enemy = mob_scene.cast::<enemy::Enemy>();
        // Generating random number between 2 constants
        // in our struct 150.0 and 250.0
        // for that we have to bind to our
        // as our Struct and not GD class
        let range = {
            let enemy = enemy.bind();
            rng.gen_range(enemy.min_speed..enemy.max_speed)
        };

        // We set only X speed because we rotate mob's direction
        // with rotated() on same amount of radians as we did, so now
        // enemy moves only on X 
        enemy.set_linear_velocity(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));

        let mut hud = self.base().get_node_as::<hud::Hud>("Hud");

        // Binding signal start_game from our Hud struct for
        // enemy on_start_game()
        // So now when start_game signal is emited it will trigger
        // on_start_game as well
        hud.connect("start_game", &enemy.callable("on_start_game"));
    }

    fn music(&mut self) -> &mut AudioStreamPlayer {
        self.music.as_deref_mut().unwrap()
    }

    fn death_sound(&mut self) -> &mut AudioStreamPlayer {
        self.death_sound.as_deref_mut().unwrap()
    }
}

#[godot_api]
impl INode for MainScene {
    fn init(base: Base<Node>) -> Self {
        Self {
            mob_scene: PackedScene::new_gd(),
            music: None,
            death_sound: None,
            score: 0,
            base
        }
    }

    fn ready(&mut self) {
        // Loading resource from root dir of godot project
        self.mob_scene = load("res://enemy.tscn");

        // Here were using child nodes
        // so no need to use load
        self.music = Some(self.base().get_node_as::<AudioStreamPlayer>("Music"));
        self.death_sound = Some(self.base().get_node_as::<AudioStreamPlayer>("Deathsound"));
    }

    // Here's my addition
    // i did bind for ESC
    // to end game with all
    // its logic like disabling collision
    // hiding player etc.
    // Thats why all this login now contains
    // in game_over()
    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();

        if input.is_action_pressed("end_game") {
            self.game_over();
        }
    }
}
