use yew::prelude::*;
pub struct DeleteApiPermissions {}

pub enum Msg {}

impl Component for DeleteApiPermissions {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DeleteApiPermissions{}
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
                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Delete API Permissions" }</h1>

                <p>{ "You can delete permissions from an API using the Auth0 Dashboard or the Management API." }
                
                </p>

                

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
                {" tab and click the trashcan icon next to the permission you want to remove and confirm."}

                <img
                class="uk-margin-top uk-margin-bottom"
                src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
                />

                </li>

            </ul>

                </div>

                <div class="uk-margin-large-bottom">
                    <h1 class="td-text-size-large">{ "Use The Manegement API" }</h1>
                    

                    <p>
                    {"Make a PATCH call to the Update Resource Server endpoint that includes all permissions you want to keep and excludes all permissions you want to delete. Replace API_ID, MGMT_API_ACCESS_TOKEN, PERMISSION_NAME, and PERMISSION_DESC placeholder values with your API ID, Management API Access Token, permission name(s), and permission description(s), respectively."}
                    </p>

                   // coding box is to be added here

                </div>

        //         <div class="uk-overflow-auto uk-height-medium">
        //            <nav class="uk-navbar-container">
        //                <div class="uk-navbar-left">
        //                <ul class="uk-navbar-nav">
        //                <li>
        //                    <a href="#">{"cURL"}</a>
        //                    <div class="uk-dropbar uk-dropbar-top"  uk-drop="stretch: x; mode: click">
        //                    <img
        //                    class="uk-margin-top uk-margin-bottom"
        //                    src="/assets/add-api-permissions/dashboard-apis-list (1).png"
        //                     />
        //                </div>
        //                </li>
        //                <li>
        //                    <a href="#">{"C#"}</a>
        //                    <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
        //                    <img
        //                    class="uk-margin-top uk-margin-bottom"
        //                    src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
        //                    />
        //                    </div>
        //                </li>
        //                <li>
        //                    <a href="#">{"Go"}</a>
        //                    <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
        //                       <img
        //                          class="uk-margin-top uk-margin-bottom"
        //                          src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
        //                        />
        //                    </div>
        //                </li>
        //                <li>
        //                <a href="#">{"Java"}</a>
        //                <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
        //                   <img
        //                      class="uk-margin-top uk-margin-bottom"
        //                      src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
        //                    />
        //                </div>
        //            </li>
        //            <li>
        //                <a href="#">{"Node.JS"}</a>
        //                  <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
        //                      <img
        //                         class="uk-margin-top uk-margin-bottom"
        //                         src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
        //                       />
        //                  </div>
        //            </li>
        //            <li>
        //                  <a href="#">{"Obj-c"}</a>
        //                     <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
        //                         <img
        //                             class="uk-margin-top uk-margin-bottom"
        //                              src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
        //                             />
        //                     </div>
        //            </li>

        //            <li>
                   
        //               <a href="#">
        //                 <span uk-icon="icon: more; ratio: 1.5;"></span>
        //              </a>
        //                 <div uk-dropdown="mode: click">
        //                    <ul class="uk-nav uk-dropdown-nav">
                       
        //                        <li><a href="#">{"PHP"}</a>
        //                             <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
        //                                 <img
        //                                   class="uk-margin-top uk-margin-bottom"
        //                                       src="/assets/add-api-permissions/dashboard-applications-apis-permissions.png"
        //                                 />
        //                            </div>
        //                        </li>
        //                        <li><a href="#">{"Phyton"}</a>
        //                              <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
        //                                  <img
        //                                       class="uk-margin-top uk-margin-bottom"
        //                                       src="/assets/add-api-permissions/dashboard-apis-list (1).png"
        //                                  />
        //                              </div>
        //                        </li>
        //                        <li><a href="#">{"Ruby"}</a>
        //                              <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
        //                                  <img
        //                                      class="uk-margin-top uk-margin-bottom"
        //                                          src="/assets/add-api-permissions/dashboard-apis-list (1).png"
        //                                  />
        //                              </div>
        //                        </li>
        //                        <li><a href="#">{"Swift"}</a>
        //                              <div class="uk-dropbar uk-dropbar-top" uk-drop="stretch: x; mode: click">
        //                                  <img
        //                                      class="uk-margin-top uk-margin-bottom"
        //                                          src="/assets/add-api-permissions/dashboard-apis-list (1).png"
        //                                  />
        //                              </div>
        //                        </li>
                       
        //                     </ul>
        //                 </div>
        //              </li>
        //           </ul>
        //       </div>
        //    </nav>
        // </div>

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
                    <span class="td-label-code td-margin-text">{ "update:resource_servers." }</span>
                  
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
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Customize Consent Prompts" }</li>
                        <li>{ "Configure Logical API for Multiple APIs" }</li>
                        <li>{ "Role-Based Access Control" }</li>
                        <li>{ "Enable Role-Based Access Control for APIs" }</li>
                    </ul>
                </div>
                
            </>
        }
    }
}
