
use leptos::*;
use leptos::logging::log;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, closure::Closure};
use js_sys::Object;
use serde::{Serialize,Deserialize};
use serde_wasm_bindgen::from_value;

use super::utils::{convert,set_property};
pub use super::common::{Type,Size};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Confirm,
    Cancel,
    Close,
    #[default]
    None,
}

impl Action {
    pub fn from_str(s: &str) -> Self {
        match s {
            "confirm" => Action::Confirm,
            "cancel" => Action::Cancel,
            "close" => Action::Close,
            _ => Action::None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Action::Confirm => "confirm",
            Action::Cancel => "cancel",
            Action::Close => "close",
            Action::None => "none",
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = element_plus, js_name = ElMessageBox)]
    fn el_message_box(msg: &JsValue);
    #[wasm_bindgen(js_namespace = element_plus, js_name = "ElMessageBox.alert")]
    fn el_message_box_alert(msg: &JsValue, title: &JsValue, options: &JsValue);
    #[wasm_bindgen(js_namespace = element_plus, js_name = "ElMessageBox.confirm")]
    fn el_message_box_confirm(msg: &JsValue, title: &JsValue, options: &JsValue)->js_sys::Promise;
    #[wasm_bindgen(js_namespace = element_plus, js_name = "ElMessageBox.prompt")]
    fn el_message_box_prompt(msg: &JsValue, title: &JsValue, options: &JsValue)->js_sys::Promise;
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default)]
pub struct MessageBox{
    js_object: Object,
    has_content: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PromptValue{
    pub value: Option<String>,
    pub action: String,
}

impl MessageBox {
    pub fn new() -> Self {
        self::MessageBox{
            js_object: Object::new(),
            has_content: false,
        }
    }

    pub fn autofocus(&mut self, v: bool) -> &mut Self {
        set_property(&self.js_object, "autofocus", v);
        self
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        set_property(&self.js_object, "title", title);
        self
    }

    pub fn message(&mut self, msg: String) -> &mut Self {
        set_property(&self.js_object, "message", msg);
        self.has_content = true;
        self
    }

    pub fn message_element(&mut self, v: View) -> &mut Self {
        if let View::Element(v) = v {
            set_property(&self.js_object, "message", convert(v.into_html_element().into_any().get_root_node()));
            self.has_content = true;
        }else{
            log!("message_element must be View::Element");
        }
        self
    }

    pub fn message_function<F>(&mut self, f: F) -> &mut Self
        where F: Fn()->View + 'static {
        let closure: Closure<dyn FnMut()->JsValue> = Closure::new(move||{
            let view = f();
            if let View::Element(v) = view {
                return convert(v.into_html_element().into_any().get_root_node());
            }else{
                log!("message_function must return View::Element");
                return JsValue::UNDEFINED.into();
            }
        });
        js_sys::Reflect::set(&self.js_object, &JsValue::from_str("message"), &closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
        self.has_content = true;
        self
    }

    pub fn callback<F>(&mut self, f: F) -> &mut Self
        where F: Fn(Option<String>,Action) + 'static {
        let closure: Closure<dyn FnMut(JsValue)> = Closure::new(move |value: JsValue|{
            log!("callback value: {:?}", value);
            if Object::instanceof(&value) {
                if let Ok(prompt_value) = from_value::<PromptValue>(value) {
                    f(prompt_value.value,Action::from_str(&prompt_value.action));
                }else{
                    log!("invalid prompt_value");
                }
            }else{
                if let Some(action) = value.as_string() {
                    f(None,Action::from_str(&action));
                }else{
                    log!("invalid action");
                }
            }
        });
        js_sys::Reflect::set(&self.js_object, &JsValue::from_str("callback"), &closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
        self
    }

    pub fn before_close<F>(&mut self, f: F) -> &mut Self
        where F: Fn(Action) + 'static {
        let closure: Closure<dyn FnMut(JsValue,JsValue,js_sys::Function)> = Closure::new(move |value: JsValue, _instance: JsValue, done: js_sys::Function|{
            if let Some(action) = value.as_string() {
                f(Action::from_str(&action));
            }else{
                log!("before_close invalid action");
            }
            let _ = done.call0(&JsValue::NULL).unwrap_or(JsValue::UNDEFINED);
        });
        js_sys::Reflect::set(&self.js_object, &JsValue::from_str("beforeClose"), &closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
        self
    }

    pub fn message_type(&mut self, t: Type) -> &mut Self {
        set_property(&self.js_object, "type", t.as_str());
        self
    }

    pub fn custom_class(&mut self, c: String) -> &mut Self {
        set_property(&self.js_object, "customClass", c);
        self
    }

    pub fn custom_style(&mut self, c: String) -> &mut Self {
        set_property(&self.js_object, "customStyle", c);
        self
    }

    pub fn show_close(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "showClose", b);
        self
    }

