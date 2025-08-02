use widget::{
    application::Application,
    global_hotkey::hotkey::{Code, HotKey, Modifiers},
    window::win_strategy::{CloseStrategy, GlobalHotKeyStrategy, WindowStrategy},
};

fn main() {
    let mut app = Application::default();

    let mut w = app.make_window("Test", 800, 600);

    w.set_strategy("close_requested".to_string(), |win| {
        win.hide();
        WindowStrategy::Close(CloseStrategy::Ignore)
    });

    w.set_strategy("hotkey".to_string(), |win| {
        win.show();
        WindowStrategy::Hotkey(GlobalHotKeyStrategy::StopSpread)
    });

    let _ = w
        .get_win_mut()
        .reg_hotkey(HotKey::new(Some(Modifiers::CONTROL), Code::Space));

    app.reg_win(w);
    app.run();
}
