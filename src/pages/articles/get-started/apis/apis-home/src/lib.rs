use yew::prelude::*;
use alert::Alert;


pub struct ApisHome {}

pub enum Msg {}

impl Component for ApisHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ApisHome {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "APIs" }</h1>
                <p>
                    { "An API is an entity that represents an external resource, capable of accepting and responding to protected resource requests made by applications. In the OAuth2 specification, an API maps to the Resource Server." }
                </p>
                <p>
                    { "At some point, your custom APIs will need to allow limited access to their protected resources on behalf of users. Authorization refers to the process of verifying what a user has access to. While often used interchangeably with authentication, authorization represents a fundamentally different function. To learn more, read Authentication and Authorization." }
                </p>
                <p>
                    { "In authorization, a user or application is granted access to an API after the API determines the extent of the permissions that it should assign. Usually, authorization occurs after identity is successfully validated through authentication so that the API has some idea of what sort of access it should grant." }
                </p>
                <p>
                    { "Authorization can be determined through the use of policies and rules, which can be used with role-based access control (RBAC). Regardless of whether RBAC is used, requested access is transmitted to the API via scopes and granted access is returned in issued Access Tokens." }
                </p>
                <p class="uk-margin-medium-bottom">
                    { "The application can then use the Access Toke to access the API's protected resources. The same Access Token can be used to access the API's resources without having to authenticate again until it expires." }
                </p>



                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "API permissions" }</h1>
                    <p>{ "Since only the API can know all of the possible actions that it can handle, it should have its own internal access control system in which it defines its own permissions. To determine a calling application's effective permissions, an API should combine incoming scopes with the permissions assigned within its own internal access control system and make access control decisions accordingly." }</p>
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Configure an API" }</h1>
                    <p>
                        { "To protect an API, you must register an API using the Auth0 Dashboard. To learn more, see Register APIs." }
                    </p>
                    <Alert message={String::from("Before you register any APIs in the Dashboard, one API will already exist: the Management API. To learn more about the features of the Management API and its available endpoints, see Management API.")} />
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "API Scopes" }</li>
                        <li>{ "Tokens" }</li>
                        <li>{ "Register APIs" }</li>
                        <li>{ "API Settings" }</li>
                    </ul>
                </div>



                
            </>
        }
    }
}