    pub fn center(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "center", b);
        self
    }

    pub fn append_to(&mut self, a: String) -> &mut Self {
        set_property(&self.js_object, "appendTo", a);
        self
    }

    pub fn dangerously_use_html_string(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "dangerouslyUseHTMLString", b);
        self
    }

    pub fn distinguish_cancel_and_close(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "distinguishCancelAndClose", b);
        self
    }

    pub fn lock_scroll(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "lockScroll", b);
        self
    }

    pub fn show_cancel_button(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "showCancelButton", b);
        self
    }

    pub fn show_confirm_button(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "showConfirmButton", b);
        self
    }

    pub fn cancel_button_text(&mut self, t: String) -> &mut Self {
        set_property(&self.js_object, "cancelButtonText", t);
        self
    }

    pub fn confirm_button_text(&mut self, t: String) -> &mut Self {
        set_property(&self.js_object, "confirmButtonText", t);
        self
    }

    pub fn cancel_button_class(&mut self, c: String) -> &mut Self {
        set_property(&self.js_object, "cancelButtonClass", c);
        self
    }

    pub fn confirm_button_class(&mut self, c: String) -> &mut Self {
        set_property(&self.js_object, "confirmButtonClass", c);
        self
    }

    pub fn close_on_click_modal(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "closeOnClickModal", b);
        self
    }

    pub fn close_on_press_escape(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "closeOnPressEscape", b);
        self
    }

    pub fn close_on_hash_change(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "closeOnHashChange", b);
        self
    }

    pub fn show_input(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "showInput", b);
        self
    }

    pub fn input_placeholder(&mut self, p: String) -> &mut Self {
        set_property(&self.js_object, "inputPlaceholder", p);
        self
    }

    pub fn input_value(&mut self, v: String) -> &mut Self {
        set_property(&self.js_object, "inputValue", v);
        self
    }

    pub fn input_type(&mut self, t: String) -> &mut Self {
        set_property(&self.js_object, "inputType", t);
        self
    }

    pub fn input_pattern(&mut self, p: String) -> &mut Self {
        set_property(&self.js_object, "inputPattern", p);
        self
    }

    pub fn input_validator<F>(&mut self, f: F) -> &mut Self
        where F: Fn(String) -> bool + 'static {
        let closure: Closure<dyn FnMut(String)> = Closure::new(move |v: String|{
            f(v);
        });
        js_sys::Reflect::set(&self.js_object, &JsValue::from_str("inputValidator"), &closure.as_ref().unchecked_ref()).unwrap();
        self
    }

    pub fn input_error_message(&mut self, m: String) -> &mut Self {
        set_property(&self.js_object, "inputErrorMessage", m);
        self
    }

    pub fn draggble(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "draggable", b);
        self
    }

    pub fn round_button(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "roundButton", b);
        self
    }

    pub fn button_size(&mut self, s: Size) -> &mut Self {
        set_property(&self.js_object, "buttonSize", s.as_str());
        self
    }

    pub fn show(&mut self) {
        if self.has_content {
            el_message_box(&self.js_object);
        }else{
            log!("Message has no content");
        }
    }

    pub fn alert(&mut self, message: &str, title: &str) {
        el_message_box_alert(&JsValue::from_str(message), &JsValue::from_str(title), &self.js_object);
    }

    pub fn confirm(&mut self, message: &str, title: &str){
        let _  = el_message_box_confirm(&JsValue::from_str(message), &JsValue::from_str(title), &self.js_object);
    }

    pub fn prompt(&mut self, message: &str, title: &str){
        let _  = el_message_box_prompt(&JsValue::from_str(message), &JsValue::from_str(title), &self.js_object);
    }

    pub fn confirm_then_catch<T,F>(&mut self, message: &str, title: &str,mut then: Option<T>, mut catch: Option<F>)
        where F: FnMut(Action) + 'static , T: FnMut(Action) + 'static {
        let closure_then = Closure::new(move |value: JsValue|{
            let mut action = Action::None;
            if let Some(value) = value.as_string() {
                action = Action::from_str(&value);
            }
            if let Some(f) = then.as_mut() {
                f(action);
            }
        });
        let closure_catch = Closure::new(move |value: JsValue|{
            let mut action = Action::None;
            if let Some(value) = value.as_string() {
                action = Action::from_str(&value);
            }
            if let Some(f) = catch.as_mut() {
                f(action);
            }
        });
        let _  = el_message_box_confirm(&JsValue::from_str(message), &JsValue::from_str(title), &self.js_object).then(&closure_then).catch(&closure_catch);
        closure_catch.forget();
        closure_then.forget();
    }

    pub fn prompt_then_catch<T,F>(&mut self, message: &str, title: &str,mut then: Option<T>, mut catch: Option<F>)
        where F: FnMut(Action) + 'static , T: FnMut(Option<String>,Action) + 'static {
        let closure_then = Closure::new(move |value: JsValue|{
            if let Ok(prompt_value) = from_value::<PromptValue>(value) {
                if let Some(f) = then.as_mut() {
                    f(prompt_value.value,Action::from_str(&prompt_value.action));
                }
            }else{
                log!("invalid prompt_value");
            }
        });
        let closure_catch = Closure::new(move |value: JsValue|{
            let mut action = Action::None;
            if let Some(value) = value.as_string() {
                action = Action::from_str(&value);
            }
            if let Some(f) = catch.as_mut() {
                f(action);
            }
        });
        let _  = el_message_box_prompt(&JsValue::from_str(message), &JsValue::from_str(title), &self.js_object).then(&closure_then).catch(&closure_catch);
        closure_catch.forget();
        closure_then.forget();
    }
}

