use yew::prelude::*;
use yew_router::prelude::*;
use route::{
    Route,
};

pub struct TelkomDomainDashboardHome {}

pub enum Msg {}

impl Component for TelkomDomainDashboardHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        TelkomDomainDashboardHome {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Telkom Domain Dashboard" }</h1>

                <div
                    class="uk-margin-large-bottom"
                >
                    <p>
                        { "The Auth0 Dashboard is where you manage all aspects of your Auth0 subscription and configuration." }
                    </p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/domain-dashboard/dashboard-activity_with-nav.png"
                    />
                    <p>
                        { "It consists of several sections that you can navigate using the sidebar menu on your left." }
                    </p>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Configure implementation" }</h1>
                    <p>
                        { "The following table contains a brief overview of the different Dashboard sections and what you can do in each." }
                    </p>
                    <table class="uk-table uk-table-divider">
                        <thead>
                            <tr>
                                <th class="uk-text-emphasis td-text-weight-600">{ "Section" }</th>
                                <th class="uk-text-emphasis td-text-weight-600">{ "Description" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>
                                    <span class="td-text-weight-600">{ "Applications" }</span>
                                </td>
                                <td>
                                    <div>{ "Manage your applications, APIs, and single sign-on (SSO) integrations." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Applications: " }
                                        </span>
                                        { "For each of your apps for which you want to authenticate users with Auth0, register an application." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "APIs: " }
                                        </span>
                                        { "For each of your APIs that you want to secure with Auth0, register an API. Create new APIs and manage existing ones." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "SSO Integrations: " }
                                        </span>
                                        { "View and enable external services for SSO. Create new SSO integrations and configure, review, and manage integration settings." }
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-text-weight-600">{ "Authentication" }</span>
                                </td>
                                <td>
                                    <div>{ "Manage the identity providers through which you allow users to authenticate to your apps." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Database: " }
                                        </span>
                                        { "Securely store and manage username/password credentials either in an Auth0 datastore or in your own database. Connect to existing databases using template-based JavaScript scripts that run on Auth0's server during every authentication. Gradually migrate an existing database of legacy credentials to Auth0 as users authenticate (no password reset required)." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Social: " }
                                        </span>
                                        { "Configure social identity providers (such as Facebook, Twitter, and Github) through which your users can log in." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Enterprise: " }
                                        </span>
                                        { "Configure enterprise identity providers (such as Active Directory, SAML, and Office 365) through which your users can log in using their enterprise credentials." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Passwordless: " }
                                        </span>
                                        { "Allow your users to sign up and log in using one-time passcodes (delivered by email or SMS) or one-click links, instead of passwords." }
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-text-weight-600">{ "User Management" }</span>
                                </td>
                                <td>
                                    <div>{ "Manage your users' identities and permissions." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Users: " }
                                        </span>
                                        { "View and create user profiles, perform password resets, block and delete users, and more." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Roles: " }
                                        </span>
                                        { "Create and manage roles for your apps. Roles contain collections of permissions and can be assigned to users." }
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-text-weight-600">{ "Branding" }</span>
                                </td>
                                <td>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Universal Login: " }
                                        </span>
                                        { "Create and customize a login page to which you can direct users to authenticate." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Custom Domains: " }
                                        </span>
                                        { "Create a custom domain to maintain a consistent experience for your users." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Email Templates: " }
                                        </span>
                                        { "Use templates to create welcome, password reset, and account verification email-based workflows." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Email Provider: " }
                                        </span>
                                        { "Designate and configure your custom email provider information." }
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-text-weight-600">{ "Security" }</span>
                                </td>
                                <td>
                                    <div>
                                        { "Configure extra layers of security by enabling shields that protect your users against different types of attacks and user access anomalies." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Attack Protection: " }
                                        </span>
                                        { "Manage settings for bot, IP throttling, brute-force, and breached password attacks." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Multi-factor Auth: " }
                                        </span>
                                        { "Require additional factors during the login process to prevent unauthorized access." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Monitoring: " }
                                        </span>
                                        { "Monitor threat intelligence events with one of our data visualization and alerting integrations." }
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-text-weight-600">{ "Actions" }</span>
                                </td>
                                <td>
                                    <div>
                                        { "Configure flows such as login, machine-to-machine, user registration, and password resets. Create and manage customized actions used in flows." }
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-text-weight-600">{ "Auth Pipeline" }</span>
                                </td>
                                <td>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Rules: " }
                                        </span>
                                        { "Configure custom JavaScript snippets that are executed in Auth0 as part of each user authentication transaction. You can call external APIs, filter which users can log in to your application, use an AllowList, configure geolocated access, and so on." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Hooks: " }
                                        </span>
                                        { "Customize the behavior of Auth0 when you use Database Connections by configuring Node.js code that is executed against extensibility points (which are comparable to webhooks that come with a server)." }
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-text-weight-600">{ "Monitoring" }</span>
                                </td>
                                <td>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Logs: " }
                                        </span>
                                        { "View log data of actions taken in the dashboard by administrators and user logins." }
                                    </div>
                                    <div>
                                        <span
                                            class="td-text-weight-600 uk-margin-small-right"
                                        >
                                            { "Streams: " }
                                        </span>
                                        { "Create and manage log event streaming to external data analysis services." }
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-text-weight-600">{ "Marketplace" }</span>
                                </td>
                                <td>
                                    <div>
                                        { "Explore integrations that help your business do more with Auth0." }
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-text-weight-600">{ "Extentions" }</span>
                                </td>
                                <td>
                                    <div>
                                        { "Extend the Auth0 platform with official and third-party add-ons." }
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-text-weight-600">{ "Settings" }</span>
                                </td>
                                <td>
                                    <div>
                                        { "Configure your tenants, manage your Auth0 subscription and payment options, control your tenant administrators and other user roles. Manage other tenant settings related to your custom domains, signing keys, and other advanced settings." }
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-text-weight-600">{ "Get Support" }</span>
                                </td>
                                <td>
                                    <div>
                                        { "Go to our Support Center. If your plan does not have access to support services, see the Auth0 Community." }
                                    </div>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Manage account settings" }</h1>
                    <p>
                        { "On the top left, you can see your tenant's name and icon, and a little arrow. This arrow displays a dropdown menu that you can use to configure different aspects of your account:" }
                    </p>
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <span
                                class="td-text-weight-600 uk-margin-small-right"
                            >
                                { "Settings: " }
                            </span>
                            { "Configure several aspects of your tenant." }
                        </li>
                        <li>
                            <span
                                class="td-text-weight-600 uk-margin-small-right"
                            >
                                { "Invite a member: " }
                            </span>
                            { "Add an additional user as an administrator or other role to your tenant configuration." }
                        </li>
                        <li>
                            <span
                                class="td-text-weight-600 uk-margin-small-right"
                            >
                                { "Quota Utilization: " }
                            </span>
                            { "See quota utilization information about your subscription and your tenants." }
                        </li>
                        <li>
                            <span
                                class="td-text-weight-600 uk-margin-small-right"
                            >
                                { "Create tenant: " }
                            </span>
                            { "Use this to create a new tenant." }
                        </li>
                        <li>
                            <span
                                class="td-text-weight-600 uk-margin-small-right"
                            >
                                { "Switch tenant: " }
                            </span>
                            { "If you have multiple tenants, use this option to switch between them. If you create an application for one tenant, you will not see it listed for another tenant." }
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <Link<Route>
                                to={Route::GetStartedHome}
                            >
                                { "Create Tenants" }
                            </Link<Route>>
                        </li>
                        <li>
                            <Link<Route>
                                to={Route::GetStartedHome}
                            >
                                { "Tenant Settings" }
                            </Link<Route>>
                        </li>
                        <li>
                            <Link<Route>
                                to={Route::GetStartedHome}
                            >
                                { "Application Settings" }
                            </Link<Route>>
                        </li>
                        <li>
                            <Link<Route>
                                to={Route::GetStartedHome}
                            >
                                { "Create Users" }
                            </Link<Route>>
                        </li>
                        <li>
                            <Link<Route>
                                to={Route::GetStartedHome}
                            >
                                { "API Settings" }
                            </Link<Route>>
                        </li>
                        <li>
                            <Link<Route>
                                to={Route::GetStartedHome}
                            >
                                { "Logs" }
                            </Link<Route>>
                        </li>
                    </ul>
                </div>
                
            </>
        }
    }
}
