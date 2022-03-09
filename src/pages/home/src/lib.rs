use yew::prelude::*;

pub struct Home {}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="uk-background-muted">
        <div class="uk-container uk-container-large" style="padding-bottom: 800px;">
            <form
                class="uk-margin-large-top uk-margin-large-bottom"
            >
                
                <div class="uk-margin">
                    <div
                        class="uk-inline uk-width-1-1 uk-border-circle"
                    >
                        <span class="uk-form-icon" uk-icon="icon: search"></span>
                        <input
                            class="uk-input uk-form-large"
                            type="text"
                            placeholder="Search the docs"
                        />
                    </div>
                </div>
            </form>


            <div
                class="uk-margin-xlarge-bottom"
            >
                <div class="uk-margin-large-bottom">
                    <h1 class="uk-heading-small uk-text-bold">{ "Start building" }</h1>
                    <p class="uk-text-large">
                        { "Choose your application type to get started" }
                    </p>
                </div>

                <div
                    class="
                        uk-grid-column-small
                        uk-grid-row-large
                        uk-child-width-1-1@s
                        uk-child-width-1-2@m
                        uk-child-width-1-3@l
                        uk-grid-small
                        uk-child-width-expand@s
                        uk-text-center
                    "
                    uk-grid="true"
                    // uk-height-match="target: > div > .uk-card"
                >
                    <div>
                        <div
                            class="uk-card uk-card-default uk-card-hover uk-card-body"
                            style="border-radius: 5px;"
                        >
                            <div
                                class="uk-margin-auto uk-position-relative"
                                style="background: hsl(165, 100%, 39%); border-radius: 15px; width: 100px; height: 100px;"
                            >
                                <span
                                    class="uk-form-icon uk-text-large uk-position-center"
                                    style="font-size: 56px;"
                                >
                                    <i
                                        class="fa-solid fa-mobile-screen-button"
                                        style="color: hsl(165, 100%, 89%);"
                                    ></i>
                                </span>
                            </div>
                            <h3 class="uk-card-title">{ "Native/Mobile App" }</h3>
                            <p>{ "Mobile or Desktop app that runs natively on a device" }</p>
                            <p
                                class="uk-text-muted uk-text-small"
                            >
                                { "e.g., iOS, Android" }
                            </p>
                        </div>
                    </div>

                    <div>
                        <div
                            class="uk-card uk-card-default uk-card-hover uk-card-body"
                            style="border-radius: 5px;"
                        >
                            <div
                                class="uk-margin-auto uk-position-relative"
                                style="background: hsl(0, 84%, 64%); border-radius: 15px; width: 100px; height: 100px;"
                            >
                                <span
                                    class="uk-form-icon uk-text-large uk-position-center"
                                    style="font-size: 56px;"
                                >
                                    <i
                                        class="fa-solid fa-desktop"
                                        style="color: hsl(0, 84%, 94%);"
                                    ></i>
                                </span>
                            </div>
                            <h3 class="uk-card-title">{ "Single-Page App" }</h3>
                            <p>{ "Javascript web app that runs in the browser" }</p>
                            <p
                                class="uk-text-muted uk-text-small"
                            >
                                { "e.g., AngularJS + Node.js, React" }
                            </p>
                        </div>
                    </div>

                    <div>
                        <div
                            class="uk-card uk-card-default uk-card-hover uk-card-body"
                            style="border-radius: 5px;"
                        >
                            <div
                                class="uk-margin-auto uk-position-relative"
                                style="background: hsl(188, 16%, 53%); border-radius: 15px; width: 100px; height: 100px;"
                            >
                                <span
                                    class="uk-form-icon uk-text-large uk-position-center"
                                    style="font-size: 56px;"
                                >
                                    <i
                                        class="fa-solid fa-pager"
                                        style="color: hsl(188, 16%, 93%);"
                                    ></i>
                                </span>
                            </div>
                            <h3 class="uk-card-title">{ "Regular Web App" }</h3>
                            <p>{ "Traditional web app that runs on the server" }</p>
                            <p
                                class="uk-text-muted uk-text-small"
                            >
                                { "e.g., Express.js, ASP.NET" }
                            </p>
                        </div>
                    </div>

                    <div>
                        <div
                            class="uk-card uk-card-default uk-card-hover uk-card-body"
                            style="border-radius: 5px;"
                        >
                            <div
                                class="uk-margin-auto uk-position-relative"
                                style="background: hsl(47, 100%, 50%); border-radius: 15px; width: 100px; height: 100px;"
                            >
                                <span
                                    class="uk-form-icon uk-text-large uk-position-center"
                                    style="font-size: 56px;"
                                >
                                    <i
                                        class="fa-solid fa-laptop-code"
                                        style="color: hsl(47, 100%, 90%);"
                                    ></i>
                                </span>
                            </div>
                            <h3 class="uk-card-title">{ "Backend/API" }</h3>
                            <p>{ "An API or service protected by Telkom Domain" }</p>
                            <p
                                class="uk-text-muted uk-text-small"
                            >
                                { "e.g., Express.js API, ASP.NET API" }
                            </p>
                        </div>
                    </div>

                </div>
            </div>

            <div
                class="uk-margin-large-bottom"
            >
                <div class="uk-margin-large-bottom">
                    <h1 class="uk-heading-small uk-text-bold">{ "Learn about Telkom Domain" }</h1>
                </div>

                <div
                    class="
                        uk-grid
                        uk-grid-row-large
                        uk-child-width-1-1@s
                        uk-child-width-1-2@m
                        uk-child-width-1-3@l
                        uk-grid-small
                        uk-text-center
                    "
                    uk-grid="true"
                >
                    <div>
                        <div
                            class="uk-flex uk-padding"
                            style="border-radius: 5px; border: .5px solid rgba(100,100,100,.2);"
                        >
                            <div
                                style="padding-right: .75rem;"
                            >
                                <div
                                    class="uk-margin-auto uk-position-relative uk-width-auto"
                                    style="background: hsl(210, 88%, 93%); border-radius: 15px; width: 64px; height: 64px;"
                                >
                                    <span
                                        class="uk-form-icon uk-text-large uk-position-center uk-text-primary"
                                        style="font-size: 39px;"
                                    >
                                        <i
                                            class="fa-solid fa-bolt"
                                        ></i>
                                    </span>
                                </div>
                            </div>
                            <div>
                                <h3
                                    class="uk-card-title uk-margin-small-bottom"
                                    style="font-size: 27px;"
                                >{ "Get started" }</h3>
                                <div
                                    class="uk-align-left"
                                    style="font-size: 18px;"
                                >
                                    <p
                                        class="uk-text-muted uk-margin-remove-top"
                                    >
                                        { "Learn the basics" }
                                    </p>
                                    <p>
                                        <a>
                                            { "Telkom Domain Overview" }
                                        </a>
                                    </p>
                                    <p>
                                        <a>
                                            { "Tenant Settings" }
                                        </a>
                                    </p>
                                    <p>
                                        <a>
                                            { "Authentication and Authorization Flows" }
                                        </a>
                                    </p>
                                    <p class="uk-align-right uk-margin-small-top">
                                        <a>
                                            { "More" }
                                            <span uk-icon="arrow-right"></span>
                                        </a>
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div>
                        <div
                            class="uk-card uk-card-default uk-card-hover uk-card-body"
                            style="border-radius: 5px;"
                        >
                            <div
                                class="uk-margin-auto uk-position-relative"
                                style="background: hsl(0, 84%, 64%); border-radius: 15px; width: 100px; height: 100px;"
                            >
                                <span
                                    class="uk-form-icon uk-text-large uk-position-center"
                                    style="font-size: 56px;"
                                >
                                    <i
                                        class="fa-solid fa-desktop"
                                        style="color: hsl(0, 84%, 94%);"
                                    ></i>
                                </span>
                            </div>
                            <h3 class="uk-card-title">{ "Single-Page App" }</h3>
                            <p>{ "Javascript web app that runs in the browser" }</p>
                            <p
                                class="uk-text-muted uk-text-small"
                            >
                                { "e.g., AngularJS + Node.js, React" }
                            </p>
                        </div>
                    </div>

                    <div>
                        <div
                            class="uk-card uk-card-default uk-card-hover uk-card-body"
                            style="border-radius: 5px;"
                        >
                            <div
                                class="uk-margin-auto uk-position-relative"
                                style="background: hsl(188, 16%, 53%); border-radius: 15px; width: 100px; height: 100px;"
                            >
                                <span
                                    class="uk-form-icon uk-text-large uk-position-center"
                                    style="font-size: 56px;"
                                >
                                    <i
                                        class="fa-solid fa-pager"
                                        style="color: hsl(188, 16%, 93%);"
                                    ></i>
                                </span>
                            </div>
                            <h3 class="uk-card-title">{ "Regular Web App" }</h3>
                            <p>{ "Traditional web app that runs on the server" }</p>
                            <p
                                class="uk-text-muted uk-text-small"
                            >
                                { "e.g., Express.js, ASP.NET" }
                            </p>
                        </div>
                    </div>

                    <div>
                        <div
                            class="uk-card uk-card-default uk-card-hover uk-card-body"
                            style="border-radius: 5px;"
                        >
                            <div
                                class="uk-margin-auto uk-position-relative"
                                style="background: hsl(47, 100%, 50%); border-radius: 15px; width: 100px; height: 100px;"
                            >
                                <span
                                    class="uk-form-icon uk-text-large uk-position-center"
                                    style="font-size: 56px;"
                                >
                                    <i
                                        class="fa-solid fa-laptop-code"
                                        style="color: hsl(47, 100%, 90%);"
                                    ></i>
                                </span>
                            </div>
                            <h3 class="uk-card-title">{ "Backend/API" }</h3>
                            <p>{ "An API or service protected by Telkom Domain" }</p>
                            <p
                                class="uk-text-muted uk-text-small"
                            >
                                { "e.g., Express.js API, ASP.NET API" }
                            </p>
                        </div>
                    </div>

                </div>
            </div>


        </div>
    </div>
        }
    }
}
