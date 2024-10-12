use crate::app::form::style;
use gloo::console::log;
use strum::{Display, EnumProperty, IntoEnumIterator};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

struct FormResult {
    name: AttrValue,
    owned_state: AttrValue,
}

#[derive(Debug, strum::EnumIter, strum::EnumProperty, Display)]
enum OwnedValues {
    #[strum(props(Id = "owned", Text = "Owned"))]
    Owned,
    #[strum(props(Id = "fostered", Text = "Fostered"))]
    Fostered,
    #[strum(props(Id = "unowned", Text = "Unowned"))]
    Unowned,
}

#[function_component(AppForm)]
pub fn app_form() -> Html {
    let owned_values = OwnedValues::iter()
        .map(|i| {
            (
                AttrValue::from(i.get_str("Id").unwrap()),
                AttrValue::from(i.get_str("Text").unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let owned_state_handler = use_state(|| None::<OwnedValues>);
    let owned_state = owned_state_handler.clone();
    let owned_onchange = {
        let owned_state_handler = owned_state_handler.clone();
        Callback::from(move |event: Event| {
            log!(format!("{:?}", owned_state));
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            owned_state_handler.set(match value.as_str() {
                "owned" => Some(OwnedValues::Owned),
                "fostered" => Some(OwnedValues::Fostered),
                "unowned" => Some(OwnedValues::Unowned),
                _ => None,
            });
            log!(value);
            log!(format!("{:?}", owned_state));
        })
    };

    let name_state_handler = use_state(String::default);
    let name_state = name_state_handler.clone();
    let name_onchange = {
        let name_state_handler = name_state_handler.clone();
        let owned_state = owned_state_handler.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            name_state_handler.set(value.clone());
            log!(value);
            log!(format!("{:?}", name_state));
            log!(format!("{:?}", owned_state));
        })
    };

    html!(
        <form class={style::style()}>
            <TextField onchange={name_onchange} name="name" prompt="Enter name of pet: " required=true />
            <Dropdown onchange={owned_onchange} name="owned" id="owned" prompt="Is the pet owned?" options={owned_values} />
        </form>
    )
}

#[derive(PartialEq, Properties)]
struct TextFieldProps {
    name: AttrValue,
    prompt: AttrValue,
    required: bool,
    onchange: Callback<Event>,
}

#[function_component(TextField)]
fn text_field(
    TextFieldProps {
        name,
        prompt,
        required,
        onchange,
    }: &TextFieldProps,
) -> Html {
    html!(
        <div class={style::entry::style()}>
            <lable class={style::prompt::style()} for={name}>{ prompt }</lable>
            <input {onchange} class={style::field::style()} type={name} name={name} id="name" required={*required} />
        </div>
    )
}

#[derive(PartialEq, Properties)]
struct DropdownProps {
    options: Vec<(AttrValue, AttrValue)>,
    name: AttrValue,
    id: AttrValue,
    prompt: AttrValue,
    onchange: Callback<Event>,
}

#[function_component(Dropdown)]
fn dropdown(
    DropdownProps {
        options,
        name,
        id,
        prompt,
        onchange,
    }: &DropdownProps,
) -> Html {
    html!(
        <div class={style::entry::style()}>
            <lable class={style::prompt::style()} name={name} id={id}>{ prompt }</lable>
            <select {onchange} class={style::dropdown::style()} name={name} id={id}>
                <option class={style::dropdown_item::style()} value="">{ "— Please choose an option —" }</option>
                { options.iter().map(|item| {
                    html!{
                        <DropdownItem name={item.0.clone()} display={item.1.clone()} />
                    }
                }).collect::<Html>() }
            </select>
        </div>
    )
}

#[derive(PartialEq, Eq, Properties)]
struct DropdownItemProps {
    name: AttrValue,
    display: AttrValue,
}

#[function_component(DropdownItem)]
fn dropdown_item(DropdownItemProps { name, display }: &DropdownItemProps) -> Html {
    html!(
        <option class={style::dropdown_item::style()} value={name}>{ display }</option>
    )
}
