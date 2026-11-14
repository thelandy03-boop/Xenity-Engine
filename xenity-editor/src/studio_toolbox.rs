use eframe::egui::{self, Color32, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::studio_style::*;
use crate::icons;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ToolboxTab {
    Marketplace,
    Inventory,
    Recent,
    Creations,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ToolboxCategory {
    Models,
    Images,
    Meshes,
    Audio,
    Videos,
}

pub struct ToolboxItem {
    pub name: &'static str,
    pub creator: &'static str,
    pub rating: u32,
    pub votes: u32,
    pub category: ToolboxCategory,
    pub color_preview: Color32,
}

pub struct StudioToolboxState {
    pub current_tab: ToolboxTab,
    pub current_category: ToolboxCategory,
    pub search_query: String,
    pub items: Vec<ToolboxItem>,
}

impl Default for StudioToolboxState {
    fn default() -> Self {
        Self {
            current_tab: ToolboxTab::Marketplace,
            current_category: ToolboxCategory::Models,
            search_query: String::new(),
            items: vec![
                ToolboxItem {
                    name: "Drooling Zombie (R15)",
                    creator: "Roblox",
                    rating: 94,
                    votes: 18200,
                    category: ToolboxCategory::Models,
                    color_preview: Color32::from_rgb(60, 140, 75),
                },
                ToolboxItem {
                    name: "Low Poly Pine Tree Pack",
                    creator: "NatureBuilder",
                    rating: 91,
                    votes: 8400,
                    category: ToolboxCategory::Models,
                    color_preview: Color32::from_rgb(34, 110, 52),
                },
                ToolboxItem {
                    name: "Sports Car (A-Chassis)",
                    creator: "VehicleWorks",
                    rating: 89,
                    votes: 24500,
                    category: ToolboxCategory::Models,
                    color_preview: Color32::from_rgb(220, 50, 45),
                },
                ToolboxItem {
                    name: "Classic Sword Weapon",
                    creator: "Roblox",
                    rating: 96,
                    votes: 45000,
                    category: ToolboxCategory::Models,
                    color_preview: Color32::from_rgb(180, 185, 195),
                },
                ToolboxItem {
                    name: "Street Lamp (PointLight)",
                    creator: "CityAssets",
                    rating: 88,
                    votes: 3200,
                    category: ToolboxCategory::Models,
                    color_preview: Color32::from_rgb(245, 210, 80),
                },
                ToolboxItem {
                    name: "Medieval Stone Castle",
                    creator: "CastleCrafter",
                    rating: 92,
                    votes: 11300,
                    category: ToolboxCategory::Models,
                    color_preview: Color32::from_rgb(120, 120, 125),
                },
                ToolboxItem {
                    name: "Military Tactical Humvee",
                    creator: "ArmoryStudios",
                    rating: 87,
                    votes: 9100,
                    category: ToolboxCategory::Models,
                    color_preview: Color32::from_rgb(140, 130, 95),
                },
                ToolboxItem {
                    name: "Laser Blaster Pistol",
                    creator: "SciFiForge",
                    rating: 93,
                    votes: 15400,
                    category: ToolboxCategory::Models,
                    color_preview: Color32::from_rgb(0, 162, 255),
                },
            ],
        }
    }
}

impl StudioToolboxState {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        let mut inserted_model = None;

        // 1. PESTAÑA SUPERIOR [ Toolbox ]
        let tab_h = 26.0;
        let (tab_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), tab_h), Sense::hover());
        ui.painter().rect_filled(tab_rect, 0.0, STUDIO_BG);

        let active_tab_rect = Rect::from_min_size(tab_rect.min, Vec2::new(88.0, tab_h));
        ui.painter().rect_filled(active_tab_rect, Rounding { nw: 3.0, ne: 3.0, sw: 0.0, se: 0.0 }, STUDIO_TREE_BG);
        ui.painter().line_segment(
            [Pos2::new(active_tab_rect.min.x, active_tab_rect.min.y), Pos2::new(active_tab_rect.max.x, active_tab_rect.min.y)],
            Stroke::new(2.0, STUDIO_SELECTION),
        );
        ui.painter().text(active_tab_rect.center(), egui::Align2::CENTER_CENTER, "Toolbox", FontId::proportional(11.5), Color32::WHITE);

        let menu_pos = Pos2::new(tab_rect.max.x - 12.0, tab_rect.center().y);
        icons::kebab_menu(ui.painter(), menu_pos, STUDIO_TEXT_MUTED);

        ui.painter().line_segment(
            [Pos2::new(tab_rect.min.x, tab_rect.max.y), Pos2::new(tab_rect.max.x, tab_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // 2. SUB-BARRA DE PESTAÑAS (Marketplace, Inventory, Recent, Creations)
        let sub_tab_h = 26.0;
        let (sub_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), sub_tab_h), Sense::hover());
        ui.painter().rect_filled(sub_rect, 0.0, STUDIO_BG);

        let mut child_sub = ui.child_ui(sub_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        child_sub.add_space(6.0);

        let tabs = [
            (ToolboxTab::Marketplace, "Marketplace"),
            (ToolboxTab::Inventory, "Inventory"),
            (ToolboxTab::Recent, "Recent"),
            (ToolboxTab::Creations, "Creations"),
        ];

        for (tab, label) in tabs {
            let is_sel = self.current_tab == tab;
            let txt_col = if is_sel { STUDIO_SELECTION } else { STUDIO_TEXT };
            if child_sub.selectable_label(is_sel, egui::RichText::new(label).size(11.0).color(txt_col)).clicked() {
                self.current_tab = tab;
            }
        }

        ui.painter().line_segment(
            [Pos2::new(sub_rect.min.x, sub_rect.max.y), Pos2::new(sub_rect.max.x, sub_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // 3. BARRA DE BÚSQUEDA (Search Marketplace...)
        let search_bar_h = 32.0;
        let (sbar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), search_bar_h), Sense::hover());
        ui.painter().rect_filled(sbar_rect, 0.0, STUDIO_BG);

        let input_h = 22.0;
        let input_y = sbar_rect.min.y + (search_bar_h - input_h) * 0.5;
        let input_rect = Rect::from_min_size(Pos2::new(sbar_rect.min.x + 8.0, input_y), Vec2::new(sbar_rect.width() - 16.0, input_h));

        ui.painter().rect_filled(input_rect, Rounding::same(4.0), STUDIO_INPUT_BG);
        ui.painter().rect_stroke(input_rect, Rounding::same(4.0), Stroke::new(1.0, STUDIO_BORDER));

        icons::magnifying_glass(ui.painter(), Pos2::new(input_rect.min.x + 12.0, input_rect.center().y), 3.8, STUDIO_TEXT_MUTED);

        let clear_w = if self.search_query.is_empty() { 0.0 } else { 18.0 };
        let text_rect = Rect::from_min_max(
            Pos2::new(input_rect.min.x + 24.0, input_rect.min.y),
            Pos2::new(input_rect.max.x - 6.0 - clear_w, input_rect.max.y),
        );

        let mut edit_ui = ui.child_ui(text_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        edit_ui.add(
            egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Search Marketplace...")
                .frame(false)
                .font(FontId::proportional(11.5))
                .desired_width(text_rect.width()),
        );

        if !self.search_query.is_empty() {
            let clear_center = Pos2::new(input_rect.max.x - 10.0, input_rect.center().y);
            let clear_rect = Rect::from_center_size(clear_center, Vec2::new(14.0, 14.0));
            let clear_resp = ui.interact(clear_rect, ui.id().with("clear_tb_search"), Sense::click());
            ui.painter().text(clear_center, egui::Align2::CENTER_CENTER, "✕", FontId::proportional(10.0), STUDIO_TEXT);
            if clear_resp.clicked() {
                self.search_query.clear();
            }
        }

        // 4. CHIPS DE CATEGORÍAS (Models, Images, Meshes, Audio)
        let cat_bar_h = 24.0;
        let (cat_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), cat_bar_h), Sense::hover());
        ui.painter().rect_filled(cat_rect, 0.0, STUDIO_BG);

        let mut child_cat = ui.child_ui(cat_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        child_cat.add_space(6.0);

        let categories = [
            (ToolboxCategory::Models, "Models"),
            (ToolboxCategory::Images, "Images"),
            (ToolboxCategory::Meshes, "Meshes"),
            (ToolboxCategory::Audio, "Audio"),
        ];

        for (cat, label) in categories {
            let is_sel = self.current_category == cat;
            let bg = if is_sel { STUDIO_SELECTION } else { Color32::from_rgb(48, 48, 52) };
            let (chip_rect, chip_resp) = child_cat.allocate_exact_size(Vec2::new(50.0, 18.0), Sense::click());
            child_cat.painter().rect_filled(chip_rect, Rounding::same(9.0), bg);
            child_cat.painter().text(chip_rect.center(), egui::Align2::CENTER_CENTER, label, FontId::proportional(10.5), Color32::WHITE);
            if chip_resp.clicked() {
                self.current_category = cat;
            }
            child_cat.add_space(4.0);
        }

        ui.painter().line_segment(
            [Pos2::new(cat_rect.min.x, cat_rect.max.y), Pos2::new(cat_rect.max.x, cat_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // 5. GRID DE TARJETAS DE ASSETS
        let body_rect = Rect::from_min_size(ui.cursor().min, ui.available_size());
        ui.painter().rect_filled(body_rect, 0.0, STUDIO_TREE_BG);

        let query = self.search_query.to_lowercase();
        let avail_w = ui.available_width();
        let card_w = ((avail_w - 20.0) / 2.0).clamp(90.0, 140.0);
        let card_h = 130.0;

        egui::ScrollArea::vertical()
            .id_salt("studio_toolbox_scroll")
            .show(ui, |ui| {
                ui.add_space(6.0);

                let filtered_items: Vec<_> = self.items.iter()
                    .filter(|item| query.is_empty() || item.name.to_lowercase().contains(&query))
                    .collect();

                let mut row_items = Vec::new();
                for item in filtered_items {
                    row_items.push(item);
                    if row_items.len() == 2 {
                        ui.horizontal(|ui| {
                            ui.add_space(6.0);
                            for item in row_items.drain(..) {
                                if draw_asset_card(ui, item, card_w, card_h) {
                                    inserted_model = Some(item.name.to_string());
                                }
                                ui.add_space(6.0);
                            }
                        });
                        ui.add_space(6.0);
                    }
                }

                if !row_items.is_empty() {
                    ui.horizontal(|ui| {
                        ui.add_space(6.0);
                        for item in row_items {
                            if draw_asset_card(ui, item, card_w, card_h) {
                                inserted_model = Some(item.name.to_string());
                            }
                            ui.add_space(6.0);
                        }
                    });
                }
            });

        inserted_model
    }
}

fn draw_asset_card(ui: &mut egui::Ui, item: &ToolboxItem, w: f32, h: f32) -> bool {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, h), Sense::click());

    let bg = if resp.hovered() { STUDIO_BTN_HOVER } else { Color32::from_rgb(38, 38, 42) };
    ui.painter().rect_filled(rect, Rounding::same(4.0), bg);
    ui.painter().rect_stroke(rect, Rounding::same(4.0), Stroke::new(1.0, STUDIO_BORDER));

    // Thumbnail
    let thumb_h = w - 16.0;
    let thumb_rect = Rect::from_min_size(Pos2::new(rect.min.x + 8.0, rect.min.y + 8.0), Vec2::new(w - 16.0, thumb_h));
    ui.painter().rect_filled(thumb_rect, Rounding::same(3.0), Color32::from_rgb(28, 28, 32));

    // 3D Isometric Preview Box inside thumbnail
    let box_center = thumb_rect.center();
    icons::roblox_model(ui.painter(), box_center, thumb_h * 0.7);

    // Title label
    let title_y = thumb_rect.max.y + 6.0;
    let title_rect = Rect::from_min_size(Pos2::new(rect.min.x + 8.0, title_y), Vec2::new(w - 16.0, 14.0));
    ui.painter().text(
        title_rect.min,
        egui::Align2::LEFT_TOP,
        item.name,
        FontId::proportional(10.5),
        Color32::WHITE,
    );

    // Creator & Rating
    let creator_y = title_y + 16.0;
    let meta_str = format!("By {} • 👍 {}%", item.creator, item.rating);
    ui.painter().text(
        Pos2::new(rect.min.x + 8.0, creator_y),
        egui::Align2::LEFT_TOP,
        meta_str,
        FontId::proportional(9.0),
        STUDIO_TEXT_MUTED,
    );

    if resp.hovered() {
        ui.painter().rect_stroke(rect, Rounding::same(4.0), Stroke::new(1.5, STUDIO_SELECTION));
    }

    resp.clicked()
}
