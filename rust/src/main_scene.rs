use std::{f32::consts::PI, ops::DerefMut};

use godot::{classes::{Marker2D, PathFollow2D, RigidBody2D, Time, Timer}, obj::{NewGd, WithBaseField}, prelude::*};
use rand::Rng;

use crate::{enemy, hud, player};

#[derive(GodotClass)]
#[class(base=Node)]
struct MainScene {
    mob_scene: Gd<PackedScene>,
    score: u32,
    music: Option<Gd<AudioStreamPlayer>>,
    death_sound: Option<Gd<AudioStreamPlayer>>,

    base: Base<Node>
}

#[godot_api]
impl MainScene {
    #[func]
    fn game_over(&mut self) {
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
        player.bind_mut().start(position.get_position());

        let mut start_timer = self.base_mut().get_node_as::<Timer>("StartTimer");
        start_timer.start();

        let mut hud = self.base().get_node_as::<hud::Hud>("Hud");
        let mut hud = hud.bind_mut();
        hud.update_score(self.score);
        hud.show_message("Get Ready".into());

        self.music().play();
    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;
        let mut hud = self.base().get_node_as::<hud::Hud>("Hud");
        hud.bind_mut().update_score(self.score);
    }

    #[func]
    fn on_start_timer_timeout(&mut self) {
        let mut score_timer = self.base_mut().get_node_as::<Timer>("ScoreTimer");
        let mut mob_timer = self.base_mut().get_node_as::<Timer>("MobTimer");
        score_timer.start();
        mob_timer.start();
    }

    #[func]
    fn on_mob_timer_timeout(&mut self) {
        let mut mob_scene = self.mob_scene.instantiate_as::<RigidBody2D>();
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");

        let mut rng = rand::thread_rng();
        let progress = rng.gen_range(u32::MIN..u32::MAX);

        mob_spawn_location.set_progress(progress as f32);
        mob_scene.set_position(mob_spawn_location.get_position());

        let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        direction += rng.gen_range(-PI / 4.0..PI / 4.0);

        mob_scene.set_rotation(direction);

        self.base_mut().add_child(&mob_scene);

        let mut enemy = mob_scene.cast::<enemy::Enemy>();
        let range = {
            let enemy = enemy.bind();
            rng.gen_range(enemy.min_speed..enemy.max_speed)
        };

        enemy.set_linear_velocity(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));
        let mut hud = self.base().get_node_as::<hud::Hud>("Hud");
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
        self.mob_scene = load("res://enemy.tscn");
        self.music = Some(self.base().get_node_as::<AudioStreamPlayer>("Music"));
        self.death_sound = Some(self.base().get_node_as::<AudioStreamPlayer>("Deathsound"));
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();

        if input.is_action_pressed("end_game") {
            self.base_mut().emit_signal("hit", &[]);
            self.game_over();
        }
    }
}
