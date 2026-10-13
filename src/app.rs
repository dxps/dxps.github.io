use eframe::egui::{self, Align, Layout, ThemePreference};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

use crate::lab;
use crate::posts::{self, Post};

#[derive(Clone, PartialEq, Debug)]
enum Route {
    Home,
    About,
    Posts,
    Post(String),
    Lab,
    LabSample(String),
}

impl Route {
    /// Routes are clean paths (`/posts/<slug>`); the History API keeps them in sync with the URL.
    fn from_path(path: &str) -> Self {
        match path.trim_matches('/') {
            "about" => Route::About,
            "posts" => Route::Posts,
            "lab" => Route::Lab,
            p => match (p.strip_prefix("posts/"), p.strip_prefix("lab/")) {
                (Some(slug), _) if !slug.is_empty() => Route::Post(slug.to_owned()),
                (_, Some(slug)) if !slug.is_empty() => Route::LabSample(slug.to_owned()),
                _ => Route::Home,
            },
        }
    }

    /// The toolbar item a route belongs to: a post or a lab sample highlights its list's icon.
    fn section(&self) -> &'static str {
        match self {
            Route::Home => "home",
            Route::About => "about",
            Route::Posts | Route::Post(_) => "posts",
            Route::Lab | Route::LabSample(_) => "lab",
        }
    }

    fn to_path(&self) -> String {
        match self {
            Route::Home => "/".into(),
            Route::About => "/about".into(),
            Route::Posts => "/posts".into(),
            Route::Post(slug) => format!("/posts/{slug}"),
            Route::Lab => "/lab".into(),
            Route::LabSample(slug) => format!("/lab/{slug}"),
        }
    }
}

