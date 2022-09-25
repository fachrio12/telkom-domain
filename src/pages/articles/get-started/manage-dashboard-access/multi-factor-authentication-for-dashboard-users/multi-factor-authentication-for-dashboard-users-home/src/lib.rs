use yew::prelude::*;
 use alert::Alert;
// // use alert2::Alert2;


pub struct MultiFactorAuthenticationForDashboardUsersHome {}

pub enum Msg {}

impl Component for MultiFactorAuthenticationForDashboardUsersHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MultiFactorAuthenticationForDashboardUsersHome {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ " Multi Factor Authentication For Dashboard Users" }</h1>
                <p>
                    { "Multi-factor authentication (MFA) adds an additional level of security to an Auth0 account. When users with MFA enabled log into the Auth0 Dashboard, Auth0 prompts for their credentials plus an additional piece of identifying information. This ensures that only valid users can access their accounts, even if a bad actor has compromised a username and password." }
                </p>

                <p>
                {"Any Dashboard user can self-enroll in MFA in Account Settings. The MFA indicator in the Auth0 Dashboard > Settings > Tenant Members list identifies whether a user has enabled MFA for their account."}
                </p>
                
                <p>
                {"Auth0 supports these authentication factors for Dashboard users:"}
                </p>
                
                <div
                class="uk-margin-large-bottom"
              >
               
                <ul class="uk-list uk-list-disc">
                    <li>
                    <b class="td-margin-text">{ "WebAuthn with FIDO security keys:" }</b>
                    { "WebAuthn roaming authenticators are removable and cross-platform, like a Yubikey, and can be used on multiple devices. To authenticate with a roaming authenticator, users must connect the authenticator to their device (through USB, NFC, or Bluetooth) and provide proof of presence (by touching it, for example)." }
                    </li>

                    <li>
                    <b class="td-margin-text">{ "WebAuthn with device biometrics: " }</b>
                    { "WebAuthn platform authenticators are attached to a device and work on that device only. Examples are the MacBook Touch Bar, Windows Hello, iOS Touch ID or Face ID, and Android fingerprint or face recognition. Because they work on the attached device only, a user must have at least one other factor enrolled in their profile before enrolling device biometrics." }
                    </li>

                    <li>
                    <b class="td-margin-text">{ "Push notification via Guardian:  " }</b>
                    { "Sends push notifications to a user's pre-registered device, typically a mobile phone or tablet. The user can immediately allow or deny account access with a button press. The push factor is available with the Guardian mobile app for iOS and Android." }
                    </li>

                    <li>
                    <b class="td-margin-text">{ "One-time passwords (OTP):" }</b>
                    { " Allows a user to use an authenticator app (such as Google Authenticator) on their personal device. The app generates an OTP that changes over time and can be entered as a second factor to validate the account." }
                    </li>

                    <li>
                    <b class="td-margin-text">{ "SMS notification:" }</b>
                    { " Sends a one-time code over SMS. Auth0 then prompts the user to enter this code before they can complete authentication" }
                    </li>
                </ul>

                <p>
                {"To learn how to enroll in Dashboard MFA, read Add Multi-Factor Authentication for Auth0 Dashboard Access."}
                </p>

                <Alert message={String::from("Auth0 recommends WebAuthn factors as the most secure and usable authentication methods. To learn more, read FIDO Authentication with WebAuthn.")} />
                //alert 2 message
            </div>

          
            <div
            class="uk-margin-large-bottom"
        >
            <h1 class="td-text-size-large">{ "Learn more" }</h1>
            <ul class="uk-list uk-list-disc">
                <li>{ "Add Multi-Factor Authentication for Auth0 Dashboard Access" }</li>
                <li>{ "Remove or Change Dashboard Multi-Factor Authentication" }</li>
                <li>{ "Update Dashboard User Email Addresses" }</li>
                <li>{ "Reset Account Passwords" }</li>
                <li>{ "Troubleshoot Multi-Factor Authentication Issues" }</li>
                </ul>
        </div>


                
            </>
        }
    }
}
