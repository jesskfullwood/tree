use futures::Future;
use serde::{Deserialize, Serialize};
use tree::fetch;
use tree::html::*;
use tree::select;
use tree::*;
// TODO remove this dependency
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
struct Model {
    select: String,
    check: bool,
    input: String,
    click_ct: u32,
    list_ct: u32,
    server_says: Option<String>,
    route: Route,
}

impl Default for Model {
    fn default() -> Model {
        Model {
            select: "this".into(),
            check: false,
            input: "Initial".into(),
            click_ct: 0,
            list_ct: 5,
            server_says: None,
            route: Route::Home,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Route {
    Home,
    Items,
    Summary,
}

fn on_url_change(url: Url) -> Cmd<Msg> {
    log!("Url change");
    let route = match url.path() {
        "/" => Route::Home,
        "/items" => Route::Items,
        "/summary" => Route::Summary,
        _other => return Cmd::load_url("/"),
    };
    Cmd::msg(Msg::Route(route))
}

#[derive(Clone, Debug)]
enum Msg {
    Select(String),
    ToggleCheck,
    Input(String),
    ButtonClick,
    AddLi,
    RmLi,
    FetchSelected(String),
    FetchedSelected(String),
    Route(Route),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Data {
    data: String,
}

impl tree::Model for Model {
    type Msg = Msg;
}

fn update(msg: Msg, model: Model) -> (Model, Cmd<Msg>) {
    log!("update model with message: {:?}", msg);
    match msg {
        Msg::Select(select) => (Model { select, ..model }, Cmd::none()),
        Msg::FetchSelected(val) => (model, Cmd::spawn(fetch_selected(val))),
        Msg::FetchedSelected(val) => (
            Model {
                server_says: Some(val),
                ..model
            },
            Cmd::none(),
        ),
        Msg::ToggleCheck => (
            Model {
                check: !model.check,
                ..model
            },
            Cmd::none(),
        ),
        Msg::Input(input) => (
            Model {
                input: input.to_ascii_lowercase(),
                ..model
            },
            Cmd::none(),
        ),
        Msg::ButtonClick => (
            Model {
                click_ct: model.click_ct + 1,
                ..model
            },
            Cmd::none(),
        ),
        Msg::AddLi => (
            Model {
                list_ct: model.list_ct + 1,
                ..model
            },
            Cmd::none(),
        ),
        Msg::RmLi => (
            Model {
                list_ct: if model.list_ct > 0 {
                    model.list_ct - 1
                } else {
                    0
                },
                ..model
            },
            Cmd::none(),
        ),
        Msg::Route(route) => (Model { route, ..model }, Cmd::none()),
    }
}

fn fetch_selected(val: String) -> impl Future<Item = Msg, Error = JsValue> {
    log!("Fetch: '{}'", val);
    fetch::Request::new("http://localhost:8001")
        .method(fetch::Method::Post)
        .header("Content-Type", "application/json")
        .body_json(&Data { data: val })
        .fetch_json()
        .map(|data: Data| Msg::FetchedSelected(data.data))
}

fn view(model: &Model) -> Html<Model> {
    log!("rendering model: {:?}", model);
    div!(
        id("my-app"),
        h1!("Tree demo"),
        p!("Enter some ", b!("text"), " if you please!"),
        div!(
            input!(
                value(model.input.clone()),
                on_input((), |(), input| Msg::Input(input)),
                placeholder("placeholder")
            ),
            p!(i!("Boldly repeat: "), b!(model.input.clone()))
        ),
        div!(
            p!(class!("bluesy"), "Classy!"),
            button!(
                on_click((), |()| Msg::ButtonClick),
                format!("Clicked: {}", model.click_ct),
            ),
            select!(
                on_input((), |(), sel| Msg::Select(sel)),
                option!(value("this"), "this"),
                option!(value("that"), "that"),
                option!(value("other"), "other"),
            ),
            button!(
                on_click(model.select.clone(), |select| Msg::FetchSelected(
                    select.clone()
                )),
                "Send request"
            ),
            p!("Our server says:", {
                let says = if let Some(ref says) = model.server_says {
                    says
                } else {
                    "Nothing!"
                };
                b!(says.to_string())
            })
        ),
        div!(
            id("links"),
            p!(a!(href("/"), "Home")),
            match model.route {
                Route::Home => div!(
                    p!(a!(href("/items"), "View items")),
                    p!(a!(href("/summary"), "View summary"))
                ),
                Route::Items => div!(
                    button!(on_click((), |()| Msg::AddLi), "+ item"),
                    button!(on_click((), |()| Msg::RmLi), "- item"),
                    ul!((0..model.list_ct)
                        .map(|i| li!(format!("List item {}", i)))
                        .collect::<Vec<_>>()),
                ),
                Route::Summary => div!(p!(format!("You have created {} items", model.list_ct)),),
            }
        )
    )
}

#[wasm_bindgen]
pub fn render() {
    tree::application(
        |_key, url| (Model::default(), on_url_change(url)),
        view,
        update,
        tree::util::on_url_request_intercept,
        on_url_change,
        "app",
    )
    .expect("Failed to run");
}
