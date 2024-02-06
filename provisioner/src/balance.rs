use std::fmt::Display;

enum CoinType {
    Planck,
    DOT,
}

pub(crate) struct Balance {
    raw: u128,
    typ: CoinType,
}

impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let postfix = match self.typ {
            CoinType::Planck => "Planck",
            CoinType::DOT => "DOT",
        };
        write!(f, "{} {}", self.raw, postfix)
    }
}

impl Balance {
    pub(crate) fn from_planck(val: u128) -> Self {
        Self {
            raw: val,
            typ: CoinType::Planck,
        }
    }

    pub(crate) fn from_dot(val: u128) -> Self {
        Self {
            raw: val,
            typ: CoinType::DOT,
        }
    }

    pub(crate) fn as_planck(&self) -> Self {
        let raw = match self.typ {
            CoinType::Planck => self.raw,
            CoinType::DOT => self.raw * 10u128.pow(10),
        };
        Self {
            raw,
            typ: CoinType::Planck,
        }
    }

    pub(crate) fn as_dot(&self) -> Self {
        let raw = match self.typ {
            CoinType::Planck => self.raw / 10u128.pow(10),
            CoinType::DOT => self.raw,
        };
        Self {
            raw,
            typ: CoinType::DOT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance_conversion() {
        let planck = Balance::from_planck(1000000000000);
        let dot = planck.as_dot();
        assert_eq!(100, dot.raw);

        let dot = Balance::from_dot(100);
        let planck = dot.as_planck();
        assert_eq!(1000000000000, planck.raw);
    }
}
