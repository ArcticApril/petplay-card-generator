use stylist::css;
use stylist::Style;

#[allow(non_upper_case_globals)]
pub fn style() -> Style {
    Style::new(css! {}).unwrap()
}

pub mod entry {
    use stylist::css;
    use stylist::Style;

    #[allow(non_upper_case_globals)]
    pub fn style() -> Style {
        Style::new(css! {
            display: grid;
            grid-template-columns: 20% 40%;
            grid-gap: 5%;
        })
        .unwrap()
    }
}
pub mod prompt {
    use stylist::css;
    use stylist::Style;

    #[allow(non_upper_case_globals)]
    pub fn style() -> Style {
        Style::new(css! {}).unwrap()
    }
}

pub mod field {
    use stylist::css;
    use stylist::Style;

    #[allow(non_upper_case_globals)]
    pub fn style() -> Style {
        Style::new(css! {
            width: 40ch;
            border-radius: ${1}em;
        })
        .unwrap()
    }
}

pub mod dropdown {
    use stylist::css;
    use stylist::Style;

    #[allow(non_upper_case_globals)]
    pub fn style() -> Style {
        let background_color = catppuccin::PALETTE.macchiato.colors.mauve.hex.to_string();
        Style::new(css! {
            background: white;
            border-radius: 1rem;
            width: 41ch;
        })
        .unwrap()
    }
}

pub mod dropdown_item {
    use stylist::css;
    use stylist::Style;

    #[allow(non_upper_case_globals)]
    pub fn style() -> Style {
        let background_color = catppuccin::PALETTE.macchiato.colors.pink.hex.to_string();
        Style::new(css! {
            background: white;
        })
        .unwrap()
    }
}
