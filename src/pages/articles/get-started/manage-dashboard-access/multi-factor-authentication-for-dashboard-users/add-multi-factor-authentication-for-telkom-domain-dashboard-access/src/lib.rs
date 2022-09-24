use yew::prelude::*;
// use alert::Alert;


pub struct AddMultiFactorAuthenticationForTelkomDomainDashboardAccess {}

pub enum Msg {}

impl Component for AddMultiFactorAuthenticationForTelkomDomainDashboardAccess {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        AddMultiFactorAuthenticationForTelkomDomainDashboardAccess {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ " Add Multi-Factor Authentication for Auth0 Dashboard Access" }</h1>
                <p>
                    { "Each Auth0 Dashboard user should self-enroll in multi-factor authentication (MFA). You can enroll in most factors in Account Settings. Device biometrics, however, require progressive enrollment." }
                </p>

                // <Alert message={String::from("Auth0 recommends WebAuthn factors as the most secure and usable authentication methods. To learn more, read  FIDO Authentication with WebAuthn.")} />
                //tambahkan alert 2

                
                <div
                class="uk-margin-large-bottom"
              >
              <h1 class="uk-heading-small uk-margin-medium-bottom">{ " Add MFA" }</h1>
              <p>
              {"To self-enroll for MFA, each Dashboard user must follow these steps:"}
              </p>
                <ul class="uk-list uk-list-decimal">
                    <li>
                    
                    { "Click on your username in the top right corner of the Dashboard and click Account Settings." }
                    </li>

                    

                    <li>
                    
                    { "Sends push notifications to a user's pre-registered device, typically a mobile phone or tablet. The user can immediately allow or deny account access with a button press. The push factor is available with the Guardian mobile app for iOS and Android." }
                    <b class="td-margin-text">{ "+ ADD " }</b>
                    {"in that row."}

                    <img
                    class="uk-margin-top uk-margin-bottom"
                    src="/assets/add-multi-factor-authentication-for-telkom-domain-dashboard-access/gambar1.png"
                    />
                    </li>

                    <li>
                    
                    { " Follow the on-screen instructions to complete the enrollment." }
                    </li>

                   
                </ul>

              </div>

              <div
              class="uk-margin-large-bottom">
              
              <h1 class="td-text-size-large">{ "Device biometrics" }</h1>
              <p>
              {"WebAuthn with device biometrics is the only method that you can't add on the Account Settings page. Instead, Auth0 progressively enrolls all of your WebAuthn-capable devices. Auth0 prompts you to enroll those devices after you enroll any other MFA method. These prompts recur each time you log in to Auth0 Dashboard."}
              </p>
              <img
                    class="uk-margin-top uk-margin-bottom"
                    src="/assets/add-multi-factor-authentication-for-telkom-domain-dashboard-access/gambar2.png"
                    />
                    <p>
                    {"As part of the enrollment, Auth0 prompts you to name your devices. This makes it easy to manage them from the Account Settings page."}
                    </p>

                    <p>
                    {"Browsers with Javascript disabled or without WebAuthn platform authenticator support can't enroll or authenticate with device biometrics. The latest versions of popular browsers and operating systems provide support for WebAuthn with Security Keys. To learn more, read the browser support section on webauthn.me."}
                    </p>
              </div>


              <div
              class="uk-margin-large-bottom">
              
              <h1 class="td-text-size-large">{ "Recovery codes" }</h1>
              <p>
              {"Immediately after successfully enabling two-factor authentication, Auth0 prompts you to copy a recovery code. If you lose access to all your enrolled factors, you can use this recovery code to log in to your account. Auth0 recommends copying and printing recovery codes or storing them in a safe place, such as a password manager. "}
              </p>
             
                   <p>
                    {"If you lose the recovery codes or just want to generate new ones, you can do so from Account Settings."}
                    </p>
              </div>


              <div
              class="uk-margin-large-bottom">
              
              <h1 class="td-text-size-large">{ "Log in to the Dashboard with MFA enabled" }</h1>
              <p>
              {"Logging in with MFA enabled is only slightly different than a normal login. When you enter admin account credentials, a second prompt appears, depending on which type of MFA factors you’ve enabled.  "}
              </p>
             
                   <p>
                    {"If a user loses access to a primary factor, they can click on "}
                    <b class="td-margin-text">{ "Select Another Method  " }</b>
                    {"and try with any of the other factors, including recovery codes. This is why it's so important to enroll in multiple methods to prevent being locked out of your account."}
                    </p>

                    <p>
                    {"After you successfully add your second authentication factor and you log in from a new device that supports WebAuthn, you see a prompt to "}
                    <q>{"Log in Faster on this Device."}</q>
                    {"This lets you use that device for multi-factor authentication the next time."}
                    </p>
              </div>

          
            <div
            class="uk-margin-large-bottom"
        >
            <h1 class="td-text-size-large">{ "Learn more" }</h1>
            <ul class="uk-list uk-list-disc">
                <li>{ "Multi-Factor Authentication for Dashboard Users" }</li>
                <li>{ "Remove or Change Dashboard Multi-Factor Authentication" }</li>
                <li>{ "Troubleshoot Multi-Factor Authentication Issues" }</li>

                </ul>
        </div>


                
            </>
        }
    }
}
