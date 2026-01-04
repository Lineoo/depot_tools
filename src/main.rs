use widget::{
    application::Application,
    control::{
        Control,
        input_box::InputBox,
        list_widget::ListWidget,
        vbox::{InsertPosition, VBox},
    },
};

fn main_old() {
    let mut app = Application::new();

    let kcore = backend::KitCore::new();

    let mut w = app.make_window("Test", 410, 40);
    w.no_decorations();
    let hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::Space);

    let input_util = app.input_util.clone();

    w.get_win_mut().set_userdata((String::new(), 0usize));

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
        if let Some(key) = key
            && *key.downcast::<u32>().unwrap() == hotkey.id
        {
            win.show();
            win.normalize();
        }
    });

    w.set_slot("keydown".to_string(), move |win, key| {
        {
            let window = win.cvs.window().clone();
            let input_util = input_util.borrow_mut();
            if !input_util.is_active(&window) {
                input_util.start(&window);
                input_util.set_rect(&window, Rect::new(7, 7, 396, 26).try_into().unwrap(), 10);
            }
        }
        let key = *key.unwrap().downcast::<Keycode>().unwrap();
        match key {
            Keycode::Return => {
                let selected_idx = 0;

                let mut kcore = kcore.lock();
                if kcore.stack().call(selected_idx) == depot_core::stack::StackCall::Exit {
                    println!("Exit!");
                    win.hide();
                } else {
                    redraw(&mut kcore);
                }
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

                let mut kcore = kcore.lock();
                kcore.stack().write(userdata.0.clone());
                redraw(&mut kcore);
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
            Keycode::Up => {
                let mut kcore = kcore.lock();
                kcore.selector_up();
            }
            Keycode::Down => {
                let mut kcore = kcore.lock();
                kcore.selector_down();
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

                let mut kcore = kcore.lock();
                kcore.stack().write(userdata.0.clone());
                redraw(&mut kcore);
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

fn main() {
    let mut app = Application::new();
    let mut w = app.make_window("Test", 800, 600);
    w.no_decorations();

    let ib = InputBox::create(app.ctrl_ctx().clone());
    let lw = ListWidget::create(app.ctrl_ctx().clone());
    let vb = VBox::create(app.ctrl_ctx().clone());

    let lw2 = lw.clone();
    (&mut *ib.borrow_mut() as &mut dyn Control)
        .connect(InputBox::SIGNAL_TEXT_CHANGED, move |text: String| {
            let mut lw = lw2.borrow_mut();
            lw.item_list_mut().clear();
            for i in 0..text.len() {
                lw.insert_item(i.to_string(), None);
            }
            // logic here
        })
        .unwrap();

    lw.borrow_mut().insert_item("hello".to_string(), None);
    lw.borrow_mut().insert_item("hello --2".to_string(), None);

    vb.borrow_mut()
        .add(ib.downgrade().untyped(), false, InsertPosition::First);
    vb.borrow_mut()
        .add(lw.downgrade().untyped(), true, InsertPosition::Last);

    w.set_child(vb.downgrade().untyped());

    app.reg_win(w);
    app.run();
}
