#[derive(PartialEq, Debug)]
pub struct Celcius(pub f64);
#[derive(PartialEq, Debug)]
pub struct Fahrenheit(pub f64);
#[derive(PartialEq, Debug)]
pub struct Kelvin(pub f64);

impl From<Fahrenheit> for Celcius {
    fn from(t: Fahrenheit) -> Self {
        Self((t.0 - 32.0) * 5.0 / 9.0)
    }
}

impl From<Kelvin> for Celcius {
    fn from(t: Kelvin) -> Self {
        Self(t.0 - 273.15)
    }
}

impl From<Celcius> for Fahrenheit {
    fn from(t: Celcius) -> Self {
        Self((9.0 * t.0 / 5.0) + 32.0)
    }
}

impl From<Kelvin> for Fahrenheit {
    fn from(t: Kelvin) -> Self {
        Celcius::from(t).into()
    }
}

impl From<Celcius> for Kelvin {
    fn from(t: Celcius) -> Self {
        Self(t.0 + 273.15)
    }
}

impl From<Fahrenheit> for Kelvin {
    fn from(t: Fahrenheit) -> Self {
        Celcius::from(t).into()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TemperatureUnit {
    C,
    F,
    K,
}

impl std::fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TemperatureUnit::*;
        match self {
            C => f.write_str("Celcius"),
            F => f.write_str("Fahrenheit"),
            K => f.write_str("Kelvin"),
        }
    }
}

impl std::str::FromStr for TemperatureUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TemperatureUnit::*;
        match s {
            "Celcius" => Ok(C),
            "Fahrenheit" => Ok(F),
            "Kelvin" => Ok(K),
            _ => Err(()),
        }
    }
}

impl TemperatureUnit {
    pub fn convert(frm: Self, to: Self, temp: f64) -> f64 {
        use TemperatureUnit::*;
        match (frm, to) {
            (C, C) => temp,
            (F, F) => temp,
            (K, K) => temp,
            (C, F) => Fahrenheit::from(Celcius(temp)).0,
            (C, K) => Kelvin::from(Celcius(temp)).0,
            (F, C) => Celcius::from(Fahrenheit(temp)).0,
            (F, K) => Kelvin::from(Fahrenheit(temp)).0,
            (K, C) => Celcius::from(Kelvin(temp)).0,
            (K, F) => Fahrenheit::from(Kelvin(temp)).0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_celcius() {
        assert_eq!(Celcius(100.0).0, 100.0)
    }

    #[test]
    fn test_can_create_fahrenheit() {
        assert_eq!(Fahrenheit(32.0).0, 32.0)
    }

    #[test]
    fn test_can_create_kelvin() {
        assert_eq!(Celcius(103.5).0, 103.5)
    }

    #[test]
    fn test_can_convert_from_celcius_to_fahrenheit() {
        use TemperatureUnit::*;
        assert_eq!(TemperatureUnit::convert(C, F, 100.0), 212.0)
    }

    #[test]
    fn test_can_convert_from_celcius_to_kelvin() {
        use TemperatureUnit::*;
        assert_eq!(TemperatureUnit::convert(C, K, 100.0), 373.15)
    }

    #[test]
    fn test_can_convert_from_fahrenheit_to_celcius() {
        use TemperatureUnit::*;
        assert_eq!(TemperatureUnit::convert(F, C, 32.0), 0.0)
    }

    #[test]
    fn test_can_convert_from_fahrenheit_to_kelvin() {
        use TemperatureUnit::*;
        assert_eq!(TemperatureUnit::convert(F, K, 32.0), 273.15)
    }

    #[test]
    fn test_can_convert_from_kelvin_to_celcius() {
        use TemperatureUnit::*;
        assert_eq!(TemperatureUnit::convert(K, C, 373.15), 100.0)
    }

    #[test]
    fn test_can_convert_from_kelvin_to_fahrenheit() {
        use TemperatureUnit::*;
        assert_eq!(TemperatureUnit::convert(K, F, 373.15), 212.0)
    }
}
