use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="flex items-center gap-x-4 px-4 py-3 text-sm font-semibold text-base-content/70 border-b border-base-300 bg-base-100 sticky top-0 z-10 shadow-sm">
            <div class="w-[5%] flex justify-center">
                <input type="checkbox" class="checkbox checkbox-xs" />
            </div>
            <div class="w-[35%] truncate">Name</div>
            <div class="w-[15%] text-right">Size</div>
            <div class="w-[30%] truncate">Mime</div>
            <div class="w-[15%] text-right">Actions</div>
        </div>
    }
}
