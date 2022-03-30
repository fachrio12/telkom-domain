use yew::prelude::*;
use sidebar_get_started::SidebarGetStarted;


pub struct GetStartedHome {}

pub enum Msg {}

impl Component for GetStartedHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        GetStartedHome {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>

                <div
                    class="uk-grid-collapse"
                    uk-grid="true"
                >

                    <div class="uk-width-1-4@m td-border-right-light">
                        <div>
                            <SidebarGetStarted/>
                        </div>
                    </div>

                    <div class="uk-width-expand@m">
                        <div
                            class="uk-padding-large"
                        >
                            <ul class="uk-breadcrumb">
                                <li><a href="#">{ "Docs" }</a></li>
                                <li><span>{ "Get Started" }</span></li>
                            </ul>

                            <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Get Started" }</h1>
                            <p>
                                { "Welcome! If you are new to Auth0, you are in the right place. Here we will cover how to get started using Auth0." }
                            </p>
                            <p>
                                { "There are only a few steps you have to complete to start using Auth0." }
                            </p>

                            <ul class="uk-list uk-list-decimal uk-margin-medium-bottom">
                                <li>{ "Sign up and create an Auth0 tenant. A tenant is a logically isolated group of users who share common access requirements with specific privileges." }</li>
                                <li>{ "Register any application (written in any language or on any stack) to your Auth0 tenant and define the identity providers you want to use (how you want your users to log in)." }</li>
                                <li>{ "Based on your app's technology, choose one of our SDKs (or call our APIs), and hook it up to your app. Now each time a user tries to authenticate, Auth0 will verify their identity and send the required information back to your app. You can also register a custom API and configure the tokens, role-based access control (RBAC), and other access settings and permissions." }</li>
                                <li>{ "Configure how your Auth0 tenants, apps, and APIs work together to optimize how you authenticate and authorize your users." }</li>
                                <li>{ "Deploy your configuration." }</li>
                            </ul>


                            <div
                                class="uk-margin-medium-bottom"
                            >
                                <h1 class="td-text-size-large">{ "Start Building" }</h1>
                                <p>{ "Explore step-by-step guides to quickly integrate Auth0 into your application, using our Quickstarts." }</p>
                            </div>

                            <div
                                class="uk-margin-medium-bottom"
                            >
                                <h1 class="td-text-size-large">{ "Learn the Basics" }</h1>
                                <table class="uk-table uk-table-divider">
                                    <thead>
                                        <tr>
                                            <th class="uk-text-emphasis">{ "Read..." }</th>
                                            <th class="uk-text-emphasis">{ "To learn..." }</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td>
                                                <a class="uk-link-text uk-text-primary" href="#">{ "Identity Fundamentals" }</a>
                                            </td>
                                            <td>{ "About the basics of identity and access management." }</td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <a class="uk-link-text uk-text-primary" href="#">{ "Telkom Domain Overview" }</a>
                                            </td>
                                            <td>{ "About Auth0 services and how to get started using them with your applications and APIs." }</td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>

                            <div
                                class="uk-margin-medium-bottom"
                            >
                                <h1 class="td-text-size-large">{ "Configure Telkom Domain" }</h1>
                                <table class="uk-table uk-table-divider">
                                    <thead>
                                        <tr>
                                            <th class="uk-text-emphasis">{ "Read..." }</th>
                                            <th class="uk-text-emphasis">{ "To learn..." }</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td>
                                                <a class="uk-link-text uk-text-primary" href="#">{ "Tenant Settings" }</a>
                                            </td>
                                            <td>{ "About the settings related to tenants available in the Telkom Domain Dashboard." }</td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <a class="uk-link-text uk-text-primary" href="#">{ "Applications in Telkom Domain" }</a>
                                            </td>
                                            <td>{ "About the basics of registering and configuring your applications in Auth0." }</td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <a class="uk-link-text uk-text-primary" href="#">{ "APIs" }</a>
                                            </td>
                                            <td>{ "About key topics related to working with APIs." }</td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <a class="uk-link-text uk-text-primary" href="#">{ "Manage Dashboard Access" }</a>
                                            </td>
                                            <td>{ "How to manage your team members to have access permissions on your Auth0 Dashboard." }</td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>

                            <div
                                class="uk-margin-medium-bottom"
                            >
                                <h1 class="td-text-size-large">{ "Plan and Design" }</h1>
                                <table class="uk-table uk-table-divider">
                                    <thead>
                                        <tr>
                                            <th class="uk-text-emphasis">{ "Read..." }</th>
                                            <th class="uk-text-emphasis">{ "To learn..." }</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td>
                                                <a class="uk-link-text uk-text-primary" href="#">{ "Authentication and Authorization Flows" }</a>
                                            </td>
                                            <td>{ "About the various flows used for authentication and authorization of applications and APIs." }</td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <a class="uk-link-text uk-text-primary" href="#">{ "Architecture Scenarios" }</a>
                                            </td>
                                            <td>{ "About common Auth0 architecture scenarios that you can use to solve your authorization and authentication needs." }</td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>

                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
