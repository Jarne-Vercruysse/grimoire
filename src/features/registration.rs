use {
    leptos::{logging, prelude::*},
    leptos_router::components::A,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct CreateUser {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[component]
pub fn RegistrationPage() -> impl IntoView {
    let action = ServerAction::<RegisterUser>::new();
    logging::log!("RegistrationPage");

    view! {
        <ActionForm action>
            <fieldset class="fieldset bg-base-200 border-accent rounded-box w-xs border-2 py-8 px-6 space-y-2">
                <legend class="fieldset-legend text-xl text-accent">Signup</legend>

                <label class="floating-label">
                    <span>First name</span>
                    <input
                        type="text"
                        name="user[first_name]"
                        required
                        placeholder="First name"
                        class="input validator"
                        minlength=3
                        pattern="[a-zA-Z]*"
                    />
                    <p class="validator-hint hidden">
                        Must be 3 to 30 characters <br />containing only letters
                    </p>
                </label>

                <label class="floating-label">
                    <span>Last name</span>
                    <input
                        type="text"
                        name="user[last_name]"
                        required
                        placeholder="Last name"
                        class="input validator"
                        minlength=3
                        pattern="[a-zA-Z]*"
                    />
                    <p class="validator-hint hidden">
                        Must be 3 to 30 characters <br />containing only letters
                    </p>
                </label>

                <label class="floating-label">
                    <span>Email</span>
                    <input
                        type="text"
                        name="user[email]"
                        required
                        placeholder="Email"
                        class="input validator"
                        pattern="[a-zA-Z0-9._%+\\-]+@[a-zA-Z0-9.\\-]+\\.[a-zA-Z]{1,}"
                    />
                    <p class="validator-hint hidden">
                        Must be a valid email format (e.g., user@example.com).
                    </p>
                </label>

                <label class="floating-label">
                    <span>Password</span>
                    <input
                        type="text"
                        name="user[password]"
                        placeholder="Password"
                        class="input validator"
                        required
                        minlength="8"
                        pattern="(?=.*\\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[!@#$%^&*]).{8,}"
                    />
                    <p class="validator-hint hidden">
                        Must be more than 8 characters, including <br />At least one number <br />
                        At least one special character <br />At least one lowercase letter <br />
                        At least one uppercase letter
                    </p>
                </label>

                <input type="submit" value="Signup" class="btn btn-accent mt-4" />
                <div class="flex flex-row place-content-between">
                    <div class="link link-hover link-accent">
                        <A href="/login">Sign in</A>
                    </div>
                </div>
            </fieldset>
        </ActionForm>
    }
}

#[server]
async fn register_user(user: CreateUser) -> Result<(), ServerFnError> {
    logging::log!(
        "User got created.Email:  {:#?}, Password: {:#?}",
        user.email,
        user.password
    );
    // Check if user is already inside of the db

    leptos_axum::redirect("/");
    Ok(())
}
