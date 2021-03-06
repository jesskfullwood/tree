use log::info;
use rand::Rng;
use std::collections::BTreeMap;
use fig::html::*;
use fig::*;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct Model {
    divs: BTreeMap<u32, String>,
    timer: Timer<Model>,
}

#[derive(Clone, Debug)]
enum Msg {
    Roll(u32, String),
}

impl fig::Model for Model {
    type Msg = Msg;
}

fn update(msg: Msg, mut model: Model) -> (Model, Cmd<Msg>) {
    match msg {
        Msg::Roll(id, text) => {
            info!("Roll!");
            if model.divs.len() > 100 {
                model.divs = BTreeMap::new();
            }
            model.divs.insert(id, text);
            (model, Cmd::none())
        }
    }
}

fn adiv(id: u32, s: &str) -> Html<Model> {
    div!(format!("{}: {}", id, s))
}

fn view(model: &Model) -> Html<Model> {
    div!(model
        .divs
        .iter()
        .map(|(k, v)| adiv(*k, v))
        .collect::<Vec<_>>())
}

#[wasm_bindgen]
pub fn render() {
    fig::application(
        |key, _| {
            (
                Model {
                    divs: BTreeMap::new(),
                    timer: Timer::new(key, 10, |_| {
                        let mut rng = rand::thread_rng();
                        let n: u32 = rng.gen_range(0, 1000);
                        let text = if n % 2 == 0 { "Hello" } else { "Goodbye" };
                        Cmd::msg(Msg::Roll(n, text.into()))
                    })
                    .expect("No timer"),
                },
                Cmd::none(),
            )
        },
        view,
        update,
        fig::util::on_url_request_force_load,
        |_| unimplemented!(),
        "app",
    )
    .expect("Failed to run");
}
