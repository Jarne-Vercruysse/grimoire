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

//#[server]
//async fn register_user(user: CreateUser) -> Result<(), ServerFnError> {
//    logging::log!(
//        "User got created.Email:  {:#?}, Password: {:#?}",
//        user.email,
//        user.password
//    );
//    Ok(())
//}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct LoginCredentials {
    email: String,
    password: String,
}

//#[derive(Deserialize, Serialize, Clone, Debug)]
//struct CreateUser {
//    first_name: String,
//    last_name: String,
//    email: String,
//    password: String,
//}

#[component]
pub fn LoginPage() -> impl IntoView {
    let action = ServerAction::<LoginUser>::new();
    logging::log!("LoginPage");

    // <div class="relative flex flex-col items-center justify-center h-screen overflow-hidden">
    view! {
        // <div class="relative flex flex-col items-center justify-center h-screen overflow-hidden">
        // <div class="w-full p-6 base-100 border-t-4 border-secondary rounded-md shadow-md border-top lg:max-w-lg">
        // <div class="bg-base-300 p-6 border-t-4 border-accent rounded-md lg:max-w-lg">
        // <div class="card-body">
        // <h1 class="card-title w-full text-5xl font-semibold text-left text-accent pb-4">
        // Grimoire
        // </h1>
        <ActionForm action>
            <fieldset class="fieldset bg-base-200 border-accent rounded-box w-xs border-2 py-8 px-6 space-y-2">
                <legend class="fieldset-legend text-xl text-accent">Sign In</legend>
                <label class="floating-label">
                    <span>Email</span>
                    <input
                        type="text"
                        name="credentials[email]"
                        placeholder="Email"
                        class="input"
                    />
                </label>

                <label class="floating-label">
                    <span>Password</span>
                    <input
                        type="text"
                        name="credentials[password]"
                        placeholder="Password"
                        class="input"
                    />
                </label>
                <input type="submit" value="Login" class="btn btn-accent mt-4" />
                <div class="flex flex-row place-content-between">
                    <div class="link link-hover link-accent">
                        <A href="/registration">Signup</A>
                    </div>
                    <div class="link link-hover link-neutral">
                        <A href="/">Forgot Password?</A>
                    </div>
                </div>
            </fieldset>
        </ActionForm>
    }
}

//#[component]
//pub fn RegistrationPage() -> impl IntoView {
//    let action = ServerAction::<RegisterUser>::new();
//    logging::log!("RegistrationPage");
//
//    view! {
//    <ActionForm action>
//                <fieldset class="fieldset bg-base-200 border-accent rounded-box w-xs border-2 py-8 px-6 space-y-2">
//                    <legend class="fieldset-legend text-xl text-accent">Signup</legend>
//
//                    <label class="floating-label">
//                        <span>First name</span>
//                        <input
//                            type="text"
//                            name="user[first_name]"
//                            placeholder="First name"
//                            class="input"
//                        />
//                    </label>
//
//                    <label class="floating-label">
//                        <span>Last name</span>
//                        <input
//                            type="text"
//                            name="user[last_name]"
//                            placeholder="Last name"
//                            class="input"
//                        />
//                    </label>
//
//
//                    <label class="floating-label">
//                        <span>Email</span>
//                        <input
//                            type="text"
//                            name="user[email]"
//                            placeholder="Last name"
//                            class="input"
//                        />
//                    </label>
//
//
//                    <label class="floating-label">
//                        <span>Password</span>
//                        <input
//                            type="text"
//                            name="user[password]"
//                            placeholder="Password"
//                            class="input"
//                        />
//                    </label>
//                    <input type="submit" value="Signup" class="btn btn-accent mt-4" />
//                    <div class="flex flex-row place-content-between">
//                        <div class="link link-hover link-primary">
//                            <A href="/login">Sign in</A>
//                        </div>
//                    </div>
//                </fieldset>
//            </ActionForm>
//        }
//}
