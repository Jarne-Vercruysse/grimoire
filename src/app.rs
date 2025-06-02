use {
    super::features::*,
    crate::{features::storage::get_files, types::Client},
    leptos::prelude::*,
    leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context},
    leptos_router::{
        StaticSegment,
        components::{Route, Router, Routes},
    },
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let files = get_files().await.unwrap();
    provide_context(Client::new(Files { files }));

    view! {
        <Stylesheet id="leptos" href="/pkg/grimoire.css" />
        <Title text="Grimoire" />
        <Router>
            <main>
                <Routes fallback=|| "Error">
                    <Route path=StaticSegment("login") view=auth::LoginPage />
                    <Route path=StaticSegment("registration") view=registration::RegistrationPage />
                    <Route path=StaticSegment("") view=|| home::HomePage />
                </Routes>
            </main>
        </Router>
    }
}
