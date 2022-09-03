use crate::temperature::TemperatureUnit;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, KeyboardEvent};

mod temperature;

#[derive(Debug)]
struct App {
    celcius: f64,
    fahrenheit: f64,
    kelvin: f64,
}

pub fn draw_number_textbox<C: Component<Message = Msg>, Msg: 'static>(
    ctx: &Context<C>,
    label: &str,
    value: f64,
    placeholder: &'static str,
    default: f64,
    mk_event: fn(f64) -> Msg,
) -> Html {
    html! {
        <div>
            <div>{label}</div>
            <input
                type="number"
                placeholder={placeholder}
                value={value.to_string()}
                onkeyup={ctx.link().batch_callback(move |e: KeyboardEvent| {
                    let target = e.target();
                    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                    input.map(move |input| mk_event(input.value().parse().unwrap_or(default)))
                })}
            />
        </div>
    }
}
enum Msg {
    Set(TemperatureUnit, f64),
}

impl App {
    fn consistent_with_respect_to(&mut self, unit: TemperatureUnit) {
        use TemperatureUnit::*;
        match unit {
            C => {
                self.fahrenheit = TemperatureUnit::convert(C, F, self.celcius);
                self.kelvin = TemperatureUnit::convert(C, K, self.celcius);
            }
            F => {
                self.celcius = TemperatureUnit::convert(F, C, self.fahrenheit);
                self.kelvin = TemperatureUnit::convert(F, K, self.fahrenheit);
            }
            K => {
                self.celcius = TemperatureUnit::convert(K, C, self.kelvin);
                self.fahrenheit = TemperatureUnit::convert(K, F, self.kelvin);
            }
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut ret = Self {
            celcius: 0.0,
            fahrenheit: 0.0,
            kelvin: 0.0,
        };
        ret.consistent_with_respect_to(TemperatureUnit::C);
        ret
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use TemperatureUnit::*;
        let Msg::Set(unit, value) = msg;
        match unit {
            C => {
                self.celcius = value;
            }
            F => {
                self.fahrenheit = value;
            }
            K => {
                self.kelvin = value;
            }
        }
        self.consistent_with_respect_to(unit);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        use TemperatureUnit::*;
        html! {
            <>
                {draw_number_textbox(ctx, "Celcius", self.celcius, "Enter temperature here", 0.0, |value| Msg::Set(C, value))}
                <br />
                {draw_number_textbox(ctx, "Fahrenheit", self.fahrenheit, "Enter temperature here", 0.0, |value| Msg::Set(F, value))}
                <br />
                {draw_number_textbox(ctx, "Kelvin", self.kelvin, "Enter temperature here", 0.0, |value| Msg::Set(K, value))}
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
