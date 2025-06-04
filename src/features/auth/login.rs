// #[component]
// pub fn LoginPage() -> impl IntoView {
//     let action = ServerAction::<LoginUser>::new();
//     logging::log!("LoginPage");
//
//     view! {
//         <ActionForm action>
//             <fieldset class="fieldset bg-base-200 border-accent rounded-box w-xs border-2 py-8 px-6 space-y-2">
//                 <legend class="fieldset-legend text-xl text-accent">Sign In</legend>
//                 <label class="floating-label">
//                     <span>Email</span>
//                     <input
//                         type="text"
//                         name="credentials[email]"
//                         placeholder="Email"
//                         class="input"
//                     />
//                 </label>
//
//                 <label class="floating-label">
//                     <span>Password</span>
//                     <input
//                         type="text"
//                         name="credentials[password]"
//                         placeholder="Password"
//                         class="input"
//                     />
//                 </label>
//                 <input type="submit" value="Login" class="btn btn-accent mt-4" />
//                 <div class="flex flex-row place-content-between">
//                     <div class="link link-hover link-accent">
//                         <A href="/registration">Signup</A>
//                     </div>
//                     <div class="link link-hover link-neutral">
//                         <A href="/">Forgot Password?</A>
//                     </div>
//                 </div>
//             </fieldset>
//         </ActionForm>
//     }
// }
