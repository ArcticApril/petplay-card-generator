use stylist::css;
use stylist::Style;

#[allow(non_upper_case_globals)]
pub fn style() -> Style {
    let top_left_color = catppuccin::PALETTE.latte.colors.surface0.hex.to_string();
    let bottom_right_color = catppuccin::PALETTE.macchiato.colors.mauve.hex.to_string();
    Style::new(css! {
        align-items: center;
        justify-content: center;
        display: flex;
        background: linear-gradient(to bottom right, ${top_left_color}, ${bottom_right_color});
        column-count: 2;
    })
    .unwrap()
}
