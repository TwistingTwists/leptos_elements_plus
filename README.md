A wrapper of [Element Plus](https://element-plus.org) for [Leptos](https://leptos.dev/).

Explore the combination of Vue components with Leptos,it’s simply for the sake of reusing the wheel.



#### A Simple

###### main.rs

```rust
mod app;

use leptos::*;
use leptos_meta::provide_meta_context;

use app::*;
use leptos_element_plus::ElementPlusSetup;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);

    provide_meta_context();
    mount_to_body(|| {
        view! {
            <ElementPlusSetup/>
            <App/>
        }
    })
}
```

###### app.rs

```rust
use leptos::*;
use leptos::logging::log;
use js_sys::Date;

use leptos_element_plus::components::el_date_time_picker::DateTimePicker;

#[component]
pub fn App() -> impl IntoView {
    let date = create_rw_signal(Date::new_0());
    create_effect(move|_| {
        log!("date: {:?}", date.get().to_string());
    });
    view! {
        <div class="flex flex-col p-6 space-y-4">
            <DateTimePicker
                value=date
                placeholder="Select date and time"
            />
        </div>
    }
}
```
