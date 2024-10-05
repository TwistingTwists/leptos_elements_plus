
use leptos::*;
use leptos::logging::log;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, closure::Closure};
use js_sys::{Array,Object/*,Reflect*/};
use serde::{Serialize,Deserialize,de::DeserializeOwned};
use serde_wasm_bindgen::from_value;

use super::utils::{convert,set_property};
pub use super::common::Alignment;

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct CellRenderProps<T>{
    cellData: T,
    rowIndex: i32,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct HeaderRenderProps{
    columnIndex: i32,
    headerIndex: i32,
}

#[component]
pub fn TableV2<T>(
    #[prop(into, optional, default = 2.into())] cache: MaybeSignal<i32>,
    #[prop(into, optional)] estimated_row_height: MaybeSignal<i32>,
    #[prop(into, optional)] header_class: MaybeSignal<String>,
    #[prop(into, optional)] header_height: MaybeSignal<i32>,
    #[prop(into, optional)] footer_height: MaybeSignal<i32>,
    #[prop(into, optional)] row_class: MaybeSignal<String>,
    #[prop(into, optional)] row_key: MaybeSignal<String>,
    #[prop(into, optional, default = 50.into())] row_height: MaybeSignal<i32>,
    #[prop(into, optional)] columns: MaybeSignal<Vec<Column>>,
    #[prop(into)] data: MaybeSignal<Vec<T>>,
    #[prop(into, optional)] fixed_data: MaybeSignal<Vec<T>>,
    #[prop(into, optional)] expand_column_key: MaybeSignal<String>,
    #[prop(into, optional)] expanded_row_keys: MaybeSignal<Vec<String>>,
    #[prop(into, optional)] default_expanded_row_keys: MaybeSignal<Vec<String>>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into, optional)] fixed: MaybeSignal<bool>,
    #[prop(into)] width: MaybeSignal<i32>,
    #[prop(into, optional)] height: MaybeSignal<i32>,
    #[prop(into, optional)] max_height: MaybeSignal<i32>,
    #[prop(into, optional, default = 6.into())] h_scrollbar_size: MaybeSignal<i32>,
    #[prop(into, optional, default = 6.into())] v_scrollbar_size: MaybeSignal<i32>,
    #[prop(into, optional)] scrollbar_always_on: MaybeSignal<bool>,
    ) -> impl IntoView
    where T: Serialize + 'static + Clone + std::fmt::Debug
{
    /*
     TODO: cleanup
    on_cleanup(move || {
        log!("TableV2: cleanup");
    });
    */
    view! {
        <el-table-v2
            width=width
            height=height
            prop:columns=move || { 
                columns_to_jsvalue(&columns.get())
            }
            prop:data=move || {
                if let Ok(data) = serde_wasm_bindgen::to_value(&data) {
                    data
                } else {
                    log!("TableV2: data is not serializable");
                    JsValue::UNDEFINED
                }
            }
            prop:fixed-data=move || {
                if let Ok(data) = serde_wasm_bindgen::to_value(&fixed_data) {
                    data
                } else {
                    log!("TableV2: fixed_data is not serializable");
                    JsValue::UNDEFINED
                }
            }
            cache=cache
            estimated-row-height=estimated_row_height
            header-class=header_class
            prop:header-height=move || {
                if header_height.get() == 0 {
                    return JsValue::UNDEFINED;
                }
                if let Ok(data) = serde_wasm_bindgen::to_value(&header_height.get()) {
                    data
                } else {
                    log!("TableV2: header_height is not serializable");
                    JsValue::UNDEFINED
                }
            }
            footer-height=footer_height
            row-class=row_class
            row-key=row_key
            row-height=row_height
            prop:expand-column-key=move || {
                if let Ok(data) = serde_wasm_bindgen::to_value(&expand_column_key.get()) {
                    data
                } else {
                    log!("TableV2: expand_column_key is not serializable");
                    JsValue::UNDEFINED
                }
            }
            prop:expanded-row-keys=move || {
                if let Ok(data) = serde_wasm_bindgen::to_value(&expanded_row_keys.get()) {
                    data
                } else {
                    log!("TableV2: expanded_row_keys is not serializable");
                    JsValue::UNDEFINED
                }
            }
            prop:default-expanded-row-keys=move || {
                if let Ok(data) = serde_wasm_bindgen::to_value(&default_expanded_row_keys.get()) {
                    data
                } else {
                    log!("TableV2: default_expanded_row_keys is not serializable");
                    JsValue::UNDEFINED
                }
            }
            class=class
            fixed=fixed
            max-height=max_height
            h-scrollbar-size=h_scrollbar_size
            v-scrollbar-size=v_scrollbar_size
            scrollbar-always-on=scrollbar_always_on
        ></el-table-v2>
    }
}

#[derive(Clone, Debug, Default)]
pub struct Column {
    js_object: Object,

    renderer: Option<JsValue>,
    header_renderer: Option<JsValue>,

    //required
    pub title: String,
    pub data_key: String,
    pub width: i32,

    //optional
    pub align: Alignment,
    pub class: String,
    pub fixed: bool,
    pub flex_grow: i32,
    pub flex_shrink: i32,
    pub header_class: String,
    pub hidden: bool,
    pub style: String,
    pub sortable: bool,
    pub max_width: String,
    pub min_width: String,
}

impl Column{
    pub fn new(data_key: String, title: String, width: i32) -> Self {
        let js_object = Object::new();
        set_property(&js_object, "key", data_key.clone());
        set_property(&js_object, "dataKey", data_key.clone());
        set_property(&js_object, "title", title.clone());
        set_property(&js_object, "width", width.to_string());
        Self {
            js_object,
            data_key,
            title,
            width,
            ..Default::default()
        }
    }

    pub fn align(mut self, align: Alignment) -> Self {
        set_property(&self.js_object, "align", align.as_str());
        self.align = align;
        self
    }

    pub fn class(mut self, class: String) -> Self {
        set_property(&self.js_object, "class", class.clone());
        self.class = class;
        self
    }

    pub fn fixed(mut self, fixed: bool) -> Self {
        set_property(&self.js_object, "fixed", fixed);
        self.fixed = fixed;
        self
    }

    pub fn flex_grow(mut self, flex_grow: i32) -> Self {
        set_property(&self.js_object, "flexGrow", flex_grow.to_string());
        self.flex_grow = flex_grow;
        self
    }

    pub fn flex_shrink(mut self, flex_shrink: i32) -> Self {
        set_property(&self.js_object, "flexShrink", flex_shrink);
        self.flex_shrink = flex_shrink;
        self
    }

    pub fn header_class(mut self, header_class: String) -> Self {
        set_property(&self.js_object, "headerClass", header_class.clone());
        self.header_class = header_class;
        self
    }

    pub fn hidden(mut self, hidden: bool) -> Self {
        set_property(&self.js_object, "hidden", hidden);
        self.hidden = hidden;
        self
    }

    pub fn style(mut self, style: String) -> Self {
        set_property(&self.js_object, "style", style.clone());
        self.style = style;
        self
    }

    pub fn sortable(mut self, sortable: bool) -> Self {
        set_property(&self.js_object, "sortable", sortable);
        self.sortable = sortable;
        self
    }

    pub fn max_width(mut self, max_width: String) -> Self {
        set_property(&self.js_object, "maxWidth", max_width.clone());
        self.max_width = max_width;
        self
    }

    pub fn min_width(mut self, min_width: String) -> Self {
        set_property(&self.js_object, "minWidth", min_width.clone());
        self.min_width = min_width;
        self
    }

    pub fn cell_renderer<T,F>(mut self, _cell_renderer: F) -> Self 
        where T: DeserializeOwned + 'static,
              F: Fn(i32,T)->View + 'static {
        let closure: Closure<dyn FnMut(JsValue)->JsValue> = Closure::new(move|cell_data: JsValue|{
            let data: CellRenderProps<T>;
            if let Ok(d) = from_value(cell_data) {
                data = d;
            }else{
                log!("TableV2: cell_data is not deserializable");
                return JsValue::UNDEFINED.into();
            }

            let view = _cell_renderer(data.rowIndex, data.cellData);

            if let View::Element(v) = view {
                return convert(v.into_html_element().into_any().get_root_node());
            }else{
                log!("cell_renderer must return View::Element");
                return JsValue::UNDEFINED.into();
            }
        });

        js_sys::Reflect::set(&self.js_object, &JsValue::from_str("cellRenderer"), &closure.as_ref().unchecked_ref()).unwrap();
        self.renderer = Some(closure.into_js_value());
        self
    }

    pub fn header_cell_renderer<F>(mut self, _header_cell_renderer: F) -> Self 
        where F: Fn(i32,i32)->View + 'static {
        let closure: Closure<dyn FnMut(JsValue)->JsValue> = Closure::new(move|cell_data: JsValue|{
            let data: HeaderRenderProps;
            if let Ok(d) = from_value(cell_data) {
                data = d;
            }else{
                log!("TableV2: cell_data is not deserializable");
                return JsValue::UNDEFINED.into();
            }

            let view = _header_cell_renderer(data.columnIndex, data.headerIndex);

            if let View::Element(v) = view {
                return convert(v.into_html_element().into_any().get_root_node());
            }else{
                log!("cell_renderer must return View::Element");
                return JsValue::UNDEFINED.into();
            }
        });

        js_sys::Reflect::set(&self.js_object, &JsValue::from_str("headerCellRenderer"), &closure.as_ref().unchecked_ref()).unwrap();
        self.header_renderer = Some(closure.into_js_value());
        self
    }
}

impl Into<JsValue> for Column {
    fn into(self) -> JsValue {
        self.js_object.into()
    }
}

fn columns_to_jsvalue(columns: &Vec<Column>) -> JsValue {
    let js_values: Vec<JsValue> = columns.iter().map(|column| {
        column.clone().into()
    }).collect();
    let array = js_values.into_iter().collect::<Array>();
    array.into()
}
