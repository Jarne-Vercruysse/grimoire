use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use reactive_stores::{Field, Store};
use thaw::Theme;
use thaw::{self, ButtonShape};
use thaw::{ssr::SSRMountStyleProvider, ButtonSize};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <SSRMountStyleProvider>
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
        </SSRMountStyleProvider>
    }
}

#[derive(Store, Clone)]
struct Data {
    #[store(key: u8 = |row| row.key.clone())]
    rows: Vec<DatabaseEntry>,
}

#[derive(Store, Clone)]
struct DatabaseEntry {
    key: u8,
    name: String,
    expired: bool,
}

#[component]
pub fn App() -> impl IntoView {
    use std::collections::HashMap;

    let brand_colors = RwSignal::new(HashMap::from([
        (10, "#050201"),
        (20, "#231310"),
        (30, "#3B1C19"),
        (40, "#4F231F"),
        (50, "#642A26"),
        (60, "#7A322C"),
        (70, "#903A32"),
        (80, "#A64139"),
        (90, "#BD4A3F"),
        (100, "#D45246"),
        (110, "#EC5B4D"),
        (120, "#FB6D5B"),
        (130, "#FF8571"),
        (140, "#FF9E8B"),
        (150, "#FFB5A4"),
        (160, "#FFCABE"),
    ]));

    let theme = RwSignal::new(Theme::light());
    let on_customize_light_theme = move |_| {
        theme.set(Theme::custom_light(&brand_colors.get()));
    };
    let on_customize_dark_theme = move |_| {
        theme.set(Theme::custom_dark(&brand_colors.get()));
    };

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let dir = RwSignal::new(thaw::ConfigDirection::Auto);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/grimoire.css" />
        // sets the document title
        <Title text="Grimoire" />

        // content for this welcome page

        <thaw::ConfigProvider dir theme>
            <Router>
                <thaw::Space>
                    <thaw::Button
                        appearance=thaw::ButtonAppearance::Primary
                        on_click=move |_| theme.set(Theme::light())
                    >
                        "Light"
                    </thaw::Button>
                    <thaw::Button
                        appearance=thaw::ButtonAppearance::Primary
                        on_click=on_customize_light_theme
                    >
                        "Custom Light Theme"
                    </thaw::Button>
                    <thaw::Button
                        appearance=thaw::ButtonAppearance::Primary
                        on_click=on_customize_dark_theme
                    >
                        "Custom Dark Theme"
                    </thaw::Button>
                </thaw::Space>
                <main>
                    <Routes fallback=|| ErrorPage>
                        <Route path=StaticSegment("") view=ShareSpace />
                        <Route path=StaticSegment("/download") view=DownloadPage />
                    </Routes>
                </main>
            </Router>
        </thaw::ConfigProvider>
    }
}

#[component]
pub fn ShareSpace() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    let files = Store::new(Data {
        rows: vec![
            DatabaseEntry {
                key: 1,
                name: String::from("bestand1"),
                expired: false,
            },
            DatabaseEntry {
                key: 2,
                name: String::from("bestand2"),
                expired: false,
            },
            DatabaseEntry {
                key: 3,
                name: String::from("bestand3"),
                expired: false,
            },
            DatabaseEntry {
                key: 4,
                name: String::from("bestand4"),
                expired: false,
            },
            DatabaseEntry {
                key: 5,
                name: String::from("bestand5"),
                expired: false,
            },
            DatabaseEntry {
                key: 6,
                name: String::from("bestand6"),
                expired: true,
            },
        ],
    });

    view! {
        <h1>"ShareSpace"</h1>
        <thaw::Button
            size=ButtonSize::Large
            shape=ButtonShape::Circular
            appearance=thaw::ButtonAppearance::Primary
            on_click=on_click
        >
            Click me
            {count}
        </thaw::Button>
        <thaw::Table>
            <thaw::TableHeader>
                <thaw::TableRow>
                    <thaw::TableHeaderCell resizable=true>
                        "ID"
                    </thaw::TableHeaderCell>
                    <thaw::TableHeaderCell resizable=true>"NAME"</thaw::TableHeaderCell>
                    <thaw::TableHeaderCell>"expired"</thaw::TableHeaderCell>
                </thaw::TableRow>
            </thaw::TableHeader>
            <thaw::TableBody>
                <For
                    each=move || files.rows()
                    key=|row| row.read().key.clone()
                    children=|child| {
                        let id = child.key();
                        let name = child.name();
                        let expired = child.expired();
                        view! {
                            //<li>{move || name.get()}</li>
                            <thaw::TableRow>
                                <thaw::TableCell>
                                    <thaw::TableCellLayout>
                                        {move || id.get()}
                                    </thaw::TableCellLayout>
                                </thaw::TableCell>
                                <thaw::TableCell>
                                    <thaw::TableCellLayout>
                                        {move || name.get()}
                                    </thaw::TableCellLayout>
                                </thaw::TableCell>
                                <thaw::TableCell>
                                    <thaw::TableCellLayout>
                                        {move || expired.get()}
                                    </thaw::TableCellLayout>
                                </thaw::TableCell>
                            </thaw::TableRow>
                        }
                    }
                />
            </thaw::TableBody>
        </thaw::Table>
    }
}

// #[component]
// pub fn FileRow(store: Store<Data>, #[prop(into)] file: Field<DatabaseEntry>) -> impl IntoView {
//     view! { <p>"DownloadPage"</p> }
// }

#[component]
pub fn DownloadPage() -> impl IntoView {
    view! { <p>"DownloadPage"</p> }
}

#[component]
pub fn ErrorPage() -> impl IntoView {
    view! { <p>"Errorrrs"</p> }
}
