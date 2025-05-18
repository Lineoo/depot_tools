use log::info;
use widget::application::Application;

fn main() {
    // static mut W: Option<Arc<Window>> = None;
    env_logger::init();

    let mut app = Application::new();
    app.on_init(|ctx| {
        let w = ctx.create_window("Hello".to_string());
        w.show(true);
        ctx.app.reg_win(w);
    });
    app.enter_event_loop();

    println!("depot main");
    info!("hi there");
}
