use yew::prelude::*;

pub struct Footer {}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav
                class="uk-navbar-container"
                uk-navbar="true"
                style="border-top: .5px solid rgba(0,0,0,.2);"
            >

                <div class="uk-navbar-left">

                    <ul class="uk-navbar-nav">
                        <li class="uk-active"><a href="#">{ "Active" }</a></li>
                        <li>
                            <a href="#">{ "Parent" }</a>
                            <div class="uk-navbar-dropdown">
                                <ul class="uk-nav uk-navbar-dropdown-nav">
                                    <li class="uk-active"><a href="#">{ "Active" }</a></li>
                                    <li><a href="#">{ "Item" }</a></li>
                                    <li><a href="#">{ "Item" }</a></li>
                                </ul>
                            </div>
                        </li>
                        <li><a href="#">{ "Item" }</a></li>
                    </ul>

                </div>

                <div class="uk-navbar-right">

                    <ul class="uk-navbar-nav">
                        <li class="uk-active"><a href="#">{ "Active" }</a></li>
                        <li>
                            <a href="#">{ "Parent" }</a>
                            <div class="uk-navbar-dropdown">
                                <ul class="uk-nav uk-navbar-dropdown-nav">
                                    <li class="uk-active"><a href="#">{ "Active" }</a></li>
                                    <li><a href="#">{ "Item" }</a></li>
                                    <li><a href="#">{ "Item" }</a></li>
                                </ul>
                            </div>
                        </li>
                        <li><a href="#">{ "Item" }</a></li>
                    </ul>

                </div>

            </nav>
        }
    }
}
