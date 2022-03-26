use piston_window as pw;
use std::collections::HashSet;
pub use piston2d_abstraction_proc_macros::derive_input_consumer;

macro_rules! create_consumer_trait {
	// syntax: Name consumes arg1: type1, arg2: type2, etc; fn Name(args) -> Ret {_code_}, fn Name2(args) {_code_}
	// automatically implement consume and accepts
	($consumer_name:ident consumes $($arg_name:ident: $arg_ty:ty),*; $(fn $fn_name:ident($($fn_arg_name:ident: $fn_arg_ty:ty),*) $(-> $fn_ret_ty:ty)? $fn_def:block),*) => {
		pub trait $consumer_name {
			fn consume(&mut self, $($arg_name: $arg_ty),*) {unimplemented!("Implement {}!", stringify!($consumer_name))}
			fn accepts(&self) -> bool;
			$(fn $fn_name($($fn_arg_name: $fn_arg_ty),*) -> $($fn_ret_ty)? $fn_def)*
		}
	}
}

pub trait InputConsumer:
    AnyButtonConsumer
    + ButtonConsumer
    + HotkeyConsumer
    + CursorPositionConsumer
    + CursorMotionConsumer
    + ScrollConsumer
    + ResizeConsumer
    + FocusConsumer
    + CursorInWindowConsumer
    + CloseConsumer
{
}

pub struct InputHandler {
    pub pressed_buttons: HashSet<pw::Button>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            pressed_buttons: HashSet::new(),
        }
    }

    pub fn consume(&mut self, event: pw::Input, consumers: &mut [&mut dyn InputConsumer]) {
        use pw::Input::*;
        for con in consumers {
            match event {
                Button(a) => {
                    if let pw::ButtonState::Press = a.state {
                        self.pressed_buttons.insert(a.button);
                    } else {
                        self.pressed_buttons.remove(&a.button);
                    }
                    if AnyButtonConsumer::accepts(*con) {
                        AnyButtonConsumer::consume(*con, a.button, a.state);
                    }
                    if ButtonConsumer::accepts(*con) {
                        if let Some(b) = ButtonConsumer::get_button(*con) {
                            if b == a.button {
                                ButtonConsumer::consume(*con, a.state);
                            }
                        }
                    }
                    if HotkeyConsumer::accepts(*con) {
                        let keys = HotkeyConsumer::get_hotkeys(*con);
                        if *keys == self.pressed_buttons && keys.contains(&a.button) {
                            HotkeyConsumer::consume(*con);
                        }
                    }
                }
                Move(m) => {
                    use pw::Motion::*;
                    match m {
                        MouseCursor(p) => {
                            if CursorPositionConsumer::accepts(*con) {
                                CursorPositionConsumer::consume(*con, p);
                            }
                        }
                        MouseRelative(p) => {
                            if CursorMotionConsumer::accepts(*con) {
                                CursorMotionConsumer::consume(*con, p);
                            }
                        }
                        MouseScroll(p) => {
                            if ScrollConsumer::accepts(*con) {
                                ScrollConsumer::consume(*con, p);
                            }
                        }
                        // ControllerAxis(_) => todo!(),
                        // Touch(_) => todo!(),
                        _ => {}
                    }
                }
                Resize(a) => {
                    if ResizeConsumer::accepts(*con) {
                        ResizeConsumer::consume(*con, a);
                    }
                }
                Focus(b) => {
                    if FocusConsumer::accepts(*con) {
                        FocusConsumer::consume(*con, b);
                    }
                }
                Cursor(b) => {
                    if CursorInWindowConsumer::accepts(*con) {
                        CursorInWindowConsumer::consume(*con, b);
                    }
                }
                // Text(_) => todo!(),
                // FileDrag(_) => todo!(),
                Close(a) => {
                    if CloseConsumer::accepts(*con) {
                        CloseConsumer::consume(*con, a);
                    }
                }

                _ => {}
            }
        }
        // use pw::Input::*;
        // match event {
        // 	Button(b) => self.button_handler.consume(b),
        // 	Move(m) => self.motion_handler.consume(m),
        // 	Resize(a) => self.resize_handlers.call(&a),
        // 	Focus(t) => self.focus_handlers.call(&t),
        // 	Cursor(t) => self.cursor_handlers.call(&t),
        // 	Close(a) => return Some(a),
        // 	_ => {}
        // }
    }
}

create_consumer_trait!(ResizeConsumer consumes args: pw::ResizeArgs;);

create_consumer_trait!(FocusConsumer consumes focus:bool;);

create_consumer_trait!(CursorInWindowConsumer consumes cursor_in_window: bool;);

create_consumer_trait!(CloseConsumer consumes args: pw::CloseArgs;);

create_consumer_trait!(AnyButtonConsumer consumes button: pw::Button, state: pw::ButtonState;);

create_consumer_trait!(ButtonConsumer consumes state: pw::ButtonState; fn get_button(self: &Self) -> Option<pw::Button> {None});

create_consumer_trait!(HotkeyConsumer consumes; fn get_hotkeys(self: &Self) -> &HashSet<pw::Button> {panic!("Implement HotkeyConsumer first!")});

create_consumer_trait!(CursorPositionConsumer consumes pos: [f64; 2];);

create_consumer_trait!(CursorMotionConsumer consumes change: [f64; 2];);

create_consumer_trait!(ScrollConsumer consumes change: [f64; 2];);