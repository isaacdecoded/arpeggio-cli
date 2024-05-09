use std::fmt;
use std::cmp::PartialEq;
use std::str::FromStr;
use anyhow::Error;

#[derive(Clone)]
pub enum ComponentType {
    Controllers,
    Presenters,
    Commands,
    Queries,
    Subscribers,
    Entities,
    Events,
    Repositories,
    Services,
    ValueObjects,
    // Custom(String),
}

impl PartialEq for ComponentType {
    fn eq(&self, _other: &Self) -> bool {
        matches!(self, _other)
    }
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComponentType::Controllers => write!(f, "controllers"),
            ComponentType::Presenters => write!(f, "presenters"),
            ComponentType::Commands => write!(f, "commands"),
            ComponentType::Queries => write!(f, "queries"),
            ComponentType::Subscribers => write!(f, "subscribers"),
            ComponentType::Entities => write!(f, "entities"),
            ComponentType::Events => write!(f, "events"),
            ComponentType::Repositories => write!(f, "repositories"),
            ComponentType::Services => write!(f, "services"),
            ComponentType::ValueObjects => write!(f, "value_objects"),
            // ComponentType::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for ComponentType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "controllers" => Ok(ComponentType::Controllers),
            "presenters" => Ok(ComponentType::Presenters),
            "commands" => Ok(ComponentType::Commands),
            "queries" => Ok(ComponentType::Queries),
            "subscribers" => Ok(ComponentType::Subscribers),
            "entities" => Ok(ComponentType::Entities),
            "events" => Ok(ComponentType::Events),
            "repositories" => Ok(ComponentType::Repositories),
            "services" => Ok(ComponentType::Services),
            "value_objects" => Ok(ComponentType::ValueObjects),
            _ => Err(Error::msg("Invalid component type")),
            // _ => Ok(ComponentType::Custom(s.to_string())),
        }
    }
}
