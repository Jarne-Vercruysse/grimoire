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

    // <div class="relative flex flex-col items-center justify-center h-screen overflow-hidden">
    view! {
        // <div class="relative flex flex-col items-center justify-center h-screen overflow-hidden">
        // <div class="w-full p-6 base-100 border-t-4 border-secondary rounded-md shadow-md border-top lg:max-w-lg">
        <div class="bg-base-300 p-6 border-t-4 border-accent rounded-md lg:max-w-lg">
            <div class="card-body">
                <h1 class="card-title w-full text-5xl font-semibold text-left text-accent pb-4">
                    Grimoire
                </h1>
                <ActionForm action=login_action>
                    <div class="space-y-4">
                        // <label class="label">
                        // <span class="text-accent label-text">"Your Email"</span>
                        // </label>
                        <input
                            type="text"
                            name="credentials[email]"
                            placeholder="Email"
                            class="w-full input input-bordered"
                        />
                        // <label class="label">
                        // <span class="text-accent label-text">"Your Password"</span>
                        // </label>
                        <input
                            type="text"
                            name="credentials[password]"
                            placeholder="Password"
                            class="w-full input input-bordered"
                        />
                        <div class="card-actions">
                            <input type="submit" class="btn btn-block btn-accent">Login</input>
                        </div>
                    </div>
                </ActionForm>

            </div>
            <div class="link">
                <A href="/registration">Create account</A>
            </div>
        </div>
    }
}

#[component]
pub fn RegistrationPage() -> impl IntoView {
    let action = ServerAction::<RegisterUser>::new();
    logging::log!("RegistrationPage");

    view! {
        <ActionForm action>
            <label>"First Name" <input type="tekst" name="user[first_name]" /></label>
            <label>"Last name" <input type="tekst" name="user[last_name]" /></label>
            <label>"Email" <input type="tekst" name="user[email]" /></label>
            <label>"password" <input type="tekst" name="user[password]" /></label>
            <input type="submit" />
        </ActionForm>
        <A href="/">root</A>
    }
}
