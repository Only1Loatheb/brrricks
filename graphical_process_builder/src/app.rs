use eframe::emath::Pos2;
use eframe::epaint::Rect;
use egui::{Key, Label, TextEdit, Ui, Vec2, Widget};
use crate::brick::*;

#[derive(serde::Deserialize, serde::Serialize)]
struct BrickRect {
  name: String,
  rect: Rect,
  brick: Brick,
}

const BRICK_INIT_SIZE: Vec2 = Vec2 { x: 100.0, y: 100.0 };

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
  bricks: Vec<BrickRect>,

  // Example stuff:
  label: String,
  #[serde(skip)] // This how you opt-out of serialization of a field
  value: f32,
}

impl Default for TemplateApp {
  fn default() -> Self {
    Self {
      bricks: vec![],
      // Example stuff:
      label: "Hello World!".to_owned(),
      value: 2.7,
    }
  }
}

impl TemplateApp {
  /// Called once before the first frame.
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    // This is also where you can customize the look and feel of egui using
    // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

    // Load previous app state (if any).
    // Note that you must enable the `persistence` feature for this to work.
    if let Some(storage) = cc.storage {
      return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
    }

    Default::default()
  }
}

impl eframe::App for TemplateApp {
  /// Called by the frame work to save state before shutdown.
  fn save(&mut self, storage: &mut dyn eframe::Storage) {
    eframe::set_value(storage, eframe::APP_KEY, self);
  }

  /// Called each time the UI needs repainting, which may be many times per second.
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
    // For inspiration and more examples, go to https://emilk.github.io/egui

    egui::CentralPanel::default().show(ctx, |ui| {
      if ctx.input(|i| i.pointer.primary_pressed()) {
        match ctx.pointer_interact_pos() {
          None => {
            println!("aaa")
          }
          Some(pos) => {
            self.bricks.push(
              BrickRect {
                name: "aaa".to_owned(),
                rect: Rect::from_center_size(pos, BRICK_INIT_SIZE),
                brick: Default::default(),
              });
          }
        }
      }

      for mut b in &mut self.bricks {
        ui.put(b.rect, |ui: &mut Ui| ui.code_editor(&mut b.name));
      }
    });

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
      // The top panel is often a good place for a menu bar:

      egui::menu::bar(ui, |ui| {
        // NOTE: no File->Quit on web pages!
        let is_web = cfg!(target_arch = "wasm32");
        if !is_web {
          ui.menu_button("File", |ui| {
            if ui.button("Quit").clicked() {
              ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
          });
          ui.menu_button("Clean", |ui| {
            if ui.button("I am sure").clicked() {
              self.bricks.clear();
            }
          });
          ui.add_space(16.0);
        }

        egui::widgets::global_dark_light_mode_buttons(ui);
      });
    });

  }
}
