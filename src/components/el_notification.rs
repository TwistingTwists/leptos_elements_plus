
use leptos::*;
use leptos::logging::log;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, closure::Closure};
use js_sys::Object;

use super::utils::{convert,set_property};
pub use super::common::Type;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = element_plus, js_name = ElNotification)]
    fn el_notification(msg: &JsValue)->JsValue;
    //#[wasm_bindgen(js_namespace = element_plus, js_name = "ElNotification.closeAll")]
    //fn el_notification_close_all();
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl Position {
    pub fn as_str(&self) -> &'static str {
        match self {
            Position::TopRight => "top-right",
            Position::TopLeft => "top-left",
            Position::BottomRight => "bottom-right",
            Position::BottomLeft => "bottom-left",
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default)]
pub struct Notification{
    instance: Option<JsValue>,
    js_object: Object,
    has_content: bool,
}

impl Notification{
    pub fn new() -> Self {
        self::Notification{
            js_object: Object::new(),
            has_content: false,
            instance: None,
        }
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

    pub fn message_type(&mut self, t: Type) -> &mut Self {
        set_property(&self.js_object, "type", t.as_str());
        self
    }

    pub fn custom_class(&mut self, c: String) -> &mut Self {
        set_property(&self.js_object, "customClass", c);
        self
    }

    pub fn duration(&mut self, d: u32) -> &mut Self {
        set_property(&self.js_object, "duration", d);
        self
    }

    pub fn position(&mut self, p: Position) -> &mut Self {
        set_property(&self.js_object, "position", p.as_str());
        self
    }

    pub fn show_close(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "showClose", b);
        self
    }

    pub fn on_close<F>(&mut self, f: F) -> &mut Self
        where F: Fn() + 'static {
        let closure: Closure<dyn FnMut()> = Closure::new(f);
        js_sys::Reflect::set(&self.js_object, &JsValue::from_str("onClose"), &closure.as_ref().unchecked_ref()).unwrap();
        self
    }

    pub fn on_click<F>(&mut self, f: F) -> &mut Self
        where F: Fn() + 'static {
        let closure: Closure<dyn FnMut()> = Closure::new(f);
        js_sys::Reflect::set(&self.js_object, &JsValue::from_str("onClick"), &closure.as_ref().unchecked_ref()).unwrap();
        self
    }

    pub fn offset(&mut self, o: u32) -> &mut Self {
        set_property(&self.js_object, "offset", o);
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

    pub fn z_index(&mut self, z: u32) -> &mut Self {
        set_property(&self.js_object, "zIndex", z);
        self
    }

    /*
    pub fn close_all() {
        el_notification_close_all();
    }
    */

    pub fn close(&mut self) {
        if let Some(instance) = &self.instance {
            let close = js_sys::Reflect::get(instance, &JsValue::from_str("close"));
            if let Ok(close) = close {
                if let Some(close) = close.dyn_ref::<js_sys::Function>() {
                    close.call0(&instance).unwrap();
                }
            }
        }
    }

    pub fn show(&mut self) {
        if self.has_content {
            self.instance = Some(el_notification(&self.js_object));
        }else{
            log!("Notification has no content");
        }
    }
}

