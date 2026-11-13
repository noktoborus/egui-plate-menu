//! Every kind of menu of egui-plate-menu, in a browser or a window: a panel
//! of switches says what the menu holds and how it behaves, and a button
//! opens it so.

mod demo;
mod entries;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    eframe::run_native(
        "egui-plate-menu",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(demo::Demo::default()))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast;

    eframe::WebLogger::init(log::LevelFilter::Info).ok();
    wasm_bindgen_futures::spawn_local(async {
        let canvas = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.get_element_by_id("demo"))
            .and_then(|element| element.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            .expect("index.html has the canvas");
        let started = eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|_| Ok(Box::new(demo::Demo::default()))),
            )
            .await;
        if let Err(error) = started {
            log::error!("the demo failed to start: {error:?}");
        }
    });
}
