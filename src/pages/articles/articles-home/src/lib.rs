use yew::prelude::*;
use sidebar_articles::SidebarArticles;

pub struct ArticlesHome {}

pub enum Msg {}

impl Component for ArticlesHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ArticlesHome {}
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
                            <SidebarArticles/>
                        </div>
                    </div>

                    <div class="uk-width-expand@m">
                        <div
                            class="uk-padding-large"
                        >
                            <ul class="uk-breadcrumb">
                                <li><a href="#">{ "Docs" }</a></li>
                                <li><span>{ "Articles" }</span></li>
                            </ul>

                            <h1 class="uk-heading-small">{ "Articles" }</h1>
                            <p>
                                { "Explore all topics contained in the Articles section of Auth0's Docs site. Topics have been separated into the following categories." }
                            </p>

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
                                            <a class="uk-link-text uk-text-primary" href="#">{ "Get Started" }</a>
                                        </td>
                                        <td>{ "About how to get started using Auth0, including the basics of identity and access management, an overview of Auth0 services and tenant settings, how to register applications and APIs with Auth0, how to give team members access to your Auth0 Dashboard, the flows used for authentication and authorization, and common Auth0 architecture scenarios that you can use to solve your authorization and authentication problems." }</td>
                                    </tr>
                                </tbody>
                                <tbody>
                                    <tr>
                                        <td>
                                            <a class="uk-link-text uk-text-primary" href="#">{ "Authenticate" }</a>
                                        </td>
                                        <td>{ "About authenticating users and verifying their identities, including implementing single sign-on (SS0), allowing users to log in without needing to remember a password, provisioning users through external identity providers and user stores, and the protocols that specify how to design an authentication and authorization system." }</td>
                                    </tr>
                                    <tr>
                                        <td>
                                            <a class="uk-link-text uk-text-primary" href="#">{ "Manage Users" }</a>
                                        </td>
                                        <td>{ "About managing users, including manipulating user data, migrating users to Auth0, using search API endpoints, using Organizations to better manage partners and customers, implementing access control, and using sessions and cookies." }</td>
                                    </tr>
                                    <tr>
                                        <td>
                                            <a class="uk-link-text uk-text-primary" href="#">{ "Customize" }</a>
                                        </td>
                                        <td>{ "About branding, customizing, and internationalizing your login pages, domain names, emails sent to users, and consent prompts." }</td>
                                    </tr>
                                    <tr>
                                        <td>
                                            <a class="uk-link-text uk-text-primary" href="#">{ "Secure" }</a>
                                        </td>
                                        <td>{ "About securing Auth0 products, including alerts for suspicious behavior, multi-factor authentication (MFA), security bulletins, basic tips to keep data and user accounts safe, using tokens, and data privacy and compliance." }</td>
                                    </tr>
                                    <tr>
                                        <td>
                                            <a class="uk-link-text uk-text-primary" href="#">{ "Deploy and Monitor" }</a>
                                        </td>
                                        <td>{ "About deploying and monitoring Auth0 products, including deployment options, tools that help with deployment, tools to monitor Auth0 status and services, and tenant log event data." }</td>
                                    </tr>
                                    <tr>
                                        <td>
                                            <a class="uk-link-text uk-text-primary" href="#">{ "Troubleshoot" }</a>
                                        </td>
                                        <td>{ "About troubleshooting Auth0 products, including customer support options, professional services, troubleshooting tips and tools, and Auth0's product lifecycle." }</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
