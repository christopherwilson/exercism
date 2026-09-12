#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}


impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            earth_years: s as f64 / (60.0 * 60.0 * 24.0 * 365.25) 
                        // converting seconds to Earth years
        }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::ORBITAL_PERIOD
    }
}

macro_rules! planet {
    ( $planet:ident, $orbital_period: expr ) => {
        pub struct $planet;

        impl Planet for $planet {
            const ORBITAL_PERIOD: f64 = $orbital_period;
        }
    }
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
