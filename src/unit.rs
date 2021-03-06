//! The Unit enum defines css units

use num_rational::Rational;
use num_traits::One;
use std::fmt;

/// Units in css.
///
/// As defined in https://www.w3.org/TR/css3-values/
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unit {
    // Distance units, <length> type
    Em,
    Ex,
    Ch,
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
    Px,
    // Other quantities
    // <angle> type
    Deg,
    Grad,
    Rad,
    Turn,
    // <time> type
    S,
    Ms,
    // <frequency> type
    Hz,
    Khz,
    // <resolution>
    Dpi,
    Dpcm,
    Dppx,
    // Special units
    Percent,
    None,
}

impl Unit {
    pub fn dimension(&self) -> &'static str {
        match *self {
            Unit::Em | Unit::Ex | Unit::Ch | Unit::Rem | Unit::Vw |
            Unit::Vh | Unit::Vmin | Unit::Vmax | Unit::Cm | Unit::Mm |
            Unit::Q | Unit::In | Unit::Pt | Unit::Pc | Unit::Px => "length",

            Unit::Deg | Unit::Grad | Unit::Rad | Unit::Turn => "angle",

            Unit::S | Unit::Ms => "time",

            Unit::Hz | Unit::Khz => "frequency",

            Unit::Dpi | Unit::Dpcm | Unit::Dppx => "resolution",

            Unit::Percent | Unit::None => "none",
        }
    }

    /// Some of these are exact and correct, others are more arbitrary.
    /// When comparing 10cm to 4in, these factors will give correct results.
    /// When comparing rems to vw, who can say?
    pub fn scale_factor(&self) -> Rational {
        match *self {
            Unit::Em | Unit::Rem => Rational::new(10, 2),
            Unit::Ex => Rational::new(10, 3),
            Unit::Ch => Rational::new(10, 4),
            Unit::Vw | Unit::Vh | Unit::Vmin | Unit::Vmax => Rational::one(),
            Unit::Cm => Rational::new(10, 1),
            Unit::Mm => Rational::one(),
            Unit::Q => Rational::new(1, 4),
            Unit::In => Rational::new(254, 10),
            Unit::Pt => Rational::new(254, 720),
            Unit::Pc => Rational::new(254, 60),
            Unit::Px => Rational::new(254, 960),

            Unit::Deg => Rational::new(360, 1),
            Unit::Grad => Rational::new(400, 1),
            Unit::Rad => Rational::new(62832, 10000), // approximate
            Unit::Turn => Rational::one(),

            Unit::S => Rational::one(),
            Unit::Ms => Rational::new(1, 1000),

            Unit::Hz => Rational::one(),
            Unit::Khz => Rational::new(1000, 1),

            Unit::Dpi => Rational::new(96, 1),
            Unit::Dpcm => Rational::new(9600, 254),
            Unit::Dppx => Rational::one(),

            Unit::Percent => Rational::new(1, 100),
            Unit::None => Rational::one(),
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Distance units, <length> type
            Unit::Em => write!(out, "em"),
            Unit::Ex => write!(out, "ex"),
            Unit::Ch => write!(out, "ch"),
            Unit::Rem => write!(out, "rem"),
            Unit::Vw => write!(out, "vw"),
            Unit::Vh => write!(out, "vh"),
            Unit::Vmin => write!(out, "vmin"),
            Unit::Vmax => write!(out, "vmax"),
            Unit::Cm => write!(out, "cm"),
            Unit::Mm => write!(out, "mm"),
            Unit::Q => write!(out, "q"),
            Unit::In => write!(out, "in"),
            Unit::Pt => write!(out, "pt"),
            Unit::Pc => write!(out, "pc"),
            Unit::Px => write!(out, "px"),
            // <angle> type
            Unit::Deg => write!(out, "deg"),
            Unit::Grad => write!(out, "grad"),
            Unit::Rad => write!(out, "rad"),
            Unit::Turn => write!(out, "turn"),
            // <time> type
            Unit::S => write!(out, "s"),
            Unit::Ms => write!(out, "ms"),
            // <frequency> type
            Unit::Hz => write!(out, "Hz"),
            Unit::Khz => write!(out, "kHz"),
            // <resolution>
            Unit::Dpi => write!(out, "dpi"),
            Unit::Dpcm => write!(out, "dpcm"),
            Unit::Dppx => write!(out, "dppx"),
            // Special units
            Unit::Percent => write!(out, "%"),
            Unit::None => Ok(()),
        }
    }
}

named!(pub unit<&[u8], Unit>,
       alt_complete!(
           // Distance units, <length> type
           value!(Unit::Em, tag!("em")) |
           value!(Unit::Ex, tag!("ex")) |
           value!(Unit::Ch, tag!("ch")) |
           value!(Unit::Rem, tag!("rem")) |
           value!(Unit::Vw, tag!("vw")) |
           value!(Unit::Vh, tag!("vh")) |
           value!(Unit::Vmin, tag!("vmin")) |
           value!(Unit::Vmax, tag!("vmax")) |
           value!(Unit::Cm, tag!("cm")) |
           value!(Unit::Mm, tag!("mm")) |
           value!(Unit::Q, tag!("q")) |
           value!(Unit::In, tag!("in")) |
           value!(Unit::Pt, tag!("pt")) |
           value!(Unit::Pc, tag!("pc")) |
           value!(Unit::Px, tag!("px")) |
           // <angle> type
           value!(Unit::Deg, tag!("deg")) |
           value!(Unit::Grad, tag!("grad")) |
           value!(Unit::Rad, tag!("rad")) |
           value!(Unit::Turn, tag!("turn")) |
           // <time> type
           value!(Unit::S, tag!("s")) |
           value!(Unit::Ms, tag!("ms")) |
           // <frequency> type
           value!(Unit::Hz, tag!("Hz")) |
           value!(Unit::Khz, tag!("kHz")) |
           // <resolution>
           value!(Unit::Dpi, tag!("dpi")) |
           value!(Unit::Dpcm, tag!("dpcm")) |
           value!(Unit::Dppx, tag!("dppx")) |
           // Special units
           value!(Unit::Percent, tag!("%")) |
           value!(Unit::None, tag!(""))));
