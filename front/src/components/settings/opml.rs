#[yew::function_component]
pub(crate) fn Component() -> yew::Html {
    let export_url = format!("{}/opml", env!("API_URL"));
    let context = crate::use_context();
    let files = yew::use_state(Vec::<gloo::file::Blob>::new);

    let on_change = yew_callback::callback!(files, move |e: yew::Event| {
        use yew::TargetCast as _;

        let mut f = Vec::new();
        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();

        if let Some(file_list) = input.files() {
            for x in 0..file_list.length() {
                if let Some(file) = file_list.get(x) {
                    f.push(file.into());
                }
            }
        }

        files.set(f);
    });

    let import = yew_callback::callback!(context, files, move |_| {
        for file in &*files {
            let context = context.clone();
            let file = file.clone();

            yew::platform::spawn_local(async move {
                let content = gloo::file::futures::read_as_text(&file)
                    .await
                    .unwrap_or_default();

                crate::api::call!(context, opml_import, content);

                let alert = crate::Alert::info("Import done");
                context.dispatch(alert.into());

                context.dispatch(crate::Action::NeedUpdate);
            });
        }
    });

    yew::html! {
        <>
            <div class="input-group mb-3">
                <input type="file" class="form-control" onchange={ on_change } />
                <button
                    class={ yew::classes!("btn", "btn-outline-primary") }
                    type="button"
                    onclick={ import }
                >{ "Import" }</button>
            </div>
            <div class="input-group">
                <a href={ export_url } target="_blank" class={ yew::classes!("btn", "btn-outline-primary") }>{ "Export" }</a>
            </div>
        </>
    }
}
