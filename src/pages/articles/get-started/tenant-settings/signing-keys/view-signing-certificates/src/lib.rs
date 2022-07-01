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
                <p>
                    { "You can revoke your tenant's application or API signing key using the Auth0 Dashboard or the Management API. The signing key is used to sign ID tokens, access tokens, SAML assertions, and WS-Fed assertions sent to your application or API. To learn more, read Signing Keys." }
                </p>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Prerequisites" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>
                            { "Before you can revoke a previously-used signing key, you must first have rotated the key. To learn more, read Rotate Signing Keys, or see the" }
                            <b class="td-margin-text">{ "Rotate and revoke signing key" }</b>
                            { "section below." }
                        </li>
                        <li>
                            { "Make sure you have updated your application or API with the new key before you revoke the previous key." }
                        </li>
                    </ul>
                    <Alert message={String::from("You cannot reuse a signing key after revocation, so be sure that you want to revoke it.")} />
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Use the Dashboard" }</h1>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Revoke previously used signing key" }</h1>
                    <ul
                        class="uk-list uk-list-decimal"
                        style="display: inline-block;"
                    >
                        <li>
                            { "Go to" }
                            <ul
                                class="uk-breadcrumb uk-margin-remove-top td-margin-text"
                                style="display: inline-block;"
                            >
                                <li>{ "Dashboard" }</li>
                                <li>{ "Settings" }</li>
                                <li>{ "Signing Keys" }</li>
                            </ul>
                        </li>
                        <li>
                             { "In the List of Valid Keys section, locate the Previously Used key, select the more options (...) menu, and select Revoke Key. The List of Valid Keys section lists the current signing key being used by your tenant, plus the next signing key that will be assigned should you choose to rotate your signing keys. If you have previously rotated signing keys, this section also lists the previously-used keys. The List of Revoked Keys section lists the last three revoked keys for your tenant." }
                        </li>
                        <li>
                            { "Select Revoke to confirm." }
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Rotate and revoke signing key" }</h1>
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
                                <li>{ "Settings" }</li>
                                <li>{ "Signing Keys" }</li>
                            </ul>
                        </li>
                        <li>
                             { "In the Rotation Settings section, locate the Rotate & Revoke Signing Key section, and select Rotate & Revoke Key." }
                        </li>
                        <li>
                            { "Select Rotate & Revoke to confirm." }
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Use the Management API" }</h1>
                    <Alert message={String::from("You can only revoke the previously used signing key.")}/>
                    <ul
                        class="uk-list uk-list-decimal"
                    >
                        <li>
                            { "To get a list of the signing keys, make a GET call to the Get all Application Signing Keys endpoint." }
                        </li>
                        <li>
                            { "Make a" }
                            <span class="td-label-code td-margin-text">{ "PUT" }</span>
                            { "call to the Revoke an Application Signing Key by its Key ID endpoint. Be sure to replace the" }
                            <span class="td-label-code td-margin-text">{ "YOUR_KEY_ID" }</span>
                            { "and" }
                            <span class="td-label-code td-margin-text">{ "MGMT_API_ACCESS_TOKEN" }</span>
                            { "placeholder values with your signing key's ID and Management API access token, respectively." }
                        </li>
                    </ul>
                    <p style="color: red;">{ "TASK: CREATE CODE BOX NODE JS" }</p>
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
                                    { "ID of the signing key to be revoked. To learn how to find your signing key ID, see Locate JSON Web Key Sets." }
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <span class="td-label-code td-margin-text">{ "MGMT_API_ACCESS_TOKEN" }</span>
                                </td>
                                <td>
                                    { "Access Token for the Management API with the scope" }
                                    { "and" }
                                    <span class="td-label-code td-margin-text">{ "update:signing_keys" }</span>
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
                        <li>{ "Rotate Signing Keys" }</li>
                        <li>{ "View Signing Certificates" }</li>
                        <li>{ "Change Application Signing Algorithms" }</li>
                    </ul>
                </div>
                
            </>
        }
    }
}
