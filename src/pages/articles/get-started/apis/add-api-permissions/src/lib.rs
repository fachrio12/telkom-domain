use yew::prelude::*;
use alert::Alert;
use alert2::Alert2;

pub struct AddApiPermissions {}

pub enum Msg {}

impl Component for AddApiPermissions {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        AddApiPermissions{}
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
                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Add API Permissions" }</h1>

                <p>{ "You can add permissions to an API using the Auth0 Dashboard or the Management API." }
                
                </p>

                <Alert2 message={String::from("By default, any user of any application can ask for any permission defined here. You can implement access policies to limit this behavior via rules.")}/>

                <div class="uk-margin-large-bottom">
                    <h1 class="td-text-size-large">{ "Use The Dashboard" }</h1>
                    
                <ul class="uk-list uk-list-decimal">
                        <li>
                            
                            { "Go to Dashboard > Applications > APIs and click the name of the API to view." }

                            <img
                            class="uk-margin-top uk-margin-bottom"
                            src="/assets/add-api-permissions/dashboard-apis-list (1).png"
                        />
                        </li>
               
                <li>
                { "Go to the" }
                <b class="td-margin-text">{ "Permissions" }</b>
                {"tab and enter a permission name and description for the permission you want to add. Be sure not to use any reserved permission names (see Reserved names section)."}

                <img
                class="uk-margin-top uk-margin-bottom"
                src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
                />

                </li>

                <li>
                            
                { "Click" }
                <b class="td-margin-text">{ "Add" }</b>

               </li>

                </ul>

                </div>

                <div class="uk-margin-large-bottom">
                    <h1 class="td-text-size-large">{ "Use The Manegement API" }</h1>
                    
                    <Alert message={String::from("Patching the permissions with an empty object removes the permissions completely.")}/>

                    <p>
                    {"Make a PATCH call to the Update Resource Server endpoint. Be sure to replace API_ID, MGMT_API_ACCESS_TOKEN, PERMISSION_NAME, and PERMISSION_DESC placeholder values with your API ID, Management API Access Token, permission name(s), and permission description(s), respectively. Be sure not to use any reserved permission names (see Reserved names section)."}
                    </p>
                    // coding box is to be added here

                </div>

                // <div class="uk-overflow-auto uk-height-medium">
                //    <nav class="uk-navbar-container">
                //        <div class="uk-navbar-left">
                //        <ul class="uk-navbar-nav">
                //        <li>
                //            <a href="#">{"cURL"}</a>
                //            <div class="uk-dropbar uk-dropbar-top"  uk-drop="stretch: x; mode: click">
                //            <img
                //            class="uk-margin-top uk-margin-bottom"
                //            src="/assets/add-api-permissions/dashboard-apis-list (1).png"
                //             />
                //        </div>
                //        </li>
                //        <li>
                //            <a href="#">{"C#"}</a>
                //            <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
                //            <img
                //            class="uk-margin-top uk-margin-bottom"
                //            src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
                //            />
                //            </div>
                //        </li>
                //        <li>
                //            <a href="#">{"Go"}</a>
                //            <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
                //               <img
                //                  class="uk-margin-top uk-margin-bottom"
                //                  src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
                //                />
                //            </div>
                //        </li>
                //        <li>
                //        <a href="#">{"Java"}</a>
                //        <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
                //           <img
                //              class="uk-margin-top uk-margin-bottom"
                //              src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
                //            />
                //        </div>
                //    </li>
                //    <li>
                //        <a href="#">{"Node.JS"}</a>
                //          <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
                //              <img
                //                 class="uk-margin-top uk-margin-bottom"
                //                 src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
                //               />
                //          </div>
                //    </li>
                //    <li>
                //          <a href="#">{"Obj-c"}</a>
                //             <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
                //                 <img
                //                     class="uk-margin-top uk-margin-bottom"
                //                      src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
                //                     />
                //             </div>
                //    </li>
                //    <li>
                   
                //    <a href="#">
                //    <span uk-icon="icon: more; ratio: 1.5;"></span>
                //    </a>
                //    <div uk-dropdown="mode: click">
                //    <ul class="uk-nav uk-dropdown-nav">
                       
                //        <li><a href="#">{"PHP"}</a>
                //        <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
                //        <img
                //            class="uk-margin-top uk-margin-bottom"
                //             src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
                //            />
                //    </div>
                //        </li>
                //        <li><a href="#">{"Phyton"}</a>
                //        <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
                //                 <img
                //                     class="uk-margin-top uk-margin-bottom"
                //                      src="/assets/add-api-permissions/dashboard-apis-list (1).png"
                //                     />
                //             </div>
                //        </li>
                //        <li><a href="#">{"Ruby"}</a>
                //        <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
                //        <img
                //            class="uk-margin-top uk-margin-bottom"
                //             src="/assets/add-api-permissions/dashboard-apis-list (1).png"
                //            />
                //    </div>
                //        </li>
                //        <li><a href="#">{"Swift"}</a>
                //        <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
                //        <img
                //            class="uk-margin-top uk-margin-bottom"
                //             src="/assets/add-api-permissions/dashboard-apis-list (1).png"
                //            />
                //    </div>
                //        </li>
                       
                //    </ul>
                // </div>
                //    </li>
                //      </ul>
                // </div>
                //   </nav>
                //        </div>

                <div class="uk-margin-large-bottom">
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
                            <span class="td-label-code td-margin-text">{ "API_ID" }</span>
                        </td>
                        <td>
                           {"ID of the API for which you want to delete permissions."}
                        </td>
                    </tr>

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

                <tr>
                <td>
                    <span class="td-label-code td-margin-text">{ "PERMISSIONS_NAME" }</span>
                </td>
                <td>
                   {"Name(s) of the permission(s) you want to keep for the specified API."}
                </td>
            </tr>

            <tr>
            <td>
                <span class="td-label-code td-margin-text">{ "PERMISSIONS_DESC" }</span>
            </td>
            <td>
               {"User-friendly description(s) of the permission(s) you want to keep for the specified API."}
            </td>
        </tr>

                </tbody>
            </table>
                </div>

                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Reserved names" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "address" }</li>
                        <li>{ "created_at" }</li>
                        <li>{ "email" }</li>
                        <li>{ "email_verified" }</li>
                        <li>{ "family_name" }</li>
                        <li>{ "given_name" }</li>
                        <li>{ "identities" }</li>
                        <li>{ "name" }</li>
                        <li>{ "nickname" }</li>
                        <li>{ "offline_access" }</li>
                        <li>{ "openid" }</li>
                        <li>{ "phone" }</li>
                        <li>{ "picture" }</li>
                        <li>{ "profile" }</li>
                    </ul>
                </div>
         
                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Customize Consent Prompts" }</li>
                        <li>{ "Configure Logical API for Multiple APIs" }</li>
                        <li>{ "Role-Based Access Control" }</li>
                        <li>{ "Enable Role-Based Access Control for APIs" }</li>
                        <li>{ "Check API Calls" }</li>
                        </ul>
                </div>
                
            </>
        }
    }
}
