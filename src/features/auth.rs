use leptos::logging;
use leptos::prelude::*;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

#[server]
async fn login_user(credentials: LoginCredentials) -> Result<(), ServerFnError> {
    logging::log!(
        "user logged in.Email:  {:#?}, Password: {:#?}",
        credentials.email,
        credentials.password
    );
    Ok(())
}

#[server]
async fn logout_user() -> Result<(), ServerFnError> {
    Ok(())
}

#[server]
async fn register_user(user: CreateUser) -> Result<(), ServerFnError> {
    logging::log!(
        "User got created.Email:  {:#?}, Password: {:#?}",
        user.email,
        user.password
    );
    Ok(())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct LoginCredentials {
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct CreateUser {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let login_action = ServerAction::<LoginUser>::new();
    logging::log!("LoginPage");

    view! {
         <ActionForm action=login_action>
        <label>
            "Email"
            <input type="tekst" name="credentials[email]"/>
        </label>
        <label>
            "password"
            <input type="tekst" name="credentials[password]"/>
        </label>
            <input type="submit"/>
        </ActionForm>
        <A href="/registration">Create account</A>
    }
}

#[component]
pub fn RegistrationPage() -> impl IntoView {
    let action = ServerAction::<RegisterUser>::new();
    logging::log!("RegistrationPage");

    view! {
         <ActionForm action>
        <label>
            "First Name"
            <input type="tekst" name="user[first_name]"/>
        </label>
        <label>
            "Last name"
            <input type="tekst" name="user[last_name]"/>
        </label>
        <label>
            "Email"
            <input type="tekst" name="user[email]"/>
        </label>
        <label>
            "password"
            <input type="tekst" name="user[password]"/>
        </label>
            <input type="submit"/>
        </ActionForm>
        <A href="/">root</A>
    }
}
