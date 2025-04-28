use leptos::prelude::*;
//use rand::prelude::*;
//use leptos_icons;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use reactive_stores::Store;
use std::sync::atomic::{AtomicU8, Ordering};
use thaw::Theme;
use thaw::{self, ButtonShape};
use thaw::{ssr::SSRMountStyleProvider, ButtonSize};

static NEXT_ID: AtomicU8 = AtomicU8::new(0);

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
struct StoreFiles {
    #[store(key: u8 = |row| row.key.clone())]
    rows: Vec<GrimFile>,
}

#[derive(Store, Clone)]
struct GrimFile {
    key: u8,
    name: String,
    expired: bool,
}

impl GrimFile {
    pub fn new(name: impl ToString, expired: bool) -> GrimFile {
        GrimFile {
            key: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            name: name.to_string(),
            expired,
        }
    }
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

fn what_is_name() -> String {
    let vec1 = vec![
        "sunrise", "ember", "silent", "crystal", "frost", "shadow", "nova", "blaze", "pixel",
        "drift",
    ];

    let vec2 = vec![
        "whisper", "lunar", "dusk", "echo", "breeze", "raven", "flint", "mist", "storm", "cinder",
    ];

    let vec3 = vec![
        "harbor", "willow", "sage", "marble", "glimmer", "velvet", "shard", "boulder", "grove",
        "hollow",
    ];

    let vec4 = vec![
        "emberly", "solstice", "prism", "vault", "aurora", "cobalt", "shiver", "tundra", "meadow",
        "zephyr",
    ];
    //let mut rng = rand::rng();
    let vec1_length = vec1.len();
    let vec2_length = vec2.len();
    let vec3_length = vec3.len();
    let vec4_length = vec4.len();

    let word_one = vec1[vec1_length];
    let word_two = vec2[vec2_length];
    let word_three = vec3[vec3_length];
    let word_four = vec4[vec4_length];
    format!("{word_one}-{word_two}-{word_three}-{word_four}")
}

fn is_expired() -> bool {
    false
}

fn add_to_list(store: Store<StoreFiles>) {
    //let name = what_is_name();
    let name = "test";
    let expired = is_expired();
    let item = GrimFile::new(name, expired);
    store.rows().write().push(item);
}

#[component]
pub fn ShareSpace() -> impl IntoView {
    let files = Store::new(StoreFiles {
        rows: vec![
            GrimFile::new("randomtekst.png", false),
            GrimFile::new("panda.png", true),
            GrimFile::new("kreeft.png", true),
            GrimFile::new("hond.png", true),
            GrimFile::new("kat.png", true),
            GrimFile::new("dog.png", true),
            GrimFile::new("gobbler.png", true),
            GrimFile::new("king.png", true),
            GrimFile::new("sucker.png", true),
        ],
    });

    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    let add_icon = RwSignal::new(Some(icondata::AiPlusCircleOutlined));
    let remove_icon = RwSignal::new(Some(icondata::AiDeleteOutlined));

    view! {
        <h1>"ShareSpace"</h1>
        <thaw::Flex justify=thaw::FlexJustify::SpaceAround>
            <thaw::Button
                icon=add_icon
                size=ButtonSize::Large
                shape=ButtonShape::Circular
                appearance=thaw::ButtonAppearance::Primary
                //on_click=add_to_list(files)
            >
                Add file
            </thaw::Button>

            <thaw::Button
                icon=remove_icon
                size=ButtonSize::Large
                shape=ButtonShape::Circular
                appearance=thaw::ButtonAppearance::Subtle
                on_click=on_click
            >
                Remove
                {count}
            </thaw::Button>

        </thaw::Flex>
        <thaw::Table>
            <thaw::TableHeader>
                <thaw::TableRow>
                    <thaw::TableHeaderCell resizable=true>"ID"</thaw::TableHeaderCell>
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
                            // <li>{move || name.get()}</li>
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
