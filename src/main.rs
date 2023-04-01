use clock_app::*;
use yew::prelude::*;
use gloo::console::log;
use web_sys::HtmlTextAreaElement;
use wasm_bindgen::JsCast;

enum Msg {
    InpScram(String),
    Solve
}

struct Clock{
    scram:String,
    sol: Vec<i8>
}

impl Component for Clock{
    type Message =  Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self{
        Self {scram: "UR1+ DR2+ DL5+ UL0+ U2- R1+ D3- L1+ ALL6+ y2 U2+ R3+ D5+ L2+ ALL4+ UL".to_string(), 
        sol: vec![]
        }
    }

    fn update(&mut self,_ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InpScram(scram) => {
                log!("got scr");
                self.scram = scram.clone();
                self.sol = Vec::new();
            }
            Msg::Solve => {
                log!("solver started");
                self.sol = run(self.scram.clone());
                log!("done started");
            }
            
        }
        true
    }

    fn view(&self,_ctx: &Context<Self>) -> Html {
        let link = _ctx.link();
        html! {
            <div class="container">
                <p>{ self.scram.clone() }</p>
                <textarea
                    oninput={link.callback(|event:InputEvent| Msg::InpScram(event.target().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap().value()))}>
                </textarea>
                <button onclick={
                    link.callback(|_| Msg::Solve)}>{ "Solve" }</button>
                <p>{ format!("Solution: {:?}",self.sol)}</p>
            </div>

        }
    }
}

fn main() {
    yew::start_app::<Clock>();
}
