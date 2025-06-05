use crate::features::storage::api::fetch_download_file_by_id;
use leptos::prelude::*;
use leptos_router::{hooks, params::Params, *};
use uuid::Uuid;

#[derive(Params, PartialEq)]
struct DownloadParams {
    id: String,
}

#[component]
pub fn DownloadPage() -> impl IntoView {
    let params = hooks::use_params_map();
    let file = Resource::new(
        move || params.get().get("id").clone(),
        |id| async move {
            let id = id?.parse::<Uuid>().ok()?;
            fetch_download_file_by_id(id).await.ok()
        },
    );

    view! {
        <div class="min-h-screen flex items-center justify-center bg-base-200 px-4">
            <div class="max-w-md w-full bg-base-100 rounded-xl shadow-lg p-6 space-y-6 text-center">
                <Suspense fallback=|| {
                    view! { <p>"Loading..."</p> }
                }>
                    {move || match file.get() {
                        Some(Some(file)) => {
                            view! {
                                <div>
                                    <h1 class="text-2xl font-bold">
                                        "Ready to download your file"
                                    </h1>
                                    <div class="space-y-1">
                                        <p class="text-lg font-medium truncate">
                                            "example-report.pdf"
                                        </p>
                                        <p class="text-sm text-base-content/60">
                                            "1.45 MB • application/pdf"
                                        </p>
                                    </div>
                                    <a href="#" class="btn btn-primary btn-wide">
                                        "Download"
                                    </a>
                                    <p class="text-xs text-base-content/50">
                                        "This link is valid for 24 hours and can be used once."
                                    </p>
                                </div>
                            }
                        }
                        Some(None) => {
                            view! {
                                <div>
                                    <p class="text-error">"File not found."</p>
                                </div>
                            }
                        }
                        None => {
                            view! {
                                <div>
                                    <p>"Invalid ID."</p>
                                </div>
                            }
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}
