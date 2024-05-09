use std::fmt;
use std::cmp::PartialEq;
use std::str::FromStr;
use anyhow::Error;

#[derive(Clone, PartialEq)]
pub enum LayerName {
    Domain,
    Application,
    Adapters,
    Infrastructure,
}

impl fmt::Display for LayerName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LayerName::Domain => write!(f, "domain"),
            LayerName::Application => write!(f, "application"),
            LayerName::Adapters => write!(f, "adapters"),
            LayerName::Infrastructure => write!(f, "infrastructure"),
        }
    }
}

impl FromStr for LayerName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "domain" => Ok(LayerName::Domain),
            "application" => Ok(LayerName::Application),
            "adapters" => Ok(LayerName::Adapters),
            "infrastructure" => Ok(LayerName::Infrastructure),

            _ => Err(anyhow::anyhow!("Invalid aggregate layer name")),
        }
    }
}
