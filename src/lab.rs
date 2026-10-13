//! The lab: small implementation examples and demos, each one a self-contained egui view.

use egui_map::map::Map;
use egui_map::map::objects::{MapPoint, MapSegment, VisibilitySetting};

pub struct Sample {
    pub slug: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub source_url: &'static str,
}

pub const SAMPLES: &[Sample] = &[Sample {
    slug: "egui-map",
    title: "egui-map",
    description: "An interactive 2D map widget: pan by dragging, zoom with the mouse wheel.",
    source_url: "https://github.com/rafaga/egui-map",
}];

pub fn find(slug: &str) -> Option<&'static Sample> {
    SAMPLES.iter().find(|s| s.slug == slug)
}

/// Same five nodes and four lines as the library's first tutorial.
pub fn egui_map_demo() -> Map {
    let mut points: Vec<MapPoint> = [
        (1, "Alpha", 0.0, 0.0),
        (2, "Beta", 120.0, 40.0),
        (3, "Gamma", 60.0, -90.0),
        (4, "Delta", -110.0, -30.0),
        (5, "Epsilon", -40.0, 100.0),
    ]
    .into_iter()
    .map(|(id, name, x, y)| {
        let mut point = MapPoint::new(id, [x, y]);
        point.set_name(name.to_string());
        point
    })
    .collect();

    // A line id must be listed in the `connections` of both of its nodes.
    let mut segments = Vec::new();
    for line_id in [(1, 2), (1, 3), (1, 4), (4, 5)] {
        let mut ends = [[0.0; 2]; 2];
        for (end, node_id) in [line_id.0, line_id.1].into_iter().enumerate() {
            let point = points
                .iter_mut()
                .find(|p| p.get_id() == node_id)
                .expect("every line joins two of the nodes above");
            point.connections.push(line_id);
            ends[end] = point.coords;
        }
        segments.push(MapSegment::new(line_id, ends[0], ends[1]));
    }

    let mut map = Map::new();
    map.add_points(points);
    map.add_lines(segments);
    map.settings.node_text_visibility = VisibilitySetting::Hover;
    map.settings.style.line_width = Some(2.0);
    map.settings.minimap.enabled = true;
    map
}
