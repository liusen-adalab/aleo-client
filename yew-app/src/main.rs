use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window};
use yew::prelude::*;
mod text_input;

use text_input::TextInput;

enum Msg {
    CreateKey,
    // (address, pool_ip:port)
    StartMine,
    SetAddr(String),
    SetPool(String),
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = start_mine, catch)]
    pub async fn start_mine(addr: String, pool: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = creat_key, catch)]
    pub async fn create_key() -> Result<JsValue, JsValue>;
}

struct Miner {
    address: String,
    pool: String,
    state: State,
}
enum State {
    Mining,
    Ready,
}

impl Component for Miner {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            address: "".into(),
            state: State::Ready,
            pool: "".into(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreateKey => (),
            Msg::StartMine => {
                let addr = self.address.clone();
                let pool = self.pool.clone();
                spawn_local(async move {
                    if let Ok(_) = start_mine(addr, pool).await {
                        true;
                    } else {
                        false;
                    }
                });
            }
            Msg::SetAddr(addr) => {
                self.address = addr;
                // console::log(self.address.as_bytes());
            }
            Msg::SetPool(pool) => {
                self.pool = pool;
                // console::log(self.pool);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let create_key = ctx.link().callback(|_| Msg::CreateKey);
        let start_mine = ctx.link().callback( |_| Msg::StartMine);
        let set_addr = ctx.link().callback(Msg::SetAddr);
        let set_pool = ctx.link().callback(Msg::SetPool);

        html! {
            <section class = "container">
                <div class = "input-container">
                    <TextInput on_change={set_addr} value={self.address.clone()} placeholder="aleo address"/>
                </div>
                <div class = "input-container">
                    <TextInput on_change={set_pool} value={self.pool.clone()} placeholder="pool ip, eg: 127.0.0.1:8080"/>
                </div>
                <div class = "button-container">
                    <button onclick={create_key}>{ "create key" }</button>
                    <button onclick={start_mine}>{ "start mine" }</button>
                </div>
            </section>
        }
    }
}

fn main() {
    yew::start_app::<Miner>();
}
