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

        let bg = if delta.abs() > 0.5 {
            format!(
                "background-color: color-mix(in srgb, var({}) 50%, transparent)",
                self.color
            )
        } else {
            String::new()
        };

        yew::html! {
            <div
                class={ yew::classes!("action", position.to_string()) }
                style={ format!("color: var({}); {bg}", self.color) }
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

    let ontouchstart = yew_callback::callback!(start, move |e: web_sys::TouchEvent| {
        start.set(e.touches().get(0).unwrap().client_x());
    });

    let ontouchmove = yew_callback::callback!(delta, start, move |e: web_sys::TouchEvent| {
        let client_x = e.touches().get(0).unwrap().client_x();
        let window = web_sys::window().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap() as f32;
        delta.set((*start - client_x) as f32 / width);
    });

    let ontouchend = yew_callback::callback!(delta, props, move |_| {
        if *delta < -0.5 {
            props.action_start.emit();
        } else if *delta > 0.5 {
            props.action_end.emit();
        }

        start.set(0);
        delta.set(0.);
    });

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
