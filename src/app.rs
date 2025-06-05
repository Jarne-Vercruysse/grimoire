use leptos::reactive::spawn_local;

use crate::{core::types::StateAction, features::storage::api::load_users_files, pages};
use {
    crate::core::types::AppState,
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
        <html lang="en" data-theme="silk">
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
    let app_state = AppState::new();

    Effect::new(move || {
        spawn_local(async move {
            if let Ok(fetched) = load_users_files().await {
                app_state.initialise_state(StateAction::InitialiseFileState(fetched));
            }
        });
    });

    provide_context(app_state);

    view! {
        <Stylesheet id="leptos" href="/pkg/grimoire.css" />
        <Title text="Grimoire" />
        <Router>
            <main>
                <Routes fallback=|| "error">
                    // <Route path=StaticSegment("login") view=auth::LoginPage />
                    // <Route path=StaticSegment("registration") view=registration::RegistrationPage />
                    // <Route path=StaticSegment("") view=|| home::HomePage />
                    <Route path=StaticSegment("") view=|| pages::home::HomePage />
                </Routes>
            </main>
        </Router>
    }
}
