use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys::HtmlInputElement;

struct Model {
    link: ComponentLink<Self>,
    state: State,
    input_ref: NodeRef,
}

pub struct State {
    pub value: String,
    pub old: Vec<String>,
}

enum Msg {
    Update(String),
    Key(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            value: "".into(),
            old: vec![],
        };

        let input_ref = NodeRef::default();

        Self {
            link,
            state,
            input_ref,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.state.value = val;
            }
            Msg::Key(val) => self.check_key(val),
        }

        if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
            input.focus().unwrap();
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
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
                <ul class="item-list" style="list-style: none; font-size: 12px; font-family: 'Microsoft Sans Serif'">
                    { for self.state.old.iter().enumerate().map(|e| self.view_entry(e))}
                    {">"}
                    <input
                        ref=self.input_ref.clone()
                        style="border: none; background: none; outline: none; padding: 0px; font-size: 12px; font-family"
                        class=""
                        value=&self.state.value
                        oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                        onkeypress=self.link.callback(|e: KeyboardEvent| Msg::Key(e.key()))
                    />
                </ul>
            </div>
        }
    }
}

impl Model {
    fn view_entry(&self, (_, entry): (usize, &String)) -> Html {
        html! {
            <>
                <li style="list-style: none; font-size: 12px; font-family: 'Microsoft Sans Serif'">
                { format!{"{}{}",">",  entry} }
                </li>
            </>
        }
    }

    fn check_key(&mut self, val: String) {
        if val != "Enter" {
            return;
        }

        self.state.old.push(self.state.value.to_string());
        self.state.value = "".into();
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
