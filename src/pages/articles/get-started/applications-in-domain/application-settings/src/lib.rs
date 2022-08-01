use yew::prelude::*;
use alert::Alert;

pub struct ApplicationSettings {}

pub enum Msg {}

impl Component for ApplicationSettings {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ApplicationSettings {}
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
                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Application Settings" }</h1>
                <Alert message={String::from("Before you register any APIs in the Auth0 Dashboard, one API will already exist: the Auth0 Management API. To learn more about the features of the Management API and its available endpoints, see Management API.")}/>

                <ul class="uk-list uk-list-decimal">
                    <li>
                        <p>
                            { "Go to" }
                            <a href="localhost:8080">{ "Dashboard > Applications > APIs" }</a>
                            { ", and select + Create API." }
                        </p>
                        <img
                            class="uk-margin-top uk-margin-bottom"
                            src="/assets/register-apis/dashboard-applications-apis-create-api.png"
                        />
                    </li>
                    <li>
                        <p>
                            { "Provide the following information for your API, and click" }
                            <b class="td-margin-text">{ "Create:" }</b>
                        </p>
                        <table class="uk-table uk-table-divider">
                            <thead>
                                <tr>
                                    <th class="uk-text-emphasis">{ "Field" }</th>
                                    <th class="uk-text-emphasis">{ "Description" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td>
                                        { "Name" }
                                    </td>
                                    <td>{ "A friendly name for the API. Does not affect any functionality." }</td>
                                </tr>
                                <tr>
                                    <td>
                                        { "Identifier" }
                                    </td>
                                    <td>
                                        { "A unique identifier for the API. Auth0 recommends using a URL. Auth0 does differentiate between URLs that include the last forward slash. For example," }
                                        <span class="td-label-code td-margin-text">
                                            { "https://example.com" }
                                        </span>
                                        { "and" }
                                        <span class="td-label-code td-margin-text">
                                            { "https://example.com/" }
                                        </span>
                                        { "are two different identifiers. The URL does not have to be a publicly available URL. Auth0 will not call your API. This value cannot be modified afterwards." }
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        { "Signing Algorithm" }
                                    </td>
                                    <td>{ "The algorithm to sign the tokens with. The available values are HS256 and RS256. When selecting RS256 the token will be signed with the tenant's private key." }</td>
                                </tr>
                            </tbody>
                        </table>
                    </li>
                    <li>
                        <p>
                            { "Make the implementation changes to your API that are described in the QuickStart. These changes consist of choosing a JWT library from a predefined list and configuring this library to validate the access tokens in your API." }
                        </p>
                        <img
                            class="uk-margin-top uk-margin-bottom"
                            src="/assets/register-apis/dashboard-apis-edit_view-quick-start.png"
                        />
                    </li>
                </ul>

                <div class="uk-margin-large-bottom">
                    <p>
                        { "The other available Dashboard views for your API are:" }
                    </p>
                    <ul class="uk-list uk-list-decimal">
                        <li>
                            <b class="td-margin-text">{ "Settings: " }</b>
                            { "Lists the settings for your API. Some are editable. Here you can change the token expiration time and enable offline access (this way Auth0 will allow your applications to ask for refresh tokens for this API)." }
                        </li>
                        <li>
                            <b class="td-margin-text">{ "Scopes: " }</b>
                            { "Define the scopes for this API by setting a name and a description." }
                        </li>
                        <li>
                            <b class="td-margin-text">{ "Machine to Machine Applications: " }</b>
                            { "Lists all applications for which the " }
                            <b class="td-margin-text">{ "Client Credentials" }</b>
                            { " grant is enabled. By default, this grant is enabled for regular web applications and machine-to-machine applications. You can authorize any of these applications to request access tokens for your API. Optionally, you can select a subset of the defined scopes to limit your authorized application's access." }
                        </li>
                        <li>
                            <b class="td-margin-text">{ "Test: " }</b>
                            { "Execute a sample client credentials flow with any of your authorized applications to check that everything is working as expected." }
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "API Settings" }</li>
                        <li>{ "Token Best Practices" }</li>
                        <li>{ "Which OAuth 2.0 Flow Should I Use?" }</li>
                    </ul>
                </div>
                
            </>
        }
    }
}
