#[allow(dead_code)]
use stylist::css;
use stylist::Style;

#[allow(non_upper_case_globals)]
pub fn h1() -> Style {
    Style::new(css! {display: block;
                     font-size: 2.5rem;
                     margin-top: ${1}em;
    })
    .unwrap()
}

pub fn h2() -> Style {
    Style::new(css! {}).unwrap()
}

pub fn h3() -> Style {
    Style::new(css! {}).unwrap()
}

pub fn h4() -> Style {
    Style::new(css! {}).unwrap()
}

pub fn h5() -> Style {
    Style::new(css! {}).unwrap()
}

pub fn h6() -> Style {
    Style::new(css! {}).unwrap()
}
