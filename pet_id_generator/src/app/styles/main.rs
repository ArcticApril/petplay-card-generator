use stylist::css;
use stylist::Style;

#[allow(non_upper_case_globals)]
pub fn style() -> Style {
    Style::new(css! {
        color: ${catppuccin::PALETTE.macchiato.colors.pink.hex.to_string()};
        text-shadow: 0rem 0rem 5px ${catppuccin::PALETTE.macchiato.colors.mantle.hex.to_string()};
        font-family: sans-serif;
        text-align: left;
        align-items: center;
        display: grid;
        grid-template-columns: 40vw 40vw;
        grid-gap: 5vw;
    })
    .unwrap()
}
