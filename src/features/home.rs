use {
    icondata,
    leptos::{logging, prelude::*},
    leptos_icons::Icon,
};

#[component]
pub fn HomePage() -> impl IntoView {
    let logout_action = ServerAction::<super::auth::LogoutUser>::new();
    logging::log!("homepage");
    view! {
        <div class="min-h-screen flex flex-row-reverse border-5 border-accent">
            <div class="border-5 border-error p-10 flex flex-col justify-between">
                <div>
                    <Icon icon=icondata::AiApiFilled width="9em" height="9em" />
                    // <img src=icondata::AiApiFilled/>
                    <h1 class="text-5xl font-bold">Grimoire</h1>
                </div>

                <ActionForm action=logout_action>
                    <input type="submit" value="Logout" class="btn btn-accent mt-4 w-full" />
                </ActionForm>
            </div>

            // div for Main content
            // "Main content"
            <div class="border-5 border-secondary flex flex-col w-screen">
                // div around drop zone
                <div class="border-dashed border-5 bg-base-100 p-20 border border-secondary glass grow-1">
                    <p class="text-5xl">DROP FILES</p>
                </div>
                // div aroun table
                <div class="border-4 border-primary p-10 grow-10">
                    <table class="table bg-base-300">
                        <thead>
                            <tr>
                                <th>
                                    <label>
                                        <input type="checkbox" class="checkbox" />
                                    </label>
                                </th>
                                <th>Name</th>
                                <td>Type</td>
                                <td>Size</td>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="hover:bg-base-300">
                                <td>
                                    <label>
                                        <input type="checkbox" class="checkbox" />
                                    </label>
                                </td>
                                <td>1Cy Ganderton</td>
                                <td>Quality Control Specialist</td>
                                <td>Blue</td>
                            </tr>
                            <tr>
                                <td>
                                    <label>
                                        <input type="checkbox" class="checkbox" />
                                    </label>
                                </td>
                                <td>2Cy Ganderton</td>
                                <td>Quality Control Specialist</td>
                                <td>Blue</td>
                            </tr>
                            <tr>
                                <td>
                                    <label>
                                        <input type="checkbox" class="checkbox" />
                                    </label>
                                </td>
                                <td>3Cy Ganderton</td>
                                <td>Quality Control Specialist</td>
                                <td>Blue</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <div class="border-4 border-accent">"Action div only visable when something is selected"</div>
            </div>
        </div>
    }
}
