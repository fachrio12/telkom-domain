use yew::prelude::*;
use alert::Alert;

pub struct SigningKeysHome {}

pub enum Msg {}

impl Component for SigningKeysHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SigningKeysHome {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Signing Keys" }</h1>
                <p>
                    { "When you select our recommended signing algorithm (RS256), Auth0 uses public-key cryptography to establish trust with your applications. In more general terms, we use a signing key that consists of a public and private key pair." }
                </p>
                <p class="uk-margin-large-bottom">
                    { "Signing keys are used to sign ID tokens, access tokens, SAML assertions, and WS-Fed assertions sent to your application or API. The signing key is a JSON web key (JWK) that contains a well-known public key used to validate the signature of a signed JSON web token (JWT). A JSON web key set (JWKS) is a set of keys containing the public keys used to verify any JWT issued by the authorization server and signed using the RS256 signing algorithm. The service may only use one JWK for validating web tokens, however, the JWKS may contain multiple keys if the service rotated signing certificates." }
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
