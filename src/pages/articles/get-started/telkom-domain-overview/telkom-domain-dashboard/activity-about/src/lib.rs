use yew::prelude::*;

pub struct ActivityAbout {}

pub enum Msg {}

impl Component for ActivityAbout {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ActivityAbout {}
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

                <h1 class="uk-heading-small uk-margin-medium-bottom">{ "About The Activity Page" }</h1>
                <p>
                    { "The Activity page provides a summary of key data about your Auth0 tenant, including information on active users, failed logins, and more." }
                </p>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Things to know" }</h1>
                    <p>
                        { "Take a look at just a few of Telkom Domain's use cases:" }
                    </p>
                    <ul class="uk-list uk-list-disc">
                        <li>
                            { "Viewing a tenant's Activity page requires an account with an administrator role or the" }
                            <span class="td-label-code td-margin-text">
                                { "read:insights" }
                            </span>
                            { "scope" }
                        </li>
                        <li>
                            { "To ensure data is consistent, the current date cannot be selected in the date picker." }
                        </li>
                        <li>
                            { "You can temporarily opt out of this feature preview by going to" }
                            <a href="#" class="td-margin-text">
                                { "Dashboard > Tenant Settings > Advanced" }
                            </a>
                            { "and disabling the switch in the" }
                            <b class="td-margin-text">{ "Feature Preview" }</b>
                            { "section" }
                        </li>
                    </ul>

                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Metrics" }</h1>
                    <p>{ "Here’s what you will see on your tenant's Activity page." }</p>
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Totals" }</h1>
                    <p>{ "At the top of the page you'll see the total number of users, applications, APIs, and connections for your tenant." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/activity-about/dashboard-activity-totals.png"
                    />
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Active Users" }</h1>
                    <p>{ "The number of daily unique users with successful authentication or authorization activity. You can see the number of unique users for each day by hovering your mouse over a point in the graph." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/activity-about/active_users.png"
                    />
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "User Retention" }</h1>
                    <p>{ "The percentage of users that were active during the given time frame, calculated from the number of active users out of the total number of users on the tenant." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/activity-about/user_retention.png"
                    />
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Signups" }</h1>
                    <p>{ "The number of successful user signups." }</p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/activity-about/sign_ups.png"
                    />
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Failed Logins" }</h1>
                    <p>
                        { "The number of failed user logins (the" }
                        <span class="td-label-code td-margin-text">
                            { "f" }
                        </span>
                        { "log event type) over the given time period." }
                    </p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/activity-about/failed_logins.png"
                    />
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "Compare to last period" }</h1>
                    <p>
                        { "When Compare to last period is enabled:" }
                        <b class="td-margin-text">{ "Compare to last period" }</b>
                        { "is enabled" }
                    </p>
                    <ul class="uk-list uk-list-disc">
                        <li>
                            { "The percentage difference is included in the graph. Positive changes are colored green while negative changes are colored red." }
                        </li>
                        <li>
                            { "Dotted lines are the last time period and solid lines are the current time period." }
                        </li>
                    </ul>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/activity-about/hover.png"
                    />
                </div>

                <div
                    class="uk-margin-medium-bottom"
                >
                    <h1 class="td-text-size-large">{ "View data by time frame" }</h1>
                    <p>
                        { "You can view data for a given time frame by using the datepicker next to the" }
                        <b class="td-margin-text">{ "Compare to last period" }</b>
                        { "checkbox. There are builtin date ranges for the last 7/14/30/60 days, or you can provide the" }
                        <b class="td-margin-text">{ "From" }</b>
                        { "and" }
                        <b class="td-margin-text">{ "To" }</b>
                        { "dates for a custom range." }
                    </p>
                    <img
                        class="uk-margin-top uk-margin-bottom"
                        src="/assets/activity-about/date_picker.png"
                    />
                </div>
                
            </>
        }
    }
}
