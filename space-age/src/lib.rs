// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    // Structure holds the number of
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            earth_years: s as f64 / (3600.0 * 24.0 * 365.25), // seconds in an Earth year
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years
    }
}

macro_rules!  create_years_during {
    ($planet:ident , $factor: expr) => {
        impl Planet for $planet {
            fn years_during(d: &Duration) ->f64 {
                d.earth_years / $factor
            }
        }
    }

}

create_years_during! (Mercury , 0.2408467 );
create_years_during! (Earth , 1.0 );
create_years_during! (Venus , 0.61519726 );
create_years_during! (Mars , 1.8808158 );
create_years_during! (Jupiter , 11.862615 );
create_years_during! (Saturn , 29.447498 );
create_years_during! (Uranus , 84.016846 );
create_years_during! (Neptune ,164.79132 );

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

