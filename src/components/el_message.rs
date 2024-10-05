use leptos::*;
use leptos::logging::log;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, closure::Closure};
use js_sys::Object;

use super::utils::{convert,set_property};
pub use super::common::Type;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = element_plus, js_name = ElMessage)]
    fn el_message(msg: &JsValue);
}


#[allow(non_snake_case)]
#[derive(Clone, Debug, Default)]
pub struct Message{
    js_object: Object,
    has_content: bool,
    message_function: Option<JsValue>,
}

impl Message{
    pub fn new() -> Self {
        self::Message{
            js_object: Object::new(),
            has_content: false,
            message_function: None,
        }
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
        self.message_function = Some(closure.into_js_value());
        self.has_content = true;
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

    pub fn show_close(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "showClose", b);
        self
    }

    pub fn center(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "center", b);
        self
    }

    pub fn on_close<F>(&mut self, f: F) -> &mut Self
        where F: Fn() + 'static {
        let closure: Closure<dyn FnMut()> = Closure::new(f);
        js_sys::Reflect::set(&self.js_object, &JsValue::from_str("onClose"), &closure.as_ref().unchecked_ref()).unwrap();
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

    pub fn grouping(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "grouping", b);
        self
    }

    pub fn repeat_num(&mut self, n: u32) -> &mut Self {
        set_property(&self.js_object, "repeatNum", n);
        self
    }

    pub fn dangerously_use_html_string(&mut self, b: bool) -> &mut Self {
        set_property(&self.js_object, "dangerouslyUseHTMLString", b);
        self
    }

    pub fn show(&mut self) {
        if self.has_content {
            el_message(&self.js_object);
        }else{
            log!("Message has no content");
        }
    }
}
