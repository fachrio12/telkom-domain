use yew::prelude::*;
// use alert::Alert;


pub struct RemoveOrChangeDashboardMultiFactorAuthentication {}

pub enum Msg {}

impl Component for RemoveOrChangeDashboardMultiFactorAuthentication {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        RemoveOrChangeDashboardMultiFactorAuthentication {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Remove Or Change Dashboard Multi-Factor Authentication" }</h1>
                <p>
                    { "Dashboard users can remove or change multi-factor authentication (MFA) factors that are no longer available or wanted. To learn more about MFA for Dashboard users, read Multi-Factor Authentication for Dashboard Users." }
                </p>

                //tambahkan alert
                //tambahkan alert 2
                
                <div
                class="uk-margin-large-bottom"
              >
              <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Remove or change an MFA factor from the Dashboard" }</h1>
              //tambahkan alert
              <p>{"Dashboard users who can log in with their current MFA factors can follow these steps:"}</p>
                <ul class="uk-list uk-list-decimal">
                    <li>{ "In the top right corner of the Dashboard, click your user name and click Account Settings." }
                    <img
                    class="uk-margin-top uk-margin-bottom"
                    src="/assets/remove-or-change-dmfa/gambar1.png"
                    />
                    </li>
                    <li>{ "Find the new authentication factor you want to use and click " }
                    <b class="td-margin-text">{ "+ ADD " }</b>
                    {" in that row. Follow the on-screen instructions to complete the enrollment."}
                    </li>
                    <li>{ "Still in Account Settings, find the authentication factor you want to stop using and click" }
                    <b class="td-margin-text">{ "REMOVE" }</b>
                    </li>
                    <li>{ "Click" }
                    <b class="td-margin-text">{ "YES" }</b>
                    {"to confirm the removal. "}
                    </li>
                    <li>{ "Auth0 prompts you to authenticate with your current (old) factors. After a successful authentication, Auth0 removes the factor." }
                    </li>
                </ul>
            </div>

            <div
            class="uk-margin-large-bottom"
          >
          <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Remove or change a lost MFA factor " }</h1>
          //tambahkan alert
          <p>{"Dashboard users who"}
          <b class="td-margin-text">{ "can't" }</b>
          {" log in with their current MFA factors can follow these steps:"}
          </p>
            <ul class="uk-list uk-list-decimal">
                <li>{ "Attempt to log in to the Dashboard. Auth0 prompts you to authenticate with your current factors." }
                <img
                class="uk-margin-top uk-margin-bottom"
                src="/assets/remove-or-change-dmfa/gambar2.png"
                />
                </li>
                <li>{ "When Auth0 asks for the device or credentials you’ve lost, click on " }
                <b class="td-margin-text">{ " Try another method." }</b>
                <img
                class="uk-margin-top uk-margin-bottom"
                src="/assets/remove-or-change-dmfa/gambar3.png"
                />
                </li>
                <li>{ "In the Other Methods box, click a different method to authenticate." }
               </li>
                <li>{ "Log in to access the Dashboard." }
                 </li>
                <li>{ "In the top right corner of the Dashboard, click your user name and click Account Settings." }
                <img
                class="uk-margin-top uk-margin-bottom"
                src="/assets/remove-or-change-dmfa/gambar4.png"
                />
                </li>
                <li>{ "Find the authentication method you can no longer use and click" }
                <b class="td-margin-text">{ "REMOVE" }</b>
                 </li>
                 <li>{ "Click " }
                 <b class="td-margin-text">{ "YES" }</b>
                 {"to confirm the removal. "}
                  </li>
                  <li>{ "Auth0 prompts you to authenticate using your current factors again. Repeat steps 2-4 to verify your identity. Auth0 removes the lost factor. " }
                 </li>
            </ul>
        </div>


                <div
                   class="uk-margin-large-bottom"
                 >
                 <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Get help from Auth0 support " }</h1>
                 <p>
                 {"If you are locked out and don’t have access to any of your enabled MFA factors, there is no guarantee that you can regain access to your account. Another administrator must file an Auth0 support ticket on your behalf. In some cases, Auth0 can verify the request and proceed with an MFA reset. However, we may not be able to confirm account ownership. This is why it’s so important to enable multiple and varied factors."}
                 </p>

                 <p>
                 {"Auth0 support does not reset"}
                 <b class="td-margin-text">{ "end-user" }</b>
                 {"accounts. You are responsible for accounts that access your applications and APIs. To learn about end-user accounts, read Manage Users."}
                 </p>
                 </div>

                 <div
                 class="uk-margin-large-bottom"
             >
                 <h1 class="td-text-size-large">{ "Learn more" }</h1>
                 <ul class="uk-list uk-list-disc">
                     <li>{ "Multi-Factor Authentication for Dashboard Users" }</li>
                     <li>{ "Add Multi-Factor Authentication for Auth0 Dashboard Access" }</li>
                     <li>{ "Troubleshoot Multi-Factor Authentication Issues" }</li>
     
                     </ul>
             </div>



                
            </>
        }
    }
}
