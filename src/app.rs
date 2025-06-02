use {
    super::features::*,
    crate::{
        features::storage::{GetFiles, get_files},
        types::{Client, FileEntry, FilesStoreFields},
    },
    leptos::prelude::*,
    leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context},
    leptos_router::{
        StaticSegment,
        components::{Route, Router, Routes},
    },
    wasm_bindgen::UnwrapThrowExt,
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
    let client = Client::new();
    let records = LocalResource::new(move || get_files());

    let entrys: Vec<FileEntry> = records
        .get()
        .unwrap()
        .unwrap_or_default()
        .iter()
        .map(|f| FileEntry::from_record(f))
        .collect();

    client.update(crate::types::Message::Welcome { list: entrys });

    //let action_get_files = ServerAction::<GetFiles>::new();
    // let fun_files = Resource::new(
    //     move || client.store.0.entries().get(),
    //     // every time `count` changes, this will run
    //     |_| async move { get_files().await },
    // );
    //
    // //let files = move |_| fun_files.get;
    // // //let files = (move || fun_files.get().unwrap().unwrap())();
    // // let files = (move || {
    // //     fun_files
    // //         .get()
    // //         .map(|f| f.unwrap())
    // //         .unwrap_or_default()
    // //         .iter()
    // //         .map(|f| FileEntry::from_record(f))
    // //         .collect()
    // // })();
    //
    // client.update(crate::types::Message::Welcome {
    //     list: fun_files
    //         .get()
    //         .unwrap()
    //         .unwrap_or_default()
    //         .iter()
    //         .map(|f| FileEntry::from_record(f))
    //         .collect(),
    // });

    provide_context(client);

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
