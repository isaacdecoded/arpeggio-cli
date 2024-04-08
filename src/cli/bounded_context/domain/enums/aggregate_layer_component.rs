use std::fmt;
use std::cmp::PartialEq;
use std::str::FromStr;
use anyhow::Error;

#[derive(Clone)]
pub enum AggregateLayerComponent {
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
    Custom(String),
}

impl PartialEq for AggregateLayerComponent {
    fn eq(&self, _other: &Self) -> bool {
        matches!(self, _other)
    }
}

impl fmt::Display for AggregateLayerComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AggregateLayerComponent::Controllers => write!(f, "controllers"),
            AggregateLayerComponent::Presenters => write!(f, "presenters"),
            AggregateLayerComponent::Commands => write!(f, "commands"),
            AggregateLayerComponent::Queries => write!(f, "queries"),
            AggregateLayerComponent::Subscribers => write!(f, "subscribers"),
            AggregateLayerComponent::Entities => write!(f, "entities"),
            AggregateLayerComponent::Events => write!(f, "events"),
            AggregateLayerComponent::Repositories => write!(f, "repositories"),
            AggregateLayerComponent::Services => write!(f, "services"),
            AggregateLayerComponent::ValueObjects => write!(f, "value_objects"),
            AggregateLayerComponent::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for AggregateLayerComponent {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "controllers" => Ok(AggregateLayerComponent::Controllers),
            "presenters" => Ok(AggregateLayerComponent::Presenters),
            "commands" => Ok(AggregateLayerComponent::Commands),
            "queries" => Ok(AggregateLayerComponent::Queries),
            "subscribers" => Ok(AggregateLayerComponent::Subscribers),
            "entities" => Ok(AggregateLayerComponent::Entities),
            "events" => Ok(AggregateLayerComponent::Events),
            "repositories" => Ok(AggregateLayerComponent::Repositories),
            "services" => Ok(AggregateLayerComponent::Services),
            "value_objects" => Ok(AggregateLayerComponent::ValueObjects),
            _ => Ok(AggregateLayerComponent::Custom(s.to_string())),
        }
    }
}
