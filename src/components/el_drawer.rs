
use leptos::*;
//use leptos::logging::log;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerDirection {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}

impl DrawerDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            DrawerDirection::Right => "rtl",
            DrawerDirection::Left => "ltr",
            DrawerDirection::Top => "ttb",
            DrawerDirection::Bottom => "btt",
        }
    }
}

#[component]
pub fn Drawer(
    #[prop(into, default = DrawerDirection::Right.into())] direction: DrawerDirection,
    #[prop(into, default = false.into())] show: RwSignal<bool>,
    #[prop(into, optional)] title: Option<AttributeValue>,
    #[prop(into, optional)] size: Option<AttributeValue>,
    children: Children
    ) -> impl IntoView
{

    view! {
        <el-drawer
            size=size
            // model-value=show
            prop:modelValue=show
            title=title
            direction=direction.as_str()
            on:ce-close=move |_ev: leptos::ev::CustomEvent| { show.set(false) }
        >

            {children()}
        </el-drawer>
    }
}

