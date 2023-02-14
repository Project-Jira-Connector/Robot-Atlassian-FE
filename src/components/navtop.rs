use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::route::AppRoute;



pub enum Msg {
    AddOne,

}


pub struct Navtop {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Navtop {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {

        type Anchor = RouterAnchor<AppRoute>;

        html! {
            <div class="base" style="background: teal; text-align: center;">
                <div class="navbar">
                    <h5>{"Robot Automation"}</h5>
                        <div class="navbar-option">
                            <div style=" text-decoration: none; color: black;">
                                <Anchor route=AppRoute::Home>
                                    <label class="link" style=" text-decoration: none!important; ">
                                        {"Homepage"}
                                    </label>
                                </Anchor>
                            </div>

                            <div style=" text-decoration: none; color: rgb(100,100,100);">
                                <Anchor route=AppRoute::InputPage>
                                    <label class="link" style=" text-decoration: none!important;" >
                                        {"Create new"}
                                    </label>
                                </Anchor>
                            </div>
                        </div>
                </div>
                // <div style="text-decoration: none; color: rgb(100,100,100);">
                //     <Anchor route=AppRoute::Schedules>
                //         <p class="link" style=" text-decoration: none!important; color: rgb(100,100,100);">
                //             {"Schedules"}
                //         </p>
                //     </Anchor>
                // </div>
                // <div style=" text-decoration: none; color: rgb(100,100,100);">
                //     <Anchor route=AppRoute::Login>
                //         <p class="link" style=" text-decoration: none!important; color: rgb(100,100,100);">
                //             {"Login"}
                //         </p>
                //     </Anchor>
                // </div>

            </div>
        }
    }
}
