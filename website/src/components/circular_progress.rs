use yew::prelude::*;
use yew_material_components::{MatCircularProgress, MatCircularProgressFourColor, MatButton};
use crate::components::Codeblock;

pub struct CircularProgress {
    link: ComponentLink<Self>,
    closed: bool,
    progress: f32,
}

pub enum Msg {
    Close,
    ChangeProgress,
}

impl Component for CircularProgress {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { closed: false, link, progress: 0.0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeProgress => {
                self.progress += 0.1;
            }
            Msg::Close => {
                yew::services::ConsoleService::log("test");
                self.closed = !self.closed;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        use crate::with_raw_code;
        let toggle_example = with_raw_code!(toggle_example { html! {
            <section class="demo">
                <span onclick=self.link.callback(|_| Msg::Close)>
                    <MatButton label="Toggle" raised=true  />
                 </span><br />
                <MatCircularProgress closed=self.closed progress=0.75 />
            </section>
        }});
        html! {<>
            <Codeblock code=toggle_example.0 title="Toggle open state">
                {toggle_example.1}
            </Codeblock>

            <section class="comp-demo">
                <h2>{"Indeterminate"}</h2>
                <MatCircularProgress indeterminate=true />
            </section>

            <section class="comp-demo">
                <h2>{"Determinate"}</h2> <br />
                <span onclick=self.link.callback(|_| Msg::ChangeProgress)>
                    <MatButton label="Increase progress" raised=true />
                </span> <br />
                <MatCircularProgress progress=self.progress />
            </section>

            <p>
                <b>{"Note:"}</b> {" Four colored variants of circular progress is available under "} <code>{"MatCircularProgressFourColor"}</code>
            </p>

            <section class="comp-demo">
                <h2>{"Four colored indeterminate"}</h2>
                <MatCircularProgressFourColor indeterminate=true />
            </section>
        </>}
    }
}