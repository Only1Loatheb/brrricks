use std::ops::Not;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Rect, Shape};
use egui::{Key, Label, Order, Stroke, TextEdit, Ui, Vec2, Widget};
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
  edges: Vec<(usize, usize)>,
  #[serde(skip)] // This how you opt-out of serialization of a field
  edge_start: Option<usize>,
}

impl Default for TemplateApp {
  fn default() -> Self {
    Self {
      bricks: vec![],
      edges: vec![],
      edge_start: None,
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
    // if let Some(storage) = cc.storage {
    //   return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
    // }

    Default::default()
  }
}

static line: Stroke = Stroke{ width: 10.0, color: Color32::LIGHT_BLUE };

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
      for b in &mut self.bricks.iter_mut(){
          ui.put(b.rect, |ui: &mut Ui| {
            ui.code_editor(&mut b.name)
          }
        );
      }
      ui.painter().add(Shape::Vec(self.edges.iter().map(|x| {
        Shape::LineSegment {
          points: [self.bricks[x.0].rect.center_bottom(), self.bricks[x.1].rect.center_top()],
          stroke: line,
        }
      }).collect()));

      if ctx.is_context_menu_open().not()
        && ctx.input(|i| i.pointer.primary_pressed()) {
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
              self.edges.clear();
            }
          });
          ui.add_space(16.0);
        }

        egui::widgets::global_dark_light_mode_buttons(ui);
      });
    });
  }
}
