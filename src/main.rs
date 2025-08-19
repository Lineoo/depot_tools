use widget::{
    Keycode,
    application::Application,
    global_hotkey::hotkey::{Code, HotKey, Modifiers},
    window::win_strategy::{CloseStrategy, MinimizeStrategy, WindowStrategy},
};

fn main() {
    let mut app = Application::default();

    let mut w = app.make_window("Test", 410, 40);
    let hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::Space);

    w.get_win_mut().set_userdata((String::new(), 0 as usize));

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

    w.set_slot("keydown".to_string(), |win, key| {
        let key = key.downcast::<Option<Keycode>>().unwrap().unwrap();
        match key {
            Keycode::Return => {
                todo!("Call commands here")
            }
            Keycode::Escape => {
                win.hide();
            }
            Keycode::Backspace => {
                let userdata = win.get_userdata_mut::<(String, usize)>().unwrap();
                if userdata.1 > 0 {
                    userdata.0.remove(userdata.1 - 1);
                    userdata.1 -= 1;
                }
            }
            Keycode::Left | Keycode::Right => {
                let userdata = win.get_userdata_mut::<(String, usize)>().unwrap();
                let cursor = &mut userdata.1;
                if key == Keycode::Left && *cursor > 0 {
                    *cursor -= 1;
                } else if key == Keycode::Right && *cursor < userdata.0.len() {
                    *cursor += 1;
                }
            }
            code if is_char(code) => {
                let c = if code == Keycode::Space {
                    ' '.to_string()
                } else {
                    code.name().to_lowercase()
                };
                let userdata = win.get_userdata_mut::<(String, usize)>().unwrap();
                userdata.0.insert_str(userdata.1, c.as_str());
                userdata.1 += 1;
            }
            _ => {}
        }
        println!("{}", win.get_userdata_mut::<(String, usize)>().unwrap().0)
    });

    let _ = w.get_win_mut().reg_hotkey(hotkey);

    app.reg_win(w);
    app.run();
}

fn is_char(code: Keycode) -> bool {
    let code = code.to_ll();
    code >= Keycode::A.to_ll() && code <= Keycode::Z.to_ll()
        || code >= Keycode::_0.to_ll() && code <= Keycode::_9.to_ll()
        || code == Keycode::Space.to_ll()
}
