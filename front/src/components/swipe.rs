#[derive(Clone, PartialEq)]
pub struct Action {
    pub active: bool,
    pub callback: yew::Callback<uuid::Uuid>,
    pub color: &'static str,
    pub icon: (&'static str, &'static str),
    pub id: uuid::Uuid,
}

impl Action {
    fn emit(&self) {
        self.callback.emit(self.id);
    }

    fn inactive_icon(&self) -> &'static str {
        self.icon.0
    }

    fn active_icon(&self) -> &'static str {
        self.icon.1
    }

    fn view(&self, position: &str, delta: f32) -> yew::Html {
        let icon = if (self.active && delta.abs() < 0.5) || (!self.active && delta.abs() > 0.5) {
            self.active_icon()
        } else {
            self.inactive_icon()
        };

        yew::html! {
            <div
                class={ yew::classes!("action", position.to_string()) }
                style={ format!("color: var({color}); background-color: color-mix(in srgb, var({color}) 50%, transparent)", color=self.color) }
            >
                <super::Svg {icon} size=24 />
            </div>
        }
    }
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub children: yew::Html,
    pub action_start: Action,
    pub action_end: Action,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let start = yew::use_state(|| 0);
    let delta = yew::use_state(|| 0.);

    let ontouchstart = {
        let start = start.clone();

        yew::Callback::from(move |e: web_sys::TouchEvent| {
            start.set(e.touches().get(0).unwrap().client_x());
        })
    };

    let ontouchmove = {
        let delta = delta.clone();
        let start = start.clone();

        yew::Callback::from(move |e: web_sys::TouchEvent| {
            use wasm_bindgen::JsCast as _;

            let container = e.target().unwrap().unchecked_into::<web_sys::Element>();
            let client_x = e.touches().get(0).unwrap().client_x();
            delta.set((*start - client_x) as f32 / container.client_width() as f32);
        })
    };

    let ontouchend = {
        let delta = delta.clone();
        let props = props.clone();

        yew::Callback::from(move |_| {
            if *delta < -0.5 {
                props.action_start.emit();
            } else if *delta > 0.5 {
                props.action_end.emit();
            }

            start.set(0);
            delta.set(0.);
        })
    };

    yew::html! {
        <>
            <div
                class="swipe-container"
                {ontouchstart}
                {ontouchmove}
                {ontouchend}
            >
                { props.action_start.view("start", *delta) }
                <div
                    class="swipe-element"
                >
                    { props.children.clone() }
                </div>
                { props.action_end.view("end", *delta) }
            </div>
        </>
    }
}
