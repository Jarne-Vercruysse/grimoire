use {leptos::prelude::*, leptos_router::components::A};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        "HomePage"

        <A href="login">Link to login</A>
    }
}
