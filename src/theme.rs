//! GitHub "Dark Dimmed" and "Light" palettes (from Primer), mapped onto egui visuals.

use eframe::egui::{
    Color32, Context, Stroke, Theme, Visuals, epaint::Shadow, style::WidgetVisuals,
};

struct Palette {
    canvas: Color32,
    subtle: Color32,
    inset: Color32,
    border: Color32,
    border_muted: Color32,
    fg: Color32,
    fg_muted: Color32,
    accent: Color32,
    selection: Color32,
    hover: Color32,
    active: Color32,
    shadow_alpha: u8,
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

const DARK_DIMMED: Palette = Palette {
    canvas: rgb(0x22272e),
    subtle: rgb(0x2d333b),
    inset: rgb(0x1c2128),
    border: rgb(0x444c56),
    border_muted: rgb(0x373e47),
    fg: rgb(0xadbac7),
    fg_muted: rgb(0x768390),
    accent: rgb(0x539bf5),
    selection: rgb(0x264466),
    hover: rgb(0x373e47),
    active: rgb(0x444c56),
    shadow_alpha: 140,
};

const LIGHT: Palette = Palette {
    canvas: rgb(0xffffff),
    subtle: rgb(0xf6f8fa),
    inset: rgb(0xeff2f5),
    border: rgb(0xd0d7de),
    border_muted: rgb(0xd8dee4),
    fg: rgb(0x1f2328),
    fg_muted: rgb(0x59636e),
    accent: rgb(0x0969da),
    selection: rgb(0xb6e3ff),
    hover: rgb(0xeaeef2),
    active: rgb(0xd0d7de),
    shadow_alpha: 62,
};

pub fn apply(ctx: &Context) {
    ctx.set_visuals_of(Theme::Dark, visuals(Visuals::dark(), &DARK_DIMMED));
    ctx.set_visuals_of(Theme::Light, visuals(Visuals::light(), &LIGHT));
    // egui waits 0.5 s by default before showing tooltips, which feels sluggish on icon-only buttons.
    ctx.all_styles_mut(|style| style.interaction.tooltip_delay = 0.15);
}

fn visuals(mut v: Visuals, p: &Palette) -> Visuals {
    v.panel_fill = p.canvas;
    v.window_fill = p.canvas;
    // Tooltips and popups: no outline, and a centered (zero-offset) shadow so it spreads evenly on all sides.
    v.window_stroke = Stroke::NONE;
    v.popup_shadow = Shadow {
        offset: [0, 0],
        blur: 22,
        spread: 2,
        color: Color32::from_black_alpha(p.shadow_alpha),
    };
    v.window_shadow = v.popup_shadow;
    v.extreme_bg_color = p.inset;
    v.faint_bg_color = p.subtle;
    v.code_bg_color = p.subtle;
    v.hyperlink_color = p.accent;
    v.warn_fg_color = rgb(0xc69026);
    v.error_fg_color = rgb(0xe5534b);
    v.selection.bg_fill = p.selection;
    v.selection.stroke = Stroke::new(1.0, p.accent);

    let widget = |w: &mut WidgetVisuals, fill, fg, stroke| {
        w.bg_fill = fill;
        w.weak_bg_fill = fill;
        w.bg_stroke = Stroke::new(1.0, stroke);
        w.fg_stroke = Stroke::new(1.0, fg);
    };
    widget(
        &mut v.widgets.noninteractive,
        p.canvas,
        p.fg,
        p.border_muted,
    );
    widget(&mut v.widgets.inactive, p.subtle, p.fg, p.border);
    widget(&mut v.widgets.hovered, p.hover, p.fg, p.fg_muted);
    widget(&mut v.widgets.active, p.active, p.fg, p.accent);
    widget(&mut v.widgets.open, p.subtle, p.fg, p.border);
    v
}
