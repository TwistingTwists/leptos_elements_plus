use leptos::*;
use leptos::logging::log;

//use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, closure::Closure};
//use js_sys::{Array,Object,Reflect};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressType {
    #[default]
    Line,
    Circle,
    Dashboard,
}

impl ProgressType{
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgressType::Line => "line",
            ProgressType::Circle => "circle",
            ProgressType::Dashboard => "dashboard",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressStatus {
    #[default]
    Success,
    Exception,
    Warning,
}

impl ProgressStatus{
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgressStatus::Success => "success",
            ProgressStatus::Exception => "exception",
            ProgressStatus::Warning => "warning",
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressLinecap {
    Butt,
    #[default]
    Round,
    Square,
}

impl ProgressLinecap{
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgressLinecap::Butt => "butt",
            ProgressLinecap::Round => "round",
            ProgressLinecap::Square => "square",
        }
    }
}

#[component]
pub fn Progress(
    #[prop(into, optional)] percentage: MaybeSignal<u8>,
    #[prop(into, optional)] type_: Option<ProgressType>,
    #[prop(into, optional)] stroke_width: Option<u8>,
    #[prop(into, optional)] text_inside: bool,
    #[prop(into, optional)] status: Option<ProgressStatus>,
    #[prop(into, optional)] indeterminate: bool,
    #[prop(into, optional)] duration: Option<u8>,
    #[prop(into, optional)] color: Option<String>,
    #[prop(into, optional)] width: Option<u8>,
    #[prop(into, optional, default = true.into())] show_text: bool,
    #[prop(into, optional)] stroke_linecap: Option<String>,
    #[prop(into, optional)] format: Option<Callback<u8,String>>,
    #[prop(into, optional)] striped: bool,
    #[prop(into, optional)] striped_flow: bool,
    #[prop(optional)] children: Option<Children>,
    ) -> impl IntoView
{
    view! {
        <el-progress
            percentage=percentage
            type=type_.unwrap_or_default().as_str()
            stroke-width=stroke_width
            text-inside=text_inside
            status=match status {
                Some(status) => status.as_str(),
                None => ""
            }
            indeterminate=indeterminate
            duration=duration
            color=color
            width=width
            show-text=show_text
            stroke-linecap=stroke_linecap
            striped=striped
            striped-flow=striped_flow
            prop:format=move||{
                if let Some(format) = format {
                    let closure: Closure<dyn FnMut(JsValue)->JsValue> = Closure::new(move|percentage| {
                        if let Ok(percentage) = serde_wasm_bindgen::from_value::<i32>(percentage) {
                            let result = format.call(percentage as u8);
                            if let Ok(result) = serde_wasm_bindgen::to_value(&result) {
                                return result;
                            }
                        }else{
                            log!("Error: format callback parameter is not a number");
                        }
                        JsValue::UNDEFINED
                    });
                    closure.into_js_value()
                }else{
                    JsValue::UNDEFINED
                }
            }
        >
            {
                match children {
                    Some(children) => children(),
                    None => Fragment::new(vec![]),
                } 
            }
        </el-progress>
    }
}
