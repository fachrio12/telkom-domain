use yew::prelude::*;
use alert::Alert;

pub struct RevokeSigningKeys {}

pub enum Msg {}

impl Component for RevokeSigningKeys {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        RevokeSigningKeys {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Revoke Signing Keys" }</h1>
                <p>
                    { "You can manually rotate a signing key periodically to change the JSON web key (JWK) key used by applications and APIs to validate tokens. If your application or API does not allow for this key change, and it attempts to use an expired signing key to verify a token, the authentication request will fail." }
                </p>
                <Alert message={ String::from("Auth0 recommends that you execute signing key rotation on a development tenant first, then verify that your applications and APIs still work as expected. After you verify that everything is working properly, perform the same signing key rotation on your production tenant.") } />
                <p>
                    { "Although Auth0 signs with only one signing key at a time, your tenant's OpenID Connect (OIDC) discovery document always contains multiple keys. The OIDC discovery document will always include both the current key and the next key, and it may also include the previous key if the previous key has not yet been revoked. To provide a seamless experience in case of an emergency, your application should be able to use any of the keys specified in the document. To learn more about OpenID Connect discovery documents, read Locate JSON Web Key Sets." }
                </p>
                <Alert message={ "To allow you time to update your application with the new signing key, all tokens signed with the previous key will still be valid until you revoke the previous key. To learn more, read Revoke Signing Keys." } />
                <p class="uk-margin-large-bottom">
                    { "You can rotate your tenant's application signing key using the Auth0 Dashboard or the Auth0 Management API." }
                </p>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Use the Dashboard" }</h1>
                    <ul class="uk-list uk-list-decimal">
                        <li>
                            { "Go to Dashboard > Settings > Signing Keys." }
                            <img
                                class="uk-margin-top uk-margin-bottom"
                                src="/assets/rotate-signing-keys/dashboard-tenant-settings-signing-keys.png"
                            />
                        </li>
                        <li>{ "Under Rotation Settings, locate Rotate Signing Key, and select Rotate Key." }</li>
                        <li>
                            <p>{ "Click Rotate to confirm." }</p>
                            <img
                                class="uk-margin-top uk-margin-bottom"
                                src="/assets/rotate-signing-keys/dashboard-tenant-settings-signing-keys-rotation-confirm.png"
                            />
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Use the Management API" }</h1>
                    <ul class="uk-list uk-list-decimal">
                        <li>
                            { "To get a list of the signing keys, make a" }
                            <span class="td-label-code td-margin-text">{ "GET" }</span>
                            { "call to the Get all Application Signing Keys endpoint." }
                        </li>
                        <li>
                            <p>
                                { "To rotate the signing key, make a" }
                                <span class="td-label-code td-margin-text">{ "POST" }</span>
                                { "call to the Rotate the Application Signing Key endpoint. Be sure to replace the" }
                                <span class="td-label-code td-margin-text">{ "MGMT_API_ACCESS_TOKEN" }</span>
                                { "placeholder value with your Management API access token." }
                            </p>
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
                                            <span class="td-label-code td-margin-text">{ "MGMT_API_ACCESS_TOKEN" }</span>
                                        </td>
                                        <td>
                                            { "Access Token for the Management API with the scopes" }
                                            <span class="td-label-code td-margin-text">{ "create:signing_keys" }</span>
                                            { "and" }
                                            <span class="td-label-code td-margin-text">{ "update:signing_keys" }</span>
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </li>
                    </ul>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Key rotation impact" }</h1>
                    <h1 class="td-text-size-big">{ "APIs and API gateways accepting access tokens" }</h1>
                    <p class="uk-margin-large-bottom">
                        { "Most middleware and API gateways leverage the JSON web key set (JWKS) endpoint to retrieve the current and future signing keys at a certain interval. If your middleware and/or API gateways" }
                        <b>{ "do not" }</b>
                        { "support this endpoint and require you to manually configure a" }
                        <span class="td-label-code td-margin-text">{ "*.cer" }</span>
                        { "file, you will need to coordinate the signing key rotation in Auth0 with the reconfiguration of your middleware and gateways." }
                    </p>
                    <h1 class="td-text-size-big">{ "Regular web applications" }</h1>
                    <p>
                        { "When rotating the signing key in Auth0, you will need to coordinate the reconfiguration of your applications which leverage WS-Fed or SAML. This typically happens when you upload the new public certificate or reconfigure the application by entering the WS-Fed/SAML metadata URL. This will change the JWKS key, which is used by applications to validate tokens, make sure your implementation does not assume JWKS keys don’t change." }
                    </p>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Revoke Signing Keys" }</li>
                        <li>{ "Locate JSON Web Key Sets" }</li>
                        <li>{ "Change Application Signing Algorithms" }</li>
                        <li>{ "View Signing Certificates" }</li>
                    </ul>
                </div>
                
            </>
        }
    }
}
