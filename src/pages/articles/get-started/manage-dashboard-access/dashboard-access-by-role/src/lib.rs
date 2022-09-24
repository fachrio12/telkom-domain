use yew::prelude::*;


pub struct DashboardAccessByRole {}

pub enum Msg {}

impl Component for DashboardAccessByRole {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DashboardAccessByRole {}
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
                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "Dashboard Access by Role" }</h1>

                <p>{ "As a tenant administrator, you can assign your colleagues roles to allow them to have limited access to Auth0 Dashboard features so they can complete their jobs without putting production applications at risk and complying with the least privilege principle." }
                </p>
               
                <div class="uk-margin-large-bottom">
                    <h1 class="td-text-size-large">{ "Dashboard roles" }</h1>
                    <p>
                        { "You can assign the following roles for the Auth0 Dashboard:" }
                       
                    </p>

                    <table class="uk-table uk-table-divider">
                    <thead>
                        <tr>
                            <th class="uk-text-emphasis">{ "Role" }</th>
                            <th class="uk-text-emphasis">{ "Permission" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="uk-text-emphasis">
                            {"Admin"}
                            </td>
                            <td>
                               {"Read and write access to all resources in the Auth0 Dashboard."}
                            </td>
                        </tr>
        
                        <tr>
                        <td class="uk-text-emphasis">
                        { "Editor - Specific Apps" }
                        </td>
                        <td>
                        { "Read and write access to specific applications only." }
                        </td>
                    </tr>
        
                    <tr>
                    <td class="uk-text-emphasis">
                    { "Editor - Connections" }
                    </td>
                    <td>
                       {"Read, write, and create access to all types of connections."}
                    </td>
                </tr>
        
                <tr>
                <td class="uk-text-emphasis">
                { "Editor - Users" }
                </td>
                <td>
                   {"User management operations (create, delete, block, unblock, reset MFA, reset password, update metadata, assign roles, etc.) and access to logs."}
                </td>
            </tr>
        
            <tr>
            <td class="uk-text-emphasis">
            { "Viewer - Users	" }
            </td>
            <td>
               {"Read-only access to users and logs."}
            </td>
             </tr>
                
             <tr>
             <td class="uk-text-emphasis">
             { "Viewer - Config Settings" }
             </td>
             <td>
                {"Read-only access to all configuration settings (applications, APIs, rules, security settings, etc.) except for sensitive information such as secrets, billing, users, and logs."}
             </td>
              </tr>
        
              <tr>
              <td class="uk-text-emphasis">
              { "Support Access" }
              </td>
              <td>
                 {"Access to tickets (submit, view, and update) and aggregated metrics"}
              </td>
               </tr>
        
                    </tbody>
                </table>
                
                </div>

                <div class="uk-margin-large-bottom">

                <div class="td-rounded-border">
                <h1 class="td-text-size-large">{ "Availability varies by Auth0 plan" }</h1>
                <p>{"Your Auth0 plan or custom agreement affects the availability of this feature. To learn more, read to Auth0's Pricing Page."}</p>
                </div>
                       
                 <p>{"Tenant members with less privileged roles will have a restricted Dashboard experience and they will have access only to the sections and actions they can perform. The following table shows the specific feature permissions for each role."}</p>


                 <table class="uk-table uk-table-divider">
                 <thead>
                     <tr>
                         <th class="uk-text-emphasis">{ "Dashboard Section" }</th>
                         <th class="uk-text-emphasis">{ "Subsection" }</th>
                         <th class="uk-text-emphasis">{ "Admin" }</th>
                         <th class="uk-text-emphasis">{ "Editor - Specific Apps" }</th>
                         <th class="uk-text-emphasis">{ "Editor - Connections" }</th>
                         <th class="uk-text-emphasis">{ "Editor - Users" }</th>
                         <th class="uk-text-emphasis">{ "Viewer - Users" }</th>
                         <th class="uk-text-emphasis">{ "Viewer - Config" }</th>
                         <th class="uk-text-emphasis">{ "Support Access" }</th>
                     </tr>
                 </thead>
                 <tbody>
                     <tr>
                         <td class="uk-text-emphasis">
                         {"Get Started"}
                         </td>
                         <td>
                            {""}
                         </td>
                         <td>
                            {"✅"}
                         </td>
                         <td>
                            {"❌"}
                         </td>
                         <td>
                            {"❌"}
                         </td>
                         <td>
                            {"❌"}
                         </td>
                         <td>
                            {"❌"}
                         </td>
                         <td>
                            {"❌"}
                         </td>
                         <td>
                            {"❌"}
                         </td>
                     </tr>
     
                     <tr>
                     <td class="uk-text-emphasis">
                     {"Activity"}
                     </td>
                     <td>
                        {"Stats"}
                     </td>
                     <td>
                        {"✅"}
                     </td>
                     <td>
                        {"❌"}
                     </td>
                     <td>
                        {"❌"}
                     </td>
                     <td>
                        {"✅"}
                     </td>
                     <td>
                        {"✅"}
                     </td>
                     <td>
                        {"❌"}
                     </td>
                     <td>
                        {"✅º"}
                     </td>
                 </tr>
 
     
                 <tr>
                 <td class="uk-text-emphasis">
                 {"Applications"}
                 </td>
                 <td>
                    {"Applications"}
                 </td>
                 <td>
                    {"✍"}
                 </td>
                 <td>
                    {"✍*¶"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
                 <td>
                    {"👁 ‡"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
             </tr>

     
             <tr>
                     <td class="uk-text-emphasis">
                     {""}
                     </td>
                     <td>
                        {"APIs"}
                     </td>
                     <td>
                        {"✍"}
                     </td>
                     <td>
                        {"❌"}
                     </td>
                     <td>
                     {"❌"}
                  </td>
                  <td>
                     {"❌"}
                  </td>
                  <td>
                     {"❌"}
                  </td>
                  <td>
                     {"👁 ‡"}
                  </td>
                  <td>
                     {"❌"}
                  </td>
                 </tr>
 
     
                 <tr>
                 <td class="uk-text-emphasis">
                 {""}
                 </td>
                 <td>
                    {"SSo integrations"}
                 </td>
                 <td>
                    {"✍"}
                 </td>
                 <td>
                    {"✍ *¶"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
                 <td>
                    {"👁 ‡"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
             </tr>

             
             <tr>
             <td class="uk-text-emphasis">
             {"Authentication"}
             </td>
             <td>
                {"Database"}
             </td>
             <td>
                {"✍"}
             </td>
             <td>
                {"❌ †"}
             </td>
             <td>
                {"✍"}
             </td>
             <td>
                    {"❌"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
                 <td>
                    {"👁 ‡"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
         </tr>

     
         <tr>
         <td class="uk-text-emphasis">
         {""}
         </td>
         <td>
            {"Social"}
         </td>
         <td>
                {"✍"}
             </td>
             <td>
                {"❌ †"}
             </td>
             <td>
                {"✍"}
             </td>
             <td>
                    {"❌"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
                 <td>
                    {"👁 ‡"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
             </tr>

             <tr>
             <td class="uk-text-emphasis">
             {""}
             </td>
             <td>
                {"Enterprise"}
             </td>
             <td>
                {"✍"}
             </td>
             <td>
                {"❌"}
             </td>
             <td>
                {"✍"}
             </td>
             <td>
                    {"❌"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
                 <td>
                    {"👁 ‡"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
            </tr>

            <tr>
             <td class="uk-text-emphasis">
             {""}
             </td>
             <td>
                {"Passwordless"}
             </td>
             <td>
                {"✍"}
             </td>
             <td>
                {"❌"}
             </td>
             <td>
                {"✍"}
             </td>
             <td>
                    {"❌"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
                 <td>
                    {"👁 ‡"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
            </tr>

            <tr>
             <td class="uk-text-emphasis">
             {"Organizations"}
             </td>
             <td>
                {"Organizations List"}
             </td>
             <td>
                {"✅"}
             </td>
             <td>
                {"❌"}
             </td>
             <td>
                {"❌"}
             </td>
             <td>
                    {"👁 "}
                 </td>
                 <td>
                    {"👁 "}
                 </td>
                 <td>
                    {"👁"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
         </tr>

         <tr>
         <td class="uk-text-emphasis">
         {""}
         </td>
         <td>
            {"Organizations Overview"}
         </td>
         <td>
            {"✅"}
         </td>
         <td>
            {"❌"}
         </td>
         <td>
            {"❌"}
         </td>
         <td>
                {"❌ "}
             </td>
             <td>
                {"❌ "}
             </td>
             <td>
                {"👁"}
             </td>
             <td>
                {"❌"}
             </td>
           </tr>
              
           <tr>
           <td class="uk-text-emphasis">
           {""}
           </td>
           <td>
              {"Organizations Members"}
           </td>
           <td>
              {"✅"}
           </td>
           <td>
              {"❌"}
           </td>
           <td>
              {"❌"}
           </td>
           <td>
                  {"👁 "}
               </td>
               <td>
                  {"👁 "}
               </td>
               <td>
                  {"❌"}
               </td>
               <td>
                  {"❌"}
               </td>
       </tr>

       <tr>
       <td class="uk-text-emphasis">
       {""}
       </td>
       <td>
          {"Organizations Invitations"}
       </td>
       <td>
          {"✅"}
       </td>
       <td>
          {"❌"}
       </td>
       <td>
          {"❌"}
       </td>
       <td>
              {"👁 "}
           </td>
           <td>
              {"❌"}
           </td>
           <td>
              {"❌"}
           </td>
           <td>
              {"❌"}
           </td>
            </tr>
             
            <tr>
            <td class="uk-text-emphasis">
            {""}
            </td>
            <td>
               {"Organizations Connections"}
            </td>
            <td>
               {"✅"}
            </td>
            <td>
               {"❌"}
            </td>
            <td>
               {"❌"}
            </td>
            <td>
                   {" ❌"}
                </td>
                <td>
                   {"❌"}
                </td>
                <td>
                   {"👁"}
                </td>
                <td>
                   {"❌"}
                </td>
           </tr>

           <tr>
           <td class="uk-text-emphasis">
           {"User Manegement"}
           </td>
           <td>
              {"Users"}
           </td>
           <td>
              {"✍"}
           </td>
           <td>
              {"❌"}
           </td>
           <td>
              {"❌"}
           </td>
           <td>
                  {" ✍"}
               </td>
               <td>
                  {"👁"}
               </td>
               <td>
                  {"❌"}
               </td>
               <td>
                  {"❌"}
               </td>
             </tr>

             <tr>
             <td class="uk-text-emphasis">
             {""}
             </td>
             <td>
                {"Roles"}
             </td>
             <td>
                {"✍"}
             </td>
             <td>
                {"❌"}
             </td>
             <td>
                {"❌"}
             </td>
             <td>
                    {" 👁"}
                 </td>
                 <td>
                    {"👁"}
                 </td>
                 <td>
                    {"👁"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
            </tr>

            <tr>
            <td class="uk-text-emphasis">
            {"Branding"}
            </td>
            <td>
               {"Universal Login"}
            </td>
            <td>
               {"✍"}
            </td>
            <td>
               {"❌"}
            </td>
            <td>
               {"❌"}
            </td>
            <td>
                   {" ❌"}
                </td>
                <td>
                   {"❌"}
                </td>
                <td>
                   {"👁"}
                </td>
                <td>
                   {"❌"}
                </td>
            </tr>

            <tr>
            <td class="uk-text-emphasis">
            {""}
            </td>
            <td>
               {"Custom domains"}
            </td>
            <td>
               {"✍"}
            </td>
            <td>
               {"❌"}
            </td>
            <td>
               {"❌"}
            </td>
            <td>
                   {" ❌"}
                </td>
                <td>
                   {"❌"}
                </td>
                <td>
                   {"👁"}
                </td>
                <td>
                   {"❌"}
                </td>
           </tr>

           <tr>
           <td class="uk-text-emphasis">
           {""}
           </td>
           <td>
              {"Email Templates"}
           </td>
           <td>
              {"✍"}
           </td>
           <td>
              {"❌"}
           </td>
           <td>
              {"❌"}
           </td>
           <td>
                  {" ❌"}
               </td>
               <td>
                  {"❌"}
               </td>
               <td>
                  {"👁"}
               </td>
               <td>
                  {"❌"}
               </td>
          </tr>

          <tr>
          <td class="uk-text-emphasis">
          {""}
          </td>
          <td>
             {"Email Providers"}
          </td>
          <td>
             {"✍"}
          </td>
          <td>
             {"❌"}
          </td>
          <td>
             {"❌"}
          </td>
          <td>
                 {" ❌"}
              </td>
              <td>
                 {"❌"}
              </td>
              <td>
                 {"👁‡"}
              </td>
              <td>
                 {"❌"}
              </td>
         </tr>

         <tr>
         <td class="uk-text-emphasis">
         {"Security"}
         </td>
         <td>
            {"Attack Protection"}
         </td>
         <td>
            {"✍"}
         </td>
         <td>
            {"❌"}
         </td>
         <td>
            {"❌"}
         </td>
         <td>
                {" ❌"}
             </td>
             <td>
                {"❌"}
             </td>
             <td>
                {"👁"}
             </td>
             <td>
                {"❌"}
             </td>
        </tr>

        <tr>
        <td class="uk-text-emphasis">
        {""}
        </td>
        <td>
           {"Multi-factor Auth"}
        </td>
        <td>
           {"✍"}
        </td>
        <td>
           {"❌"}
        </td>
        <td>
           {"❌"}
        </td>
        <td>
               {" ❌"}
            </td>
            <td>
               {"❌"}
            </td>
            <td>
               {"👁‡"}
            </td>
            <td>
               {"❌"}
            </td>
          </tr>

          <tr>
          <td class="uk-text-emphasis">
          {""}
          </td>
          <td>
             {"Monitoring"}
          </td>
          <td>
             {"✅"}
          </td>
          <td>
             {"❌"}
          </td>
          <td>
             {"❌"}
          </td>
          <td>
                 {" ❌"}
              </td>
              <td>
                 {"❌"}
              </td>
              <td>
                 {"❌"}
              </td>
              <td>
                 {"❌"}
              </td>
            </tr>

            <tr>
            <td class="uk-text-emphasis">
            {"Action"}
            </td>
            <td>
               {"Flows"}
            </td>
            <td>
               {"✍"}
            </td>
            <td>
               {"❌"}
            </td>
            <td>
               {"❌"}
            </td>
            <td>
                   {" ❌"}
                </td>
                <td>
                   {"❌"}
                </td>
                <td>
                   {"👁"}
                </td>
                <td>
                   {"❌"}
                </td>
           </tr>

           <tr>
           <td class="uk-text-emphasis">
           {""}
           </td>
           <td>
              {"Library"}
           </td>
           <td>
              {"✍"}
           </td>
           <td>
              {"❌"}
           </td>
           <td>
              {"❌"}
           </td>
           <td>
                  {" ❌"}
               </td>
               <td>
                  {"❌"}
               </td>
               <td>
                  {"👁"}
               </td>
               <td>
                  {"❌"}
               </td>
          </tr>

          <tr>
          <td class="uk-text-emphasis">
          {"Auth Pipeline"}
          </td>
          <td>
             {"Rules"}
          </td>
          <td>
             {"✍"}
          </td>
          <td>
             {"❌"}
          </td>
          <td>
             {"❌"}
          </td>
          <td>
                 {" ❌"}
              </td>
              <td>
                 {"❌"}
              </td>
              <td>
                 {"👁‡"}
              </td>
              <td>
                 {"❌"}
              </td>
         </tr>

         <tr>
         <td class="uk-text-emphasis">
         {""}
         </td>
         <td>
            {"Hooks"}
         </td>
         <td>
            {"✍"}
         </td>
         <td>
            {"❌"}
         </td>
         <td>
            {"❌"}
         </td>
         <td>
                {" ❌"}
             </td>
             <td>
                {"❌"}
             </td>
             <td>
                {"❌"}
             </td>
             <td>
                {"❌"}
             </td>
        </tr>

        <tr>
        <td class="uk-text-emphasis">
        {"Monitoring"}
        </td>
        <td>
           {"Logs"}
        </td>
        <td>
           {"✅"}
        </td>
        <td>
           {"❌"}
        </td>
        <td>
           {"❌"}
        </td>
        <td>
               {"👁§"}
            </td>
            <td>
               {"👁§"}
            </td>
            <td>
               {"❌"}
            </td>
            <td>
               {"❌"}
            </td>
           </tr>

           <tr>
           <td class="uk-text-emphasis">
           {""}
           </td>
           <td>
              {"Streams"}
           </td>
           <td>
              {"✍"}
           </td>
           <td>
              {"❌"}
           </td>
           <td>
              {"❌"}
           </td>
           <td>
                  {" ❌"}
               </td>
               <td>
                  {"❌"}
               </td>
               <td>
                  {"❌"}
               </td>
               <td>
                  {"❌"}
               </td>
               </tr>

               <tr>
               <td class="uk-text-emphasis">
               {"Marketplace"}
               </td>
               <td>
                  {""}
               </td>
               <td>
                  {"✍"}
               </td>
               <td>
                  {"👁"}
               </td>
               <td>
                  {"👁"}
               </td>
               <td>
                      {"👁"}
                   </td>
                   <td>
                      {"👁"}
                   </td>
                   <td>
                      {"👁"}
                   </td>
                   <td>
                      {"❌"}
                   </td>
                 </tr>

                 <tr>
                 <td class="uk-text-emphasis">
                 {"Extensions"}
                 </td>
                 <td>
                    {""}
                 </td>
                 <td>
                    {"✍"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
                 <td>
                    {"❌"}
                 </td>
                 <td>
                        {" ❌"}
                     </td>
                     <td>
                        {"❌"}
                     </td>
                     <td>
                        {"❌"}
                     </td>
                     <td>
                        {"❌"}
                     </td>
                    </tr>

                    <tr>
                    <td class="uk-text-emphasis">
                    {"Settings"}
                    </td>
                    <td>
                       {"General"}
                    </td>
                    <td>
                       {"✍"}
                    </td>
                    <td>
                       {"❌"}
                    </td>
                    <td>
                       {"❌"}
                    </td>
                    <td>
                           {" ❌"}
                        </td>
                        <td>
                           {"❌"}
                        </td>
                        <td>
                           {"👁"}
                        </td>
                        <td>
                           {"❌"}
                        </td>
                          </tr>

                          <tr>
                          <td class="uk-text-emphasis">
                          {""}
                          </td>
                          <td>
                             {"Subscription"}
                          </td>
                          <td>
                             {"✍"}
                          </td>
                          <td>
                             {"❌"}
                          </td>
                          <td>
                             {"❌"}
                          </td>
                          <td>
                                 {" ❌"}
                              </td>
                              <td>
                                 {"❌"}
                              </td>
                              <td>
                                 {"❌"}
                              </td>
                              <td>
                                 {"❌"}
                              </td>
                               </tr>

                               <tr>
                               <td class="uk-text-emphasis">
                               {""}
                               </td>
                               <td>
                                  {"Tenant Members"}
                               </td>
                               <td>
                                  {"✍"}
                               </td>
                               <td>
                                  {"❌"}
                               </td>
                               <td>
                                  {"❌"}
                               </td>
                               <td>
                                      {" ❌"}
                                   </td>
                                   <td>
                                      {"❌"}
                                   </td>
                                   <td>
                                      {"❌"}
                                   </td>
                                   <td>
                                      {"❌"}
                                   </td>
                                  </tr>

                                  <tr>
                                  <td class="uk-text-emphasis">
                                  {""}
                                  </td>
                                  <td>
                                     {"Signing Keys"}
                                  </td>
                                  <td>
                                     {"✍"}
                                  </td>
                                  <td>
                                     {"❌"}
                                  </td>
                                  <td>
                                     {"❌"}
                                  </td>
                                  <td>
                                         {" ❌"}
                                      </td>
                                      <td>
                                         {"❌"}
                                      </td>
                                      <td>
                                         {"❌"}
                                      </td>
                                      <td>
                                         {"❌"}
                                      </td>
                                    </tr>

                                    <tr>
                                    <td class="uk-text-emphasis">
                                    {""}
                                    </td>
                                    <td>
                                       {"Advanced"}
                                    </td>
                                    <td>
                                       {"✍"}
                                    </td>
                                    <td>
                                       {"❌"}
                                    </td>
                                    <td>
                                       {"❌"}
                                    </td>
                                    <td>
                                           {" ❌"}
                                        </td>
                                        <td>
                                           {"❌"}
                                        </td>
                                        <td>
                                           {"👁"}
                                        </td>
                                        <td>
                                           {"❌"}
                                        </td>
                                   </tr>

                                   <tr>
                                   <td class="uk-text-emphasis">
                                   {"Get Support"}
                                   </td>
                                   <td>
                                      {"Support Tickets-Create"}
                                   </td>
                                   <td>
                                      {"✅"}
                                   </td>
                                   <td>
                                      {"✅"}
                                   </td>
                                   <td>
                                      {"✅"}
                                   </td>
                                   <td>
                                          {"✅ "}
                                       </td>
                                       <td>
                                          {"✅"}
                                       </td>
                                       <td>
                                          {"✅"}
                                       </td>
                                       <td>
                                          {"✅"}
                                       </td>
                                        </tr>


                                        <tr>
                                        <td class="uk-text-emphasis">
                                        {""}
                                        </td>
                                        <td>
                                           {"Support Tickets-View"}
                                        </td>
                                        <td>
                                           {"✅"}
                                        </td>
                                        <td>
                                           {"❌"}
                                        </td>
                                        <td>
                                           {"❌"}
                                        </td>
                                        <td>
                                               {"❌"}
                                            </td>
                                            <td>
                                               {"❌"}
                                            </td>
                                            <td>
                                               {"❌"}
                                            </td>
                                            <td>
                                               {"❌"}
                                            </td>
                                             </tr>

                                             <tr>
                                             <td class="uk-text-emphasis">
                                             {""}
                                             </td>
                                             <td>
                                                {"Quota Reports"}
                                             </td>
                                             <td>
                                                {"✅"}
                                             </td>
                                             <td>
                                                {"❌"}
                                             </td>
                                             <td>
                                                {"❌"}
                                             </td>
                                             <td>
                                                    {"❌"}
                                                 </td>
                                                 <td>
                                                    {"❌"}
                                                 </td>
                                                 <td>
                                                    {"❌"}
                                                 </td>
                                                 <td>
                                                    {"❌"}
                                                 </td>
                                                  </tr>

                                                  <tr>
                                                  <td class="uk-text-emphasis">
                                                  {""}
                                                  </td>
                                                  <td>
                                                     {"Usage Reports"}
                                                  </td>
                                                  <td>
                                                     {"✅"}
                                                  </td>
                                                  <td>
                                                     {"✅"}
                                                  </td>
                                                  <td>
                                                     {"✅"}
                                                  </td>
                                                  <td>
                                                         {"✅"}
                                                      </td>
                                                      <td>
                                                         {"✅"}
                                                      </td>
                                                      <td>
                                                         {"✅"}
                                                      </td>
                                                      <td>
                                                         {"✅"}
                                                      </td>
                                                       </tr>

                                                       <tr>
                                                       <td class="uk-text-emphasis">
                                                       {""}
                                                       </td>
                                                       <td>
                                                          {"Compliance"}
                                                       </td>
                                                       <td>
                                                     {"✅"}
                                                  </td>
                                                  <td>
                                                     {"✅"}
                                                  </td>
                                                  <td>
                                                     {"✅"}
                                                  </td>
                                                  <td>
                                                         {"✅"}
                                                      </td>
                                                      <td>
                                                         {"✅"}
                                                      </td>
                                                      <td>
                                                         {"✅"}
                                                      </td>
                                                      <td>
                                                         {"✅"}
                                                      </td>
                                                            </tr>

                                                            <tr>
                                                            <td class="uk-text-emphasis">
                                                            {""}
                                                            </td>
                                                            <td>
                                                               {"Tenant Tagging"}
                                                            </td>
                                                            <td>
                                                               {"✍"}
                                                            </td>
                                                            <td>
                                                               {"❌"}
                                                            </td>
                                                            <td>
                                                               {"❌"}
                                                            </td>
                                                            <td>
                                                                   {"❌"}
                                                                </td>
                                                                <td>
                                                                   {"❌"}
                                                                </td>
                                                                <td>
                                                                   {"❌"}
                                                                </td>
                                                                <td>
                                                                   {"❌"}
                                                                </td>
                                                                 </tr>

                                                                 <tr>
                                                                 <td class="uk-text-emphasis">
                                                                 {""}
                                                                 </td>
                                                                 <td>
                                                                    {"Production Checks"}
                                                                 </td>
                                                                 <td>
                                                                    {"✅"}
                                                                 </td>
                                                                 <td>
                                                                    {"❌"}
                                                                 </td>
                                                                 <td>
                                                                    {"❌"}
                                                                 </td>
                                                                 <td>
                                                                        {"❌"}
                                                                     </td>
                                                                     <td>
                                                                        {"❌"}
                                                                     </td>
                                                                     <td>
                                                                        {"❌"}
                                                                     </td>
                                                                     <td>
                                                                        {"❌"}
                                                                     </td>
                                                                      </tr>

                                                                      <tr>
                                                                      <td class="uk-text-emphasis">
                                                                      {""}
                                                                      </td>
                                                                      <td>
                                                                         {"Notifications"}
                                                                      </td>
                                                                      <td>
                                                                         {"✅"}
                                                                      </td>
                                                                      <td>
                                                                         {"❌"}
                                                                      </td>
                                                                      <td>
                                                                         {"❌"}
                                                                      </td>
                                                                      <td>
                                                                             {"❌"}
                                                                          </td>
                                                                          <td>
                                                                             {"❌"}
                                                                          </td>
                                                                          <td>
                                                                             {"❌"}
                                                                          </td>
                                                                          <td>
                                                                             {"❌"}
                                                                          </td>
                                                                           </tr>
     
           </tbody>
             </table>

                </div>

                <div class="uk-margin-large-bottom">
                <h4 class="td-margin-text">
                { "Legend" }
               </h4>
                <table class="uk-table uk-table-divider">
              
                <thead>
                    <tr>
                        <th class="uk-text-emphasis">{ "Symbol" }</th>
                        <th class="uk-text-emphasis">{ "Permission" }</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                        {"✅"}
                        </td>
                        <td>
                           {"Access"}
                        </td>
                    </tr>
    
                    <tr>
                    <td>
                    { "❌" }
                    </td>
                    <td>
                    { "No access" }
                    </td>
                </tr>
    
                <tr>
                <td>
                { "✍" }
                </td>
                <td>
                   {"Edit"}
                </td>
            </tr>
    
            <tr>
            <td>
            { "👁 " }
            </td>
            <td>
               {"View"}
            </td>
        </tr>
    
       
                </tbody>
            </table>
              </div>

              <div class="uk-margin-large-bottom">
              
              <table class="uk-table uk-table-divider">
            
              <thead>
                  <tr>
                      <th class="uk-text-emphasis">{ "Footnote" }</th>
                      <th class="uk-text-emphasis">{ "Description" }</th>
                  </tr>
              </thead>
              <tbody>
                  <tr>
                      <td>
                      {"*"}
                      </td>
                      <td>
                         {"Specific application"}
                      </td>
                  </tr>
  
                  <tr>
                  <td>
                  { "‡" }
                  </td>
                  <td>
                  { "Except secrets" }
                  </td>
              </tr>
  
              <tr>
              <td>
              { "†" }
              </td>
              <td>
                 {"Previously available for Application Admin role but removed from Editor Specific Apps role"}
              </td>
          </tr>
  
          <tr>
          <td>
          { "¶" }
          </td>
          <td>
             {"Update only"}
          </td>
          </tr>

          <tr>
          <td>
          { "§" }
          </td>
          <td>
             {"	User events"}
          </td>
           </tr>
                  
           <tr>
           <td>
           { "º" }
           </td>
           <td>
              {"Metrics only"}
           </td>
       </tr>
  
     
              </tbody>
          </table>
            </div>


            <div class="uk-margin-large-bottom">
            <h1 class="td-text-size-large">{ "Log events available to user roles" }</h1>
            <p>
                { "Logs can contain sensitive data, such as secrets, PII, etc. It is important not to disclose sensitive data to users whose role does not require that information. However, the " }
                <b class="td-margin-text">
                { "Editor - Users" }
               </b>
               {"or"}
               <b class="td-margin-text">
               { "Viewer - Users" }
              </b>
              {"roles need to have some access to logs to identity user issues. For example, finding out if the user signed up correctly, if the user was blocked, etc."}
            </p>

            <p>
            {"We allow the "}
            <b class="td-margin-text">
            { "Editor - Users" }
           </b>
           {"and"}
           <b class="td-margin-text">
           { "Viewer - Users" }
          </b>
           {"with access to a limited set of log types, that are connected to user events. The log events in the list provide the necessary information about user actions but do not disclose sensitive information about other parts of the tenant configuration. To learn more, read Log Event Type Codes."}
            </p>
             
            <img
            class="uk-margin-top uk-margin-bottom"
            src="/assets/manage-dashboard-access/gambar.jpg"
            />
           </div>

           <div class="uk-margin-large-bottom">
           <h1 class="td-text-size-large">{ "Limitations" }</h1>
           <ul class="uk-list uk-list-disc">
                        <li>{ "Users with" }
                        <b class="td-margin-text">
                        { "Admin" }
                        </b>
                        {"role can invite"}
                        <b class="td-margin-text">
                        { "Editor - Specific Apps" }
                        </b>
                        {"users to one application at a time. To work around this, the"}
                        <b class="td-margin-text">
                        { "Admin" }
                        </b>
                        {" user can edit their role to assign multiple applications after the user accepts the invitation."}
                        </li>

                        <li>{ "The " }
                        <b class="td-margin-text">
                        { "Viewer - Users " }
                        </b>
                        {"and"}
                        <b class="td-margin-text">
                        { " Editor - Users " }
                        </b>
                        {"roles don't have access to the Users'"}
                        <b class="td-margin-text">
                        { "Devices " }
                        </b>
                        {" and"}
                        <b class="td-margin-text">
                        { "Authorized Apps" }
                        </b>
                        {"sections."}
                        </li>

                        <li>{ "The New Activity Page is visible to " }
                        <b class="td-margin-text">
                        { "Admin" }
                        </b>
                        {"and"}
                        <b class="td-margin-text">
                        { "Support Access" }
                        </b>
                        {"users only."}
                        <b class="td-margin-text">
                        { "Editor - Users" }
                        </b>
                        {"and"}
                        <b class="td-margin-text">
                        { "Viewer - Users" }
                        </b>
                        {"can access daily activity (such as logins or signups) through the Auth0 Management API."}
                        </li>
                        
                    </ul>
           </div>

                  <div
                      class="uk-margin-large-bottom"
                  >
                     <h1 class="td-text-size-large">{ "Private Cloud requirements" }</h1>
                     <p>
                     {"The"}
                     <b class="td-margin-text">
                        { "Editor - Users" }
                        </b>
                        {"and the"}
                        <b class="td-margin-text">
                        { " Viewer - Users " }
                        </b>
                        {"roles require that User Search v3 and Logs Search v3 are enabled in your environment. If your environments don’t support these versions, these two roles are unavailable."}
                     </p>
                     </div>
         
                <div
                    class="uk-margin-large-bottom"
                >
                    <h1 class="td-text-size-large">{ "Learn more" }</h1>
                    <ul class="uk-list uk-list-disc">
                        <li>{ "Add Tenant Members" }</li>
                        <li>{ "Edit Tenant Members" }</li>
                        <li>{ "Remove Tenant Members" }</li>
                        <li>{ "Troubleshoot Role-Based Access Control and Authorization" }</li>
                        <li>{ "Check Error Messages" }</li>
                        
                    </ul>
                </div>
                
            </>
        }
    }
}
