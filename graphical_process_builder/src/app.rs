use crate::brick::*;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Rect, Shape};
use egui::Key::M;
use egui::{
    Context, Key, Label, Order, PointerButton, Response, Stroke, TextEdit, Ui, Vec2, Widget,
};
use std::cmp::max_by;
use std::ops::{Deref, Not};

#[derive(serde::Deserialize, serde::Serialize)]
struct BrickModel {
    name: String,
    rect: Rect,
    brick: Brick,
    uuid: usize,
}

const BRICK_INIT_SIZE: Vec2 = Vec2 { x: 100.0, y: 100.0 };

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    bricks: Vec<BrickModel>,
    edges: Vec<(usize, usize)>,
    #[serde(skip)]
    next_brick_uuid: usize,
    #[serde(skip)]
    edge_start: Option<(Pos2, usize)>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            bricks: vec![],
            edges: vec![],
            next_brick_uuid: 0,
            edge_start: None,
        }
    }
}

static line: Stroke = Stroke {
    width: 1.0,
    color: Color32::LIGHT_BLUE,
};
const EDGE_DRAW_BUTTON: PointerButton = PointerButton::Primary;

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut app: TemplateApp =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            app.next_brick_uuid = app
                .bricks
                .iter()
                .map(|brick| brick.uuid)
                .max_by(|left, right| left.cmp(&right))
                .unwrap_or(0)
                + 1;
            return app;
        }

        Default::default()
    }

    fn edges(&mut self, ui: &Ui) {
        ui.painter().extend(self.edges.iter().filter_map(|edge| {
            if let (Some(a), Some(b)) = (
                self.bricks.iter().find(|brick| brick.uuid == edge.0),
                self.bricks.iter().find(|brick| brick.uuid == edge.1),
            ) {
                Some(Shape::LineSegment {
                    points: [a.rect.center_bottom(), b.rect.center_top()],
                    stroke: line,
                })
            } else {
                None
            }
        }));
    }

    fn bricks(&mut self, ctx: &Context, ui: &mut Ui) {
        if let (true, Some(mouse), Some(edge_start)) = (
            ctx.input(|i| i.pointer.primary_released()),
            ctx.pointer_latest_pos(),
            self.edge_start,
        ) {
            let maybe_edge = self.retain_bricks_with_add_edge(ui, |(brick_model, brick_rect)| {
                TemplateApp::create_edge(mouse, edge_start.1, brick_model, brick_rect)
            });
            if let Some(edge) = maybe_edge {
                self.edges.push(edge);
            }
            self.edge_start = None;
        } else {
            self.retain_bricks_with_add_edge(ui, |_| None);
        }
    }

    fn retain_bricks_with_add_edge(
        &mut self,
        ui: &mut Ui,
        create_edge: impl Fn((&BrickModel, Rect)) -> Option<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        let mut edge_to_add = None;
        self.bricks.retain_mut(|brick_model| {
            let mut keep = true;
            ui.put(brick_model.rect, |ui: &mut Ui| {
                let brick_script_editor = ui.code_editor(&mut brick_model.name);
                keep = brick_script_editor
                    .clicked_by(PointerButton::Secondary)
                    .not();
                if brick_script_editor.drag_started_by(EDGE_DRAW_BUTTON) {
                    println!("start edge");
                    self.edge_start = Some((brick_model.rect.center_bottom(), brick_model.uuid));
                }
                let maybe_created_edge = create_edge((&brick_model, brick_script_editor.rect));
                edge_to_add = edge_to_add.or(maybe_created_edge);
                brick_script_editor
            });
            if keep.not() {
                self.edges
                    .retain(|edge| edge.0 != brick_model.uuid && edge.1 != brick_model.uuid)
            }
            keep
        });
        edge_to_add
    }

    fn create_edge(
        mouse: Pos2,
        edge_start: usize,
        brick_model: &BrickModel,
        brick_rect: Rect,
    ) -> Option<(usize, usize)> {
        if brick_rect.contains(mouse) && edge_start != brick_model.uuid {
            dbg!("draw", edge_start, brick_model.uuid);
            Some((edge_start, brick_model.uuid))
        } else {
            None
        }
    }

    fn show_edge_creation(&mut self, ctx: &Context, ui: &Ui) {
        if let (Some(edge_start), Some(mouse)) = (self.edge_start, ctx.pointer_latest_pos()) {
            ui.painter().add(Shape::LineSegment {
                points: [edge_start.0, mouse],
                stroke: line,
            });
        }
    }

    fn handle_brick_creation(&mut self, ctx: &Context) {
        if ctx.is_context_menu_open().not()
            && ctx.wants_keyboard_input().not()
            && ctx.input(|i| i.pointer.primary_pressed())
        {
            match ctx.pointer_interact_pos() {
                Some(pos)
                    if ctx
                        .layer_id_at(pos)
                        .is_some_and(|x| x.order != Order::Background) =>
                {
                    println!("over something")
                }
                Some(pos) => {
                    println!("create brick");
                    self.bricks.push(BrickModel {
                        name: "aaa".to_owned(),
                        rect: Rect::from_center_size(pos, BRICK_INIT_SIZE),
                        brick: Default::default(),
                        uuid: self.next_brick_uuid,
                    });
                    self.next_brick_uuid += 1;
                }
                None => {
                    println!("not create brick")
                }
            }
        }
    }

    fn menu(&mut self, ctx: &Context, ui: &mut Ui) {
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
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.bricks(ctx, ui);
            self.edges(ui);
            self.show_edge_creation(ctx, ui);
            self.handle_brick_creation(ctx);
        });
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    self.menu(ctx, ui);
                }
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }
}
