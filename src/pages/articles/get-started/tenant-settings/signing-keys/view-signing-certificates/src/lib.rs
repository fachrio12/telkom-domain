use yew::prelude::*;
use alert::Alert;

pub struct ViewSigningCertificates {}

pub enum Msg {}

impl Component for ViewSigningCertificates {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ViewSigningCertificates {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "View Signing Certificates" }</h1>
                <p
                    class="uk-margin-large-bottom"
                >
                    { "You can view your tenant's application client secrets and signing keys using the Auth0 Dashboard or the Management API. The application signing key is used to sign ID tokens, access tokens, SAML assertions, and WS-Fed assertions sent to your application. These keys are different from those used to sign interactions with connections, including signing SAML requests to identity providers (IdPs) and encrypting responses from IdPs. By default, SAML assertions for IdP connections are signed, which we recommend. To learn more, read SAML Identity Provider Configuration Settings." }
                </p>
                
                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Use the Dashboard" }</h1>
                    <h1 class="td-text-size-large">{ "Tenant settings" }</h1>
                    <ul
                        class="uk-list uk-list-decimal"
                    >
                        <li>
                            <p>
                                { "Go to" }
                                <ul
                                    class="uk-breadcrumb uk-margin-remove-top td-margin-text"
                                    style="display: inline-block;"
                                >
                                    <li>{ "Dashboard" }</li>
                                    <li>{ "Settings" }</li>
                                    <li>{ "Signing Keys" }</li>
                                </ul>
                            </p>
                            <img
                                class="uk-margin-top uk-margin-bottom"
                                src="/assets/view-signing-certificates/dashboard-tenant-settings-signing-keys.png"
                            />
                        </li>
                        <li>
                            <p>
                                { "In the Rotation Settings section, locate List of Valid Keys and List of Revoked Keys." }
                            </p>
                            <ul class="uk-list uk-list-square">
                                <li>
                                    { "The" }
                                    <b class="td-margin-text">{ "List of Valid Keys" }</b>
                                    { "section lists the current signing key being used by your tenant, plus the next signing key that will be assigned should you choose to rotate your signing keys. If you have previously rotated signing keys, this section also lists the previously-used keys." }
                                </li>
                                <li>
                                    { "The" }
                                    <b class="td-margin-text">{ "List of Revoked Keys" }</b>
                                    { "section lists the last three revoked keys for your tenant." }
                                </li>
                            </ul>
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Application settings" }</h1>
                    <p>{ "You can also view an application's signing key and/or client secret depending on the type of signing algorithm you are using." }</p>
                    <h1 class="td-text-size-large">{ "If using the RS256 signing algorithm" }</h1>
                    <ul
                        class="uk-list uk-list-decimal"
                    >
                        <li>
                            { "Go to" }
                            <ul
                                class="uk-breadcrumb uk-margin-remove-top td-margin-text"
                                style="display: inline-block;"
                            >
                                <li>{ "Dashboard" }</li>
                                <li>{ "Applications" }</li>
                            </ul>
                            { ", and select the name of the application to view." }
                        </li>
                        <li>
                            { "Scroll to the bottom of the" }
                            <b class="td-margin-text">{ "Settings" }</b>
                            { "tab, and select" }
                            <b class="td-margin-text">{ "Advanced Settings" }</b>
                        </li>
                        <li>
                            <p>
                                { "Go to the" }
                                <b class="td-margin-text">{ "Certificates" }</b>
                                { "tab and locate the" }
                                <b class="td-margin-text">{ "Signing Certificates" }</b>
                                { "field." }
                            </p>
                            <img
                                class="uk-margin-top uk-margin-bottom"
                                src="/assets/view-signing-certificates/dashboard-applications-applications-settings-advanced-certificates.png"
                            />
                        </li>
                    </ul>

