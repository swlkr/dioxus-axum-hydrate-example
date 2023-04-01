use dioxus_web::Config;
use frontend::app::app;

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus_web::launch_with_props(app, (), Config::new().hydrate(true));
}
