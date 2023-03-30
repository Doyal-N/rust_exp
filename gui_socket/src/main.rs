use fltk::{app, button::Button, prelude::*, window::Window};
use smart_socket::client;
fn main() -> Result<(), FltkError> {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 300, 200, "Manage Socket");
    // let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but_state = Button::new(160, 110, 80, 40, "State");
    let mut but_switch = Button::new(70, 110, 80, 40, "Switch");
    wind.make_resizable(true);

    wind.end();
    wind.show();

    but_state.set_callback(move |_| client::check_state());
    but_switch.set_callback(move |_| client::switch_socket());

    app.run()
}
