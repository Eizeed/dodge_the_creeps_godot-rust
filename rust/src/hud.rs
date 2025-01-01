use godot::{classes::{Button, CanvasLayer, ICanvasLayer, Label, Timer}, obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct Hud {
    base: Base<CanvasLayer>
}

#[godot_api]
impl Hud {
    #[signal]
    fn start_game();

    #[func]
    pub fn show_message(&self, text: GString) {
        let mut message = self.base().get_node_as::<Label>("Message");
        let mut message_timer = self.base().get_node_as::<Timer>("MessageTimer");

        message.set_text(&text);
        message.show();

        message_timer.start();
    }

    #[func]
    pub fn show_gameover(&mut self) {
        self.show_message("Game over".into());

        let mut timer = self.base().get_tree().unwrap().create_timer(2.0).unwrap();
        timer.connect("timeout", &self.base().callable("show_start_button"));
    }

    #[func]
    fn show_start_button(&mut self) {
        let mut message = self.base().get_node_as::<Label>("Message");
        message.set_text("Dodge the creeps!");
        message.show();

        let mut button = self.base().get_node_as::<Button>("StartButton");
        button.show();
    }

    #[func]
    pub fn update_score(&mut self, score: u32) {
        let mut score_label = self.base().get_node_as::<Label>("ScoreLabel");
        score_label.set_text(&score.to_string());
    }

    #[func]
    fn on_start_button_pressed(&mut self) {
        let mut button = self.base().get_node_as::<Button>("StartButton");
        button.hide();

        self.base_mut().emit_signal("start_game", &[]);
    }

    #[func]
    fn on_message_timer_timeout(&mut self) {
        let mut message = self.base().get_node_as::<Label>("Message");
        message.hide();
    }

}

#[godot_api]
impl ICanvasLayer for Hud {
    fn init(base: Base<CanvasLayer>) -> Self {
        Self {
            base
        }
    }
}
