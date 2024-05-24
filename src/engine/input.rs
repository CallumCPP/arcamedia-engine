use std::collections::HashMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

static mut INPUT: Option<Box<Input>> = None;

pub struct Input {
    key_map: HashMap<String, bool>,
    key_pressed_map: HashMap<String, bool>,
}

impl Input {
    pub fn init() {
        let key_map: HashMap<String, bool> = HashMap::new();
        let key_pressed_map: HashMap<String, bool> = HashMap::new();

        unsafe {
            INPUT = Some(Box::new(Self {
                key_map,
                key_pressed_map,
            }));
        }

        input()._init();
    }

    fn _init(&mut self) {
        let document = web_sys::window().unwrap().document().unwrap();

        let keydown_closure = Closure::new(Box::new(move |event: KeyboardEvent| {
            match input().key_map.get(&event.code()) {
                None => {
                    input().key_pressed_map.insert(event.code(), true);
                }
                Some(down) => {
                    if !down {
                        input().key_pressed_map.insert(event.code(), true);
                    }
                }
            }

            input().key_map.insert(event.code(), true);
        }) as Box<dyn FnMut(_)>);

        document
            .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
            .expect("Should be able to add listener.");

        keydown_closure.forget();

        let keyup_closure = Closure::new(Box::new(move |event: KeyboardEvent| {
            input().key_map.insert(event.code(), false);
        }) as Box<dyn FnMut(_)>);

        document
            .add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())
            .expect("Should be able to add listener.");

        keyup_closure.forget();
    }

    pub fn flush_pressed_map(&mut self) {
        self.key_pressed_map.clear();
    }

    pub fn get_key_down(&self, code: &str) -> bool {
        match self.key_map.get(code) {
            None => false,
            Some(pressed) => *pressed,
        }
    }

    pub fn key_was_pressed(&self, code: &str) -> bool {
        match self.key_pressed_map.get(code) {
            None => false,
            Some(pressed) => *pressed,
        }
    }
}

pub fn input() -> &'static mut Input {
    unsafe { INPUT.as_deref_mut().expect("Input should be initialized") }
}