pub struct App {
    route: Route,
    posts: Vec<Post>,
    markdown: CommonMarkCache,
    // Built on first visit: the demos are only paid for when opened.
    egui_map: Option<egui_map::map::Map>,
    home_height: f32,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        crate::theme::apply(&cc.egui_ctx);
        // The markdown viewer treats root-relative image paths as `file://` URLs,
        // so make them absolute against the site origin to reach the http loader.
        let origin = location_origin();
        let mut posts = posts::all();
        for post in &mut posts {
            post.body = post
                .body
                .replace("](/images/", &format!("]({origin}/images/"));
        }
        Self {
            route: Route::from_path(&location_path()),
            posts,
            markdown: CommonMarkCache::default(),
            egui_map: None,
            home_height: 0.0,
        }
    }

    fn go(&mut self, route: Route) {
        if self.route != route {
            push_location_path(&route.to_path());
            self.route = route;
        }
    }

    fn nav(&mut self, ui: &mut egui::Ui) {
        // Hover/active feedback is background-only: strokes and expansion would nudge the label.
        let widgets = &mut ui.style_mut().visuals.widgets;
        for w in [
            &mut widgets.inactive,
            &mut widgets.hovered,
            &mut widgets.active,
            &mut widgets.open,
        ] {
            w.bg_stroke = egui::Stroke::NONE;
            w.expansion = 0.0;
        }
        const ICON_BOX: f32 = 32.0;
        const GAP: f32 = 8.0;
        let items = [
            (
                egui::include_image!("../static/images/home.png"),
                "Home",
                Route::Home,
            ),
            (
                egui::include_image!("../static/images/about.png"),
                "About",
                Route::About,
            ),
            (
                egui::include_image!("../static/images/posts.png"),
                "Posts",
                Route::Posts,
            ),
            (
                egui::include_image!("../static/images/lab.png"),
                "Lab",
                Route::Lab,
            ),
        ];
        // GitHub link is the 5th item of the centered group.
        let group_width = ICON_BOX * 5.0 + GAP * 4.0;

        let (_, row) = ui.allocate_space(egui::vec2(ui.available_width(), ICON_BOX));

        // Both groups share the row: the theme toggle hugs the right edge while the icons stay centered.
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(row)
                .layout(Layout::right_to_left(Align::Center)),
            |ui| {
                // Matches the visual gap above the icon, which comes from centering it in the 32px row.
                ui.add_space(6.0);
                let dark = ui.ctx().global_style().visuals.dark_mode;
                let (icon, next) = if dark {
                    ("☀", ThemePreference::Light)
                } else {
                    ("🌙", ThemePreference::Dark)
                };
                let toggle = ui
                    .add(egui::Button::new(egui::RichText::new(icon).size(18.0)).frame(false))
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .on_hover_text("Toggle light/dark theme");
                if toggle.clicked() {
                    ui.ctx().set_theme(next);
                }
            },
        );

        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(row)
                .layout(Layout::left_to_right(Align::Center)),
            |ui| {
                ui.spacing_mut().item_spacing.x = GAP;
                ui.add_space(((row.width() - group_width) / 2.0).max(0.0));
                for (source, tooltip, route) in items {
                    let active = route.section() == self.route.section();
                    if icon_button(ui, source, tooltip, active).clicked() {
                        self.go(route);
                    }
                }
                let github = icon_button(
                    ui,
                    egui::include_image!("../static/images/github.png"),
                    "github.com/dxps",
                    false,
                );
                if github.clicked() {
                    ui.ctx()
                        .open_url(egui::OpenUrl::new_tab("https://github.com/dxps"));
                }
            },
        );
    }

    fn home(&mut self, ui: &mut egui::Ui) {
        // Centered by padding with half the free height; the block's own height is
        // measured on the previous frame since it isn't known before layout.
        ui.add_space(((ui.available_height() - self.home_height) / 2.0).max(0.0));
        let block = ui.vertical_centered(|ui| {
            ui.add(
                egui::Image::new(egui::include_image!("../static/images/dxps_logo.png"))
                    .max_width(144.0),
            );
            ui.add_space(12.0);
            ui.heading("dxps");
            ui.label("development experiences");
        });
        let height = block.response.rect.height();
        if (height - self.home_height).abs() > 0.5 {
            self.home_height = height;
            ui.ctx().request_repaint();
        }
    }

    fn about(&mut self, ui: &mut egui::Ui) {
        CommonMarkViewer::new().show(ui, &mut self.markdown, posts::ABOUT);
    }

    fn posts_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("posts");
        ui.separator();
        let mut target = None;
        for post in &self.posts {
            ui.horizontal(|ui| {
                ui.monospace(&post.date);
                if ui.link(&post.title).clicked() {
                    target = Some(Route::Post(post.slug.to_owned()));
                }
            });
        }
        if let Some(route) = target {
            self.go(route);
        }
    }

    fn lab_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("lab");
        ui.label("Small implementation examples and demos.");
        ui.separator();
        let mut target = None;
        for sample in lab::SAMPLES {
            if lab_entry(ui, sample).clicked() {
                target = Some(Route::LabSample(sample.slug.to_owned()));
            }
        }
        if let Some(route) = target {
            self.go(route);
        }
    }

    fn lab_sample(&mut self, ui: &mut egui::Ui, slug: &str) {
        let Some(sample) = lab::find(slug) else {
            ui.heading("Not found");
            ui.label("There is no such sample.");
            return;
        };
        ui.horizontal(|ui| {
            ui.heading(sample.title);
            ui.hyperlink_to("source", sample.source_url);
        });
        ui.label(sample.description);
        ui.separator();
        match sample.slug {
            "egui-map" => {
                let map = self.egui_map.get_or_insert_with(lab::egui_map_demo);
                ui.add(map);
            }
            _ => {}
        }
    }

    fn post(&mut self, ui: &mut egui::Ui, slug: &str) {
        let Some(post) = self.posts.iter().find(|p| p.slug == slug) else {
            ui.heading("Not found");
            ui.label("There is no such post.");
            return;
        };
        ui.heading(&post.title);
        ui.monospace(&post.date);
        ui.separator();
        CommonMarkViewer::new().show(ui, &mut self.markdown, &post.body);
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Follow browser back/forward navigation.
        let from_url = Route::from_path(&location_path());
        if from_url != self.route {
            self.route = from_url;
        }

        egui::Panel::top("nav")
            .show_separator_line(false)
            .show(ui, |ui| {
                ui.add_space(6.0);
                self.nav(ui);
                ui.add_space(6.0);
            });

        egui::CentralPanel::default().show(ui, |ui| {
            if self.route == Route::Home {
                // No scrolling here, so the panel height is known and the content can be centered in it.
                self.home(ui);
                return;
            }
            // The map owns drag and wheel input, so it can't live inside a scroll area.
            if let Route::LabSample(slug) = self.route.clone() {
                self.lab_sample(ui, &slug);
                return;
            }
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    // Reading column: keeps line length comfortable on wide screens.
                    let avail = ui.available_rect_before_wrap();
                    let width = avail.width().min(760.0);
                    let column = egui::Rect::from_x_y_ranges(
                        (avail.center().x - width / 2.0)..=(avail.center().x + width / 2.0),
                        // Finite height: a scroll area reports an unbounded one.
                        avail.top()..=avail.top() + ui.ctx().content_rect().height(),
                    );
                    ui.scope_builder(egui::UiBuilder::new().max_rect(column), |ui| {
                        match self.route.clone() {
                            Route::Home => {}
                            Route::About => self.about(ui),
                            Route::Posts => self.posts_list(ui),
                            Route::Post(slug) => self.post(ui, &slug),
                            Route::Lab => self.lab_list(ui),
                            Route::LabSample(_) => {}
                        }
                    });
                });
        });
    }
}

#[cfg(target_arch = "wasm32")]
fn location_path() -> String {
    web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
fn location_origin() -> String {
    web_sys::window()
        .and_then(|w| w.location().origin().ok())
        .unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
fn push_location_path(path: &str) {
    if let Some(history) = web_sys::window().and_then(|w| w.history().ok()) {
        let _ = history.push_state_with_url(&eframe::wasm_bindgen::JsValue::NULL, "", Some(path));
    }
}

// Native builds (local dev only) keep the route in memory.
#[cfg(not(target_arch = "wasm32"))]
fn location_path() -> String {
    String::new()
}

#[cfg(not(target_arch = "wasm32"))]
fn location_origin() -> String {
    "http://localhost:8080".into()
}

#[cfg(not(target_arch = "wasm32"))]
fn push_location_path(_path: &str) {}

/// A whole-row clickable entry. The hover fill is the toolbar buttons' one; it is painted
/// behind the text, into a slot reserved before the content's size is known.
fn lab_entry(ui: &mut egui::Ui, sample: &lab::Sample) -> egui::Response {
    const PAD: f32 = 8.0;
    let background = ui.painter().add(egui::Shape::Noop);
    let content = ui.scope(|ui| {
        ui.set_width(ui.available_width());
        ui.add_space(PAD);
        ui.horizontal(|ui| {
            ui.add_space(PAD);
            ui.vertical(|ui| {
                // Non-selectable labels, otherwise they would swallow the click.
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(sample.title).color(ui.visuals().hyperlink_color),
                    )
                    .selectable(false),
                );
                ui.add(egui::Label::new(sample.description).selectable(false));
            });
        });
        ui.add_space(PAD);
    });
    let rect = content.response.rect;
    let response = ui
        .interact(rect, ui.id().with(sample.slug), egui::Sense::click())
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    if response.hovered() {
        ui.painter().set(
            background,
            egui::Shape::rect_filled(rect, 6.0, ui.visuals().widgets.hovered.bg_fill),
        );
    }
    response
}

/// Square, borderless icon button. The SVGs are white so the tint alone decides their color.
fn icon_button(
    ui: &mut egui::Ui,
    source: egui::ImageSource<'static>,
    tooltip: &str,
    active: bool,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(32.0, 32.0), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let visuals = ui.visuals();
        // The active item is marked by its tint only; background is reserved for hover.
        let fill = if response.hovered() {
            visuals.widgets.hovered.bg_fill
        } else {
            egui::Color32::TRANSPARENT
        };
        let tint = if active || response.hovered() {
            visuals.hyperlink_color
        } else {
            visuals.text_color()
        };
        ui.painter().rect_filled(rect, 6.0, fill);
        egui::Image::new(source)
            .tint(tint)
            .paint_at(ui, rect.shrink(6.0));
    }
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(tooltip)
}
