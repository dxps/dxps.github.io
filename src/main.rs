mod app;
mod lab;
mod posts;
mod theme;

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Without a logger, eframe's panic messages never reach the browser console.
    eframe::WebLogger::init(log::LevelFilter::Error).ok();

    wasm_bindgen_futures::spawn_local(async {
        let canvas = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("app_canvas"))
            .and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            .expect("missing <canvas id=\"app_canvas\">");

        let started = eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
            )
            .await;

        // Replace the loading text with a message if the browser can't run the app.
        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading"))
        {
            match started {
                Ok(()) => el.remove(),
                Err(_) => el
                    .set_inner_html("The app failed to start. Your browser may not support WebGL."),
            }
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    eframe::run_native(
        "dxps",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