                    <h1 class="td-text-size-large">{ "If using the HS256 signing algorithm" }</h1>
                    <ul
                        class="uk-list uk-list-decimal"
                    >
                        <li>
                            { "Go to" }
                            <ul
                                class="uk-breadcrumb uk-margin-remove-top td-margin-text"
                                style="display: inline-block;"
                            >
                                <li>{ "Dashboard" }</li>
                                <li>{ "Applications" }</li>
                            </ul>
                            { ", and select the name of the application to view." }
                        </li>
                        <li>
                            <p>
                                { "Under Basic Information, locate the Client Secret field for the client secret." }
                                <b class="td-margin-text">{ "Basic Information" }</b>
                                { ", locate the" }
                                <b class="td-margin-text">{ "Client Secret" }</b>
                                { "field for the client secret." }
                            </p>
                            <img
                                class="uk-margin-top uk-margin-bottom"
                                src="/assets/view-signing-certificates/dashboard-applications-applications-settings-basic-information.png"
                            />
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Use the Management API" }</h1>
                    <h1 class="td-text-size-large">{ "Get all signing keys" }</h1>
                    <p>
                        { "Make a" }
                        <span class="td-label-code td-margin-text">{ "GET" }</span>
                        { "call to the" }
                        <span class="td-label-code td-margin-text">{ "/signing_keys/get_signing_keys" }</span>
                        { "endpoint. Be sure to replace the" }
                        <span class="td-label-code td-margin-text">{ "MGMT_API_ACCESS_TOKEN" }</span>
                        { "placeholder value with your Management API Access Token." }
                    </p>
                    <p style="color: red;">{ "CODE BOX" }</p>
                    <table class="uk-table uk-table-divider">
                        <thead>
                            <tr>
                                <th class="uk-text-emphasis">{ "Value" }</th>
                                <th class="uk-text-emphasis">{ "Description" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>
                                    <span class="td-label-code td-margin-text">{ "MGMT_API_ACCESS_TOKEN" }</span>
                                </td>
                                <td>
                                    { "Access Token for the Management API with the scope" }
                                    <span class="td-label-code td-margin-text">{ "read:signing_keys" }</span>
                                    { "." }
                                </td>
                            </tr>
                        </tbody>
                    </table>

                    <h1 class="td-text-size-large">{ "Get a single signing keys" }</h1>
                    <p>
                        { "Make a" }
                        <span class="td-label-code td-margin-text">{ "GET" }</span>
                        { "call to the" }
                        <span class="td-label-code td-margin-text">{ "/signing_keys/get_signing_keys" }</span>
                        { "endpoint. Be sure to replace the" }
                        <span class="td-label-code td-margin-text">{ "YOUR_KEY_ID" }</span>
                        { "and" }
                        <span class="td-label-code td-margin-text">{ "MGMT_API_ACCESS_TOKEN" }</span>
                        { "placeholder values with your signing key's ID and Management API Access Token, respectively." }
                    </p>
                    <p style="color: red;">{ "CODE BOX" }</p>
                    <table class="uk-table uk-table-divider">
                        <thead>
                            <tr>
                                <th class="uk-text-emphasis">{ "Value" }</th>
                                <th class="uk-text-emphasis">{ "Description" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>
                                    <span class="td-label-code td-margin-text">{ "YOUR_KEY_ID" }</span>
                                </td>
                                <td>
                                    { "ID of the signing key to be viewed. To learn how to find your signing key ID, see Locate JSON Web Key Sets." }
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-label-code td-margin-text">{ "MGMT_API_ACCESS_TOKEN" }</span>
                                </td>
                                <td>
                                    { "Access Token for the Management API with the scope" }
                                    <span class="td-label-code td-margin-text">{ "read:signing_keys" }</span>
                                    { "." }
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Revoke Signing Keys" }</li>
                        <li>{ "Rotate Signing Keys" }</li>
                        <li>{ "Signing Alo" }</li>
                        <li>{ "Change Application Signing Algorithms" }</li>
                    </ul>
                </div>
                
            </>
        }
    }
}
