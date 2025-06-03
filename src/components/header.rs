use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <h2 class="text-lg font-bold text-gray-700 mb-3">My Files</h2>
        <div class="grid grid-cols-4 text-sm text-gray-500 border-b font-semibold py-2">
            <div>Name</div>
            <div>Size</div>
            <div>Mime</div>
        </div>
    }
}
