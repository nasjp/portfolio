use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys::HtmlInputElement;
extern crate web_sys;

pub mod shell;

struct Model {
    link: ComponentLink<Self>,
    shell: shell::Shell,
    input_ref: NodeRef,
}

enum Msg {
    Update(String),
    Key(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            shell: Default::default(),
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => self.shell.update(val),
            Msg::Key(val) => self.check_key(val),
        }

        if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
            input.focus().unwrap();
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, _: bool) {
        if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
            input.focus().unwrap();
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <ul style="border: none; background: none; outline: none; padding: 0px; font-size: 12px; font-family: 'Microsoft Sans Serif'">
                    { for self.shell.histories().iter().enumerate().map(|e| self.view_entry(e)) }
                    { self.shell.prompt() } <input
                        ref=self.input_ref.clone()
                        style="border: none; background: none; outline: none; padding: 0px; font-size: 12px;"
                        class=""
                        oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                        onkeypress=self.link.callback(|e: KeyboardEvent| Msg::Key(e.key()))
                    />
                </ul>
            </div>
        }
    }
}

impl Model {
    fn view_entry(&self, (_, line): (usize, &shell::Line)) -> Html {
        html! {
            <>
                <li>
                { format!{"{}",  line }}
                </li>
            </>
        }
    }

    fn check_key(&mut self, val: String) {
        if val != "Enter" {
            return;
        }

        self.shell.exec()
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
