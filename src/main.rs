use widget::{
    application::Application,
    global_hotkey::hotkey::{Code, HotKey, Modifiers},
    window::win_strategy::{CloseStrategy, MinimizeStrategy, WindowStrategy},
};

fn main() {
    let mut app = Application::default();

    let mut w = app.make_window("Test", 800, 600);
    let hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::Space);

    w.set_strategy("close_requested".to_string(), |win| {
        win.normalize();
        win.hide();
        WindowStrategy::Close(CloseStrategy::Ignore)
    });

    w.set_strategy("minimize".to_string(), |win| {
        win.hide();
        WindowStrategy::Minimize(MinimizeStrategy::Ignore)
    });

    w.set_slot("hotkey".to_string(), move |win, key| {
        if *key.downcast::<u32>().unwrap() == hotkey.id {
            win.show();
            win.normalize();
        }
    });

    let _ = w.get_win_mut().reg_hotkey(hotkey);

    let ww = app.make_window("2", 500, 600);

    app.reg_win(w);
    app.reg_win(ww);
    app.run();
}
