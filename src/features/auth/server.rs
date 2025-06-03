//#[server]
// async fn login_user(credentials: LoginCredentials) -> Result<(), ServerFnError> {
//     logging::log!(
//         "user logged in.Email:  {:#?}, Password: {:#?}",
//         credentials.email,
//         credentials.password
//     );
//     Ok(())
// }
//
// #[server]
// pub async fn logout_user() -> Result<(), ServerFnError> {
//     leptos_axum::redirect("/login");
//     Ok(())
// }
//
// #[server]
// async fn register_user(user: CreateUser) -> Result<(), ServerFnError> {
//     logging::log!(
//         "User got created.Email:  {:#?}, Password: {:#?}",
//         user.email,
//         user.password
//     );
//     // Check if user is already inside of the db
//
//     leptos_axum::redirect("/");
//     Ok(())
// }
