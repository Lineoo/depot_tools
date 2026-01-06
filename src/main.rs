use depot_core::stack::Stack;
use widget::{
    Keycode,
    application::Application,
    control::{
        input_box::InputBox,
        list_widget::ListWidget,
        vbox::{InsertPosition, VBox},
    },
};

fn main() {
    let mut app = Application::new();
    let mut w = app.make_window("Test", 800, 600);
    w.no_decorations();

    let _stack = std::rc::Rc::new(parking_lot::Mutex::new(Stack::new(Box::new("Depot KIT"))));

    let ib = InputBox::create(app.ctrl_ctx().clone(), true);
    let lw = ListWidget::create(app.ctrl_ctx().clone());
    let vb = VBox::create(app.ctrl_ctx().clone());

    let ibr = ib.untyped();

    let lwo = lw.clone();
    let lwo2 = lw.clone();
    let stack = _stack.clone();
    ibr.borrow_mut()
        .connect(InputBox::SIGNAL_TEXT_CHANGED, move |text: String| {
            let mut lw = lwo.borrow_mut();
            lw.item_list_mut().clear();

            let mut stack = stack.lock();
            stack.write(text);
            for read in stack.iter() {
                lw.insert_item(format!("{}:  {}", read.title, read.description), None);
            }
        })
        .unwrap();
    ibr.borrow_mut()
        .connect(
            InputBox::SIGNAL_IMPORTANT_KEY_PRESSED,
            move |key: Keycode| {
                let mut lw = lwo2.borrow_mut();
                if lw.selected_item().is_none() {
                    lw.select_item(0);
                } else {
                    match key {
                        Keycode::Up => {
                            lw.select_up(true);
                        }
                        Keycode::Down => {
                            lw.select_down(true);
                        }
                        _ => {}
                    }
                }
            },
        )
        .unwrap();

    let stack = _stack.clone();
    ibr.borrow_mut()
        .connect(InputBox::SIGNAL_SUBMIT, move |_: String| {
            let mut stack = stack.lock();
            stack.call(0);
            println!("submitted!");
        })
        .unwrap();

    lw.borrow_mut().insert_item("hello".to_string(), None);
    lw.borrow_mut().insert_item("hello --2".to_string(), None);
    lw.borrow_mut().insert_item("hello --3".to_string(), None);
    lw.borrow_mut().select_item(0);

    vb.borrow_mut()
        .add(ib.downgrade().into_untyped(), false, InsertPosition::First);
    vb.borrow_mut()
        .add(lw.downgrade().into_untyped(), true, InsertPosition::Last);

    w.set_child(vb.downgrade().into_untyped());

    app.reg_win(w);
    app.run();
}
