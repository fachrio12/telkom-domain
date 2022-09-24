use yew::prelude::*;



pub struct ManageDashboardAccessHome {}

pub enum Msg {}

impl Component for ManageDashboardAccessHome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ManageDashboardAccessHome {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Manage Dashboard Access" }</h1>
                <p>
                    { "As an Auth0 tenant administrator, you are responsible for all activities that occur under your Auth0 account and tenants including managing your tenant members. You can add, change, and remove tenant members (dashboard users) in the Auth0 Dashboard. Auth0 recommends that you periodically review the list of Auth0 Dashboard tenant members with access to your Auth0 tenant and make sure that:" }
                </p>
                
                <div
                class="uk-margin-large-bottom"
              >
               
                <ul class="uk-list uk-list-disc">
                    <li>{ "Each person has a legitimate need for tenant member access." }</li>
                    <li>{ "Members are registered with a company account." }</li>
                    <li>{ "Former employees no longer have access." }</li>
                    <li>{ "There's more than one Dashboard admin." }</li>
                </ul>
            </div>

            <div class="uk-margin-large-bottom">
            <table class="uk-table uk-table-divider">
            <thead>
                <tr>
                    <th class="uk-text-emphasis">{ "Read..." }</th>
                    <th class="uk-text-emphasis">{ "To learn..." }</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>
                    {"Dashboard Access by Role"}
                    </td>
                    <td>
                       {"About tenant member roles and Auth0 Dashboard feature access."}
                    </td>
                </tr>

                <tr>
                <td>
                { "Add Tenant Members" }
                </td>
                <td>
                { "How to add tenant members to access the Auth0 Dashboard." }
                </td>
            </tr>

            <tr>
            <td>
            { "Edit Tenant Members" }
            </td>
            <td>
               {"How to edit tenant member roles."}
            </td>
        </tr>

        <tr>
        <td>
        { "Manage Support Center Users" }
        </td>
        <td>
           {"How to allow team members to access Auth0 Support Center only."}
        </td>
    </tr>

    <tr>
    <td>
    { "Remove Tenant Members" }
    </td>
    <td>
       {"How to revoke a tenant member's access to the Auth0 Dashboard."}
    </td>
     </tr>
        
     <tr>
     <td>
     { "Add Multi-factor Authentication (MFA) for Auth0 Dashboard Access" }
     </td>
     <td>
        {"How Auth0 Dashboard users can implement multi-factor authentication (MFA)."}
     </td>
      </tr>

      <tr>
      <td>
      { "Update Dashboard User Email Addresses" }
      </td>
      <td>
         {"How to update a tenant member's email address."}
      </td>
       </tr>

            </tbody>
        </table>
            </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <p>{"Here are some examples of users who may have special access requirements:"}</p>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Support specialists who need to troubleshoot login issues for your app end users." }</li>
                        <li>{ "Support/IT specialists who need to assign roles and permissions to end users." }</li>
                        <li>{ "Developers who need to troubleshoot their applications in production environments. " }</li>
                        <li>{ "Product managers who need to analyze their applications' configuration and usage." }</li>
                        <li>{ "Developers who need to configure settings for their own applications." }</li>
                        <li>{ "Support/IT specialists that need to create connections for their customers in a B2B use case." }</li>
                    </ul>
                </div>



                
            </>
        }
    }
}
