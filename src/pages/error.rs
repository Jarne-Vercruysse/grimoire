use leptos::prelude::*;

#[component]
pub fn ErrorPage() -> impl IntoView {
    view! {
        <div class="error-page flex flex-col items-center justify-center min-h-screen bg-gray-100 text-center">
            <div class="error-icon text-6xl">
                <span role="img" aria-label="screaming-face">
                    "😱"
                </span>
            </div>
            <h1 class="text-4xl font-bold text-red-600">
                "Whoops! Something went terribly wrong!"
            </h1>
            <p class="text-xl mt-4">
                "It seems we've encountered an unexpected error. Don't worry, we're on it!"
            </p>
            <p class="mt-2 text-lg">"In the meantime, feel free to enjoy this error page."</p>
            <div class="mt-4 text-sm text-gray-500">
                "Error Code: 500 - Spaghetti code everywhere!"
            </div>
        </div>
    }
}
