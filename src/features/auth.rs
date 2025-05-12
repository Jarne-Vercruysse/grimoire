use leptos::prelude::*;
use leptos_router::components::A;

#[server]
async fn login_user() -> Result<(), ServerFnError> {
    Ok(())
}

#[server]
async fn logout_user() -> Result<(), ServerFnError> {
    Ok(())
}

#[server]
async fn register_user() -> Result<(), ServerFnError> {
    Ok(())
}

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        "Something"
        <A href="/">To root</A>
    }
}

#[component]
fn RegistrationPage() -> impl IntoView {
    todo!()
}
