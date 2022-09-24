use yew::prelude::*;



pub struct Scopes {}

pub enum Msg {}

impl Component for Scopes {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Scopes {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Scopes" }</h1>
                <p>
                    { "Different pieces of user information are often stored across a number of online resources. Users may upload and store photos with a service like Flickr, keep digital files on Dropbox, and store contacts and events in Google Calendar or on Facebook." }
                </p>
                <p>
                    { "Often, new applications will want to make use of the information that has already been created in an online resource. To do so, the application must ask for authorization to access this information on a user's behalf. Scopes define the specific actions applications can be allowed to do on a user's behalf." }
                </p>
                

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Ways to use scopes" }</h1>
                    <p>{ "When an app requests permission to access a resource through an authorization server, it uses the" }
                    <b class="td-label-code">
                       { "Scope" }
                    </b>
                     {"parameter to specify what access it needs, and the authorization server uses the"}
                     <b class="td-label-code">
                     { "Scope" }
                     </b>
                     {"parameter to respond with the access that was actually granted (if the granted access was different from what was requested)."}
                    </p>

                    <p>
                    {"Generally, you use scopes in three ways:"}
                    </p>

                    <ul class="uk-list uk-list-disc">
                    <li>
                       
                        { "From an application, to verify the identity of a user and get basic profile information about the user, such as their email or picture. In this scenario, the scopes available to you include those implemented by the OpenID Connect (OIDC) protocol. To learn more, read OpenID Connect Scopes." }
                    </li>
                    <li>
                      
                        { "In an API, to implement access control. In this case, you need to define custom scopes for your API and then identify these scopes so that calling applications can use them. To learn more, read API Scopes." }
                    
                    </li>
                    <li>
                      
                        { "From an application, to call an API that has implemented its own custom scopes. In this case, you need to know which custom scopes are defined for the API you are calling. To see examples of calling a custom API from an application, read Sample Use Cases: Scopes and Claims" }
                    </li>
                    
                </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Best practices" }</h1>
                    <p>
                        { "Understand your use case and choose the most restrictive scopes possible." }
                    </p>

                    <p>
                        { "If you are requesting scopes, make sure you ask for enough access for your application to function, but only request what you absolutely need. Are you establishing a user's identity or asking the user to allow you to interact with their data? There's a big difference between importing a user's Facebook profile information and posting to their wall. By only requesting what you need, you are more likely to gain user consent when required since users are more likely to grant access for limited, clearly specified scopes." }
                    </p>

                    <p>
                    { "Similarly, when creating custom scopes for an API, consider what levels of granular access applications may need and design accordingly." }
                </p>
                    
                </div>

                <div
                class="uk-margin-large-bottom"
            >
                <h1 class="td-text-size-large">{ "Requested scopes versus granted scopes" }</h1>
                <p>
                    { "In certain cases, users get to consent to the access being requested. While usually, the scopes returned will be identical to those requested, users can edit granted scopes (both during initial consent and sometimes after, depending on the resource), thereby granting an app less access than it requested." }
                </p>

                <p>
                    { "As an application developer, you should be aware of this possibility and handle these cases in your app. For example, your app could warn the user that they will see reduced functionality. It could also send the user back through the authorization flow to ask for additional permissions. But again, remember that when asked for consent, users can always say no." }
                </p>

                <p>
                { "By default, Auth0 skips user consent for first-party applications, which are applications that are registered under the same Auth0 domain as the API they are calling; however, you can configure your API in Auth0 to require user consent from first-party applications. Third-party applications, which are external applications, require user consent." }
            </p>
                
            </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "OpenID Connect Scopes" }</li>
                        <li>{ "API Scopes" }</li>
                        <li>{ "Sample Use Cases: Scopes and Claims" }</li>
                        <li>{ "Enable Role-Based Access Control for APIs" }</li>
                    </ul>
                </div>



                
            </>
        }
    }
}
