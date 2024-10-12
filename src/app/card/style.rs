use stylist::css;
use stylist::Style;

#[allow(non_upper_case_globals)]
pub fn style() -> Style {
    Style::new(css! {
        width: 40vw;
        aspect-ratio: 27/17;
        background: ${catppuccin::PALETTE.latte.colors.base.hex.to_string()};
        border-radius: 1rem;
    })
    .unwrap()
}
