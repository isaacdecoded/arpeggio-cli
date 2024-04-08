use std::fmt;
use std::cmp::PartialEq;
use std::str::FromStr;
use anyhow::Error;

#[derive(Clone)]
pub enum AggregateLayerName {
    Domain,
    Application,
    Adapters,
    Infrastructure,
}

impl PartialEq for AggregateLayerName {
    fn eq(&self, _other: &Self) -> bool {
        matches!(self, _other)
    }
}

impl fmt::Display for AggregateLayerName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AggregateLayerName::Domain => write!(f, "domain"),
            AggregateLayerName::Application => write!(f, "application"),
            AggregateLayerName::Adapters => write!(f, "adapters"),
            AggregateLayerName::Infrastructure => write!(f, "infrastructure"),
        }
    }
}

impl FromStr for AggregateLayerName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "domain" => Ok(AggregateLayerName::Domain),
            "application" => Ok(AggregateLayerName::Application),
            "adapters" => Ok(AggregateLayerName::Adapters),
            "infrastructure" => Ok(AggregateLayerName::Infrastructure),

            _ => Err(anyhow::anyhow!("Invalid aggregate layer name")),
        }
    }
}
