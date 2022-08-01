use yew::prelude::*;


pub struct ApplicationsInDomainHome {}

pub enum Msg {}

impl Component for ApplicationsInDomainHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ApplicationsInDomainHome {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Applications In Domain" }</h1>
                <p>
                    { "The term application or app in Auth0 does not imply any particular implementation characteristics. For example, it could be a native app that executes on a mobile device, a single-page application that executes on a browser, or a regular web application that executes on a server." }
                </p>
                <p>
                    { "Auth0 categorizes apps based on these characteristics:" }
                </p>

                <ul class="uk-list uk-list-disc uk-margin-medium-bottom">
                    <li>
                        <p>
                            <b style="margin-right: 5px;">{ "Application type:" }</b>
                            { "To add authentication to your application, you must register it in the Auth0 Dashboard and select from one of the following application types:" }
                        </p>
                        <ul class="uk-list uk-list-disc uk-margin-medium-bottom">
                            <li>
                                <b style="margin-right: 5px;">{ "Regular web application:" }</b>
                                { "Traditional web apps that perform most of their application logic on the server (such as Express.js or ASP.NET). To learn how to set up a regular web application, read Register Regular Web Applications." }
                            </li>
                            <li>
                                <b style="margin-right: 5px;">{ "Single page web application (SPA):" }</b>
                                { "JavaScript apps that perform most of their user interface logic in a web browser, communicating with a web server primarily using APIs (such as AngularJS + Node.js or React). To learn how to set up a Single-page web application, read Register Single-Page Web Applications." }
                            </li>
                            <li>
                                <b style="margin-right: 5px;">{ "Native application:" }</b>
                                { "Mobile or Desktop applications that run natively on a device (such as iOS or Android). To learn how to set up a native application, read Register Native Applications." }
                            </li>
                            <li>
                                <b style="margin-right: 5px;">{ "Machine to machine (M2M) application:" }</b>
                                { "Non-interactive applications, such as command-line tools, daemons, IoT devices, or services running on your backend. Typically, you use this option if you have a service that requires access to an API. To learn how to set up a native application, read Register Machine-to-Machine Applications." }
                            </li>
                        </ul>
                    </li>
                    <li>
                        <b style="margin-right: 5px;">{ "Credential security:" }</b>
                        { "Credential security: According to the OAuth 2.0 spec, apps can be classified as either public or confidential; confidential apps can hold credentials securely, while public apps cannot. To learn more, read Confidential and Public Applications." }
                    </li>
                    <li>
                        <b style="margin-right: 5px;">{ "Ownership:" }</b>
                        { "Whether an app is classified as first- or third-party depends on app ownership and control. First-party apps are controlled by the same organization or person that owns the Auth0 domain. Third-party apps enable external parties or partners to securely access protected resources behind your API. To learn more, read First-Party and Third-Party Applications." }
                    </li>
                </ul>


                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Manage applications settings" }</h1>
                    <p>{ "You register applications in Dashboard > Applications > Applications. In addition to setting up applications in the Dashboard, you can also set up applications programmatically as described in the OpenID Connect (OIDC) Dynamic Client Registration 1.0 specification." }</p>
                    <p>
                        { "You can set up a more complex configuration that allows users to log in differently for different apps. To learn more, read Multi-Tenant Application Best Practices and Create Multiple Tenants." }
                    </p>
                    <p>
                        { "By default, Auth0 enables all connections associated with your tenant when you create a new application. To change this, update application connections in the Application Settings in the Dashboard." }
                    </p>
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Monitor applications" }</h1>
                    <p>
                        { "You can monitor apps and perform end-to-end testing using your own tests. Auth0 stores log data including Dashboard administrator actions, successful and failed user authentications, and password change requests. You can use Auth0 Extensions to export your log data and use tools like Sumo Logic, Splunk, or Loggly to analyze and store your log data." }
                    </p>
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Remove applications" }</h1>
                    <p>
                        { "You can remove an application using the Dashboard or the Management API." }
                    </p>
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Manage client secrets" }</h1>
                    <p>
                        { "A client secret is a secret known only to your application and the authorization server. It protects your resources by only granting tokens to authorized requestors." }
                    </p>
                    <p>
                        { "Protect your client secrets and never include them in mobile or browser-based apps. If your client secret is ever compromised, you should rotate to a new one and update all authorized apps with the new client secret." }
                    </p>
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Grant types" }</h1>
                    <p>
                        { "Auth0 provides many different authentication and authorization grant types or flows and allows you to indicate which grant types are appropriate based on the grant_types property of your Auth0-registered app. To learn more, read Application Grant Types." }
                    </p>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Application Settings" }</li>
                        <li>{ "Confidential and Public Applications" }</li>
                        <li>{ "First-Party and Third-Party Applications" }</li>
                        <li>{ "Application Grant Types" }</li>
                        <li>{ "Subdomain URL Placehorders" }</li>
                        <li>{ "Dynamic Application Registration" }</li>
                    </ul>
                </div>



                
            </>
        }
    }
}
