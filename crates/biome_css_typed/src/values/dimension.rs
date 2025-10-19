#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthUnit {
    Px,
    Em,
    Rem,
    Vw,
    Vh,
    Vmin,
    Vmax,
    Cm,
    Mm,
    Q,
    In,
    Pt,
    Pc,
    Ch,
    Ex,
    Svw,
    Svh,
    Lvw,
    Lvh,
    Dvw,
    Dvh,
}

impl LengthUnit {
    pub fn from_str(u: &str) -> Option<Self> {
        use LengthUnit::*;
        let unit = if u.eq_ignore_ascii_case("px") {
            Px
        } else if u.eq_ignore_ascii_case("em") {
            Em
        } else if u.eq_ignore_ascii_case("rem") {
            Rem
        } else if u.eq_ignore_ascii_case("vw") {
            Vw
        } else if u.eq_ignore_ascii_case("vh") {
            Vh
        } else if u.eq_ignore_ascii_case("vmin") {
            Vmin
        } else if u.eq_ignore_ascii_case("vmax") {
            Vmax
        } else if u.eq_ignore_ascii_case("cm") {
            Cm
        } else if u.eq_ignore_ascii_case("mm") {
            Mm
        } else if u.eq_ignore_ascii_case("q") {
            Q
        } else if u.eq_ignore_ascii_case("in") {
            In
        } else if u.eq_ignore_ascii_case("pt") {
            Pt
        } else if u.eq_ignore_ascii_case("pc") {
            Pc
        } else if u.eq_ignore_ascii_case("ch") {
            Ch
        } else if u.eq_ignore_ascii_case("ex") {
            Ex
        } else if u.eq_ignore_ascii_case("svw") {
            Svw
        } else if u.eq_ignore_ascii_case("svh") {
            Svh
        } else if u.eq_ignore_ascii_case("lvw") {
            Lvw
        } else if u.eq_ignore_ascii_case("lvh") {
            Lvh
        } else if u.eq_ignore_ascii_case("dvw") {
            Dvw
        } else if u.eq_ignore_ascii_case("dvh") {
            Dvh
        } else {
            return None;
        };
        Some(unit)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    pub value: f64,
    pub unit: LengthUnit,
}
