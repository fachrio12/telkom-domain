use yew::prelude::*;

pub struct IntroductionToDomain {}

pub enum Msg {}

impl Component for IntroductionToDomain {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        IntroductionToDomain {}
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

                <h1 class="uk-heading-small uk-margin-bottom">{ "INTRODUCTIOn To DOMAIN" }</h1>
                    
                <div class="uk-margin-large-bottom">
                    <p>
                        { "While often used interchangeably, authentication and authorization represent fundamentally different functions. In this article, we compare and contrast the two to show how they protect applications in complementary ways." }
                    </p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">{ "What are authentication and authorization?" }</h2>
                    <p>
                        { "In simple terms, authentication is the process of verifying who a user is, while authorization is the process of verifying what they have access to." }
                    </p>
                    <p>
                        { "Comparing these processes to a real-world example, when you go through security in an airport, you show your ID to authenticate your identity. Then, when you arrive at the gate, you present your boarding pass to the flight attendant, so they can authorize you to board your flight and allow access to the plane." }
                    </p>
                </div>

                <div class="uk-margin-large-bottom">
                    <h2 class="uk-margin-bottom">
                        { "Authentication vs. authorization" }
                    </h2>
                    <p>
                        { "Here's a quick overview of the differences between authentication and authorization:" }
                    </p>
                    <table class="uk-table uk-table-divider">
                        <thead>
                            <tr>
                                <th class="uk-text-emphasis">{ "Authentication" }</th>
                                <th class="uk-text-emphasis">{ "Authorization" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>
                                    { "Determines whether users are who they claim to be" }
                                </td>
                                <td>{ "Determines what users can and cannot access
                                " }</td>
                            </tr>
                            <tr>
                                <td>
                                    { "Challenges the user to validate credentials (for example, through passwords, answers to security questions, or facial recognition)" }
                                </td>
                                <td>{ "Verifies whether access is allowed through policies and rules" }</td>
                            </tr>
                            <tr>
                                <td>
                                    { "Usually done before authorization" }
                                </td>
                                <td>{ "Usually done after successful authentication" }</td>
                            </tr>
                            <tr>
                                <td>
                                    { "Generally, transmits info through an ID Token" }
                                </td>
                                <td>{ "Generally, transmits info through an Access Token" }</td>
                            </tr>
                            <tr>
                                <td>
                                    { "Generally governed by the OpenID Connect (OIDC) protocol" }
                                </td>
                                <td>{ "Generally governed by the OAuth 2.0 framework" }</td>
                            </tr>
                            <tr>
                                <td>
                                    { "Example: Employees in a company are required to authenticate through the network before accessing their company email" }
                                </td>
                                <td>{ "Example: After an employee successfully authenticates, the system determines what information the employees are allowed to access" }</td>
                            </tr>
                        </tbody>
                    </table>
                    <p>
                        { "In short, access to a resource is protected by both authentication and authorization. If you can't prove your identity, you won't be allowed into a resource. And even if you can prove your identity, if you are not authorized for that resource, you will still be denied access." }
                    </p>
                </div>

            </>
        }
    }
}
