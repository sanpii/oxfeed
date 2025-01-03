#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub values: Vec<String>,
    pub on_change: yew::Callback<Vec<String>>,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let tags = yew::use_state(|| props.values.clone());
    let on_change = props.on_change.clone();

    yew::use_effect_with(tags.clone(), move |tags| {
        on_change.emit((**tags).clone());
    });

    let on_select = {
        let tags = tags.clone();

        yew::Callback::from(move |value| {
            let mut new_value = (*tags).clone();

            if !new_value.contains(&value) {
                new_value.push(value);
                new_value.sort();

                tags.set(new_value);
            }
        })
    };

    let on_delete = {
        let tags = tags.clone();

        yew::Callback::from(move |_| {
            let mut new_value = (*tags).clone();
            new_value.pop();

            tags.set(new_value);
        })
    };

    yew::html! {
        <div class="form-control tags-input">
            {
                for tags.iter().enumerate().map(|(idx, tag)| {
                    let tags = tags.clone();

                    yew::html! {
                        <crate::components::Tag
                            value={ tag.clone() }
                            editable=true
                            on_click={
                                yew::Callback::from(move |_| {
                                    let mut new_value = (*tags).clone();
                                    new_value.remove(idx);

                                    tags.set(new_value);
                                })
                            }
                        />
                    }
                })
            }
            <super::Autocomplete
                on_select={ on_select }
                on_delete={ on_delete }
            />
        </div>
    }
}
