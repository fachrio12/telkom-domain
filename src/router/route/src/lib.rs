use yew_router::prelude::*;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/articles")]
    Articles,
    // #[at("/get-started/:s")]
    // GetStarted,
    #[at("/get-started/identity-fundamentals/identity-and-access-management")]
    IntroductionToIAM,
    #[at("/get-started/identity-fundamentals")]
    GetStartedIdentityFundamentals,
    #[at("/get-started/")]
    GetStartedHome,

    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// #[derive(Clone, Routable, PartialEq)]
// pub enum RouteGetStarted {
//     #[at("/get-started/")]
//     Home,
//     #[not_found]
//     #[at("get-started/404")]
//     NotFound,
// }