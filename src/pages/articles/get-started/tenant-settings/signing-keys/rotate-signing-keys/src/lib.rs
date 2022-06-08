use yew::prelude::*;
use alert::Alert;

pub struct RotateSigningKeys {}

pub enum Msg {}

impl Component for RotateSigningKeys {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        RotateSigningKeys {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Rotate Signing Keys" }</h1>
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
                    <h1 class="td-text-size-large">{ "How it works" }</h1>
                    <p>{ "When a user signs in to your application, we create a token that contains information about the user and sign the token using its private key before we send it back to your application. Auth0 secures the private key, which is unique per tenant." }</p>
                    <p>{ "To verify that the token is valid and originated from Auth0, your application validates the token’s signature using the public key. We provide other application security key management capabilities through both our Dashboard and Management API." }</p>
                    <p>{ "Auth0 will notify you periodically if you haven't rotated your key in more than 365 days. Auth0 recommends that you rotate keys regularly to ensure that in case of a security breach you will be ready to take the actions needed." }</p>
                    <p>{ "Additional application signing certificates links are as follows:" }</p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "CER" }</li>
                        <li>{ "PEM" }</li>
                        <li>{ "raw PEM" }</li>
                        <li>{ "PB7" }</li>
                        <li>{ "Fingerprint" }</li>
                    </ul>
                    <Alert message={ String::from("We use the application signing key to sign assertions that are sent to applications. These assertions may include ID tokens, access tokens, SAML assertions, and WS-Fed assertions. Note that these keys are different from those used to sign interactions with connections, including signing SAML requests to Identity Providers (IdPs) and encrypting responses from IdPs.

                    By default, SAML assertions for IdP connections are signed, which we recommend. To get public keys you can use to configure the IdP, see SAML Identity Provider Configuration: Signed Assertions.") }/>
                    <p>{ "The rotation and revocation process supports your personal preferences and promotes a graceful transition for your application. If you prefer to update your application first, then rotate and revoke your key, you may do that. Alternatively, if you prefer to rotate your key, and then update your application and revoke your old key, you may also do that." }</p>
                    <p>{ "Available keys include:" }</p>
                    <ul class="uk-list uk-list-disc">
                        <li>
                            <b>{ "Currently used:" }</b>
                            <span class="td-margin-text">{ "Key that is currently being used to sign all new assertions." }</span>
                        </li>
                        <li>
                            <b>{ "Previously used:" }</b>
                            <span class="td-margin-text">{ "Key that was previously used, but has been rotated out. Assertions that were generated with this key will still work." }</span>
                        </li>
                        <li>
                            <b>{ "Next in queue:" }</b>
                            <span class="td-margin-text">{ "Key that is queued and will replace the current key when the application signing key is next rotated." }</span>
                        </li>
                    </ul>
                    <Alert message={ String::from("Always test signing key rotation on a development tenant before rotating application signing keys in production.") } />
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Limitations" }</h1>
                    <p>{ "Rotating your signing key will be subject to a smaller rate limit than other API endpoints. To learn more, read Management API Rate Limits." }</p>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Rotate Signing Keys" }</li>
                        <li>{ "Revoke Signing Keys" }</li>
                        <li>{ "View Signing Certificates" }</li>
                        <li>{ "Change Application Signing Algorithms" }</li>
                    </ul>
                </div>
                
            </>
        }
    }
}
