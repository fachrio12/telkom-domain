use yew_router::prelude::*;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/articles")]
    Articles,
    #[at("/get-started")]
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
// }