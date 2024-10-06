use yew::prelude::*;

mod styles;
use styles::{body, header, main};
mod card;
use card::Card;
mod form;
use form::AppForm;

#[function_component(App)]
pub fn app() -> Html {
    html! {
            <body class={body::style()}>
        <main class={main::style()}>
            <Card />
            <div>
                <h1 class={header::h1()}>{ "Welcome to the Pet Id card generator!" }</h1>
                <h2 class={header::h2()}>{ "Tell us a little bit about your new (or old!) friend!" }</h2>
                <AppForm />
            </div>
        </main>
            </body>
    }
}
