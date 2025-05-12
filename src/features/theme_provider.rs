//use leptos::logging::log;
//use leptos::prelude::*;
//use std::collections::HashMap;
//use thaw::*;
//
//fn get_catppuccin_latte_theme_map() -> HashMap<i32, &'static str> {
//    HashMap::from([
//        (10, "#eff1f5"),  // Base (Lightest background)
//        (20, "#e6e9ef"),  // Mantle
//        (30, "#dce0e8"),  // Crust
//        (40, "#ccd0da"),  // Surface0
//        (50, "#bcc0cc"),  // Surface1
//        (60, "#acb0be"),  // Surface2
//        (70, "#6c6f85"),  // Subtext0 (Lighter text)
//        (80, "#4c4f69"),  // Text (Main text, dark on light background)
//        (90, "#7287fd"),  // Lavender
//        (100, "#1e66f5"), // Blue
//        (110, "#209fb5"), // Sapphire
//        (120, "#04a5e5"), // Sky
//        (130, "#179299"), // Teal
//        (140, "#40a02b"), // Green
//        (150, "#df8e1d"), // Yellow
//        (160, "#d20f39"), // Red
//    ])
//}
//
//#[server]
//async fn change_theme() -> Result<(), ServerFnError> {
//    Ok(())
//}
//
//#[component]
//pub fn ThemeSwitcher() -> impl IntoView {
//    let brand_colors = RwSignal::new(HashMap::from([
//        (10, "#050201"),
//        (20, "#231310"),
//        (30, "#3B1C19"),
//        (40, "#4F231F"),
//        (50, "#642A26"),
//        (60, "#7A322C"),
//        (70, "#903A32"),
//        (80, "#A64139"),
//        (90, "#BD4A3F"),
//        (100, "#D45246"),
//        (110, "#EC5B4D"),
//        (120, "#FB6D5B"),
//        (130, "#FF8571"),
//        (140, "#FF9E8B"),
//        (150, "#FFB5A4"),
//        (160, "#FFCABE"),
//    ]));
//    //let brand_colors = get_catppuccin_latte_theme_map();
//
//    let theme = RwSignal::new(Theme::light());
//    let on_customize_light_theme = move |_| {
//        log!("Light theme");
//        theme.set(Theme::custom_light(&brand_colors.get()));
//    };
//    let on_customize_dark_theme = move |_| {
//        log!("Dark theme");
//        theme.set(Theme::custom_dark(&brand_colors.get()));
//    };
//    theme.set(Theme::custom_light(&brand_colors.get()));
//
//    view! {
//    <Card>
//                <Space>
//                    <Button on_click=on_customize_light_theme>"Light"</Button>
//                    <Button on_click=on_customize_dark_theme>"Dark"</Button>
//                </Space>
//            </Card>
//        }
//}
