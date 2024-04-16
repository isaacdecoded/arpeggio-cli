mod core;
mod cli;

use clap::Parser;
use anyhow::Result;
use log::{ info, error };
use std::sync::{ Arc, Mutex };
use dialoguer::{ MultiSelect, Select };

use core::application::use_case_input_port::UseCaseInputPort;
use crate::cli::bounded_context::{
    domain::{
        services::project_service::ProjectService,
        enums::{
            aggregate_layer_name::AggregateLayerName,
            aggregate_layer_component::AggregateLayerComponent,
        },
    },
    application::{
        commands::{
            create_bounded_context_use_case::{
                CreateBoundedContextRequestModel,
                CreateBoundedContextUseCase,
            },
            add_aggregate_use_case::{
                AddAggregateRequestModel,
                AddAggregateLayerRequestModel,
                AddAggregateUseCase,
            },
        },
        queries::find_bounded_contexts_use_case::{
            FindBoundedContextsUseCase,
            FindBoundedContextsRequestModel,
        },
    },
    adapters::presenters::{
        create_bounded_context_presenter::CreateBoundedContextPresenter,
        add_aggregate_presenter::AddAggregatePresenter,
        find_bounded_contexts_presenter::FindBoundedContextsPresenter,
    },
    infrastructure::{
        repositories::{
            filesystem_bounded_context_repository::FilesystemBoundedContextRepository,
            filesystem_find_bounded_contexts_repository::FilesystemFindBoundedContextsRepository,
        },
        services::filesystem_project_service::FilesystemProjectService,
    },
};

#[derive(Parser)]
enum ArpeggioCommand {
    New(NewCommand),
    Add(AddCommand),
    Version,
}

#[derive(Parser)]
enum AddComponentCommand {
    BoundedContext(BoundedContextCommand),
    Aggregate(AggregateCommand),
    // Controller,
    // Presenter,
    // Command,
    // Query,
    // Subscriber,
    // UseCase,
    // Entity,
    // Event,
    // Repository,
    // Service,
    // ValueObject,
}

#[derive(Parser)]
struct BoundedContextCommand {
    bounded_context_name: String,
}

#[derive(Parser)]
struct AggregateCommand {
    aggregate_name: String,
    bounded_context_name: Option<String>,
}

#[derive(Parser)]
struct NewCommand {
    project_name: String,
}

#[derive(Parser)]
struct AddCommand {
    #[clap(subcommand)]
    component: AddComponentCommand,
}

#[derive(Parser)]
struct ArpeggioCli {
    #[clap(subcommand)]
    command: ArpeggioCommand,
}

async fn ask_bounded_context() -> Result<String> {
    let caught_bounded_contexts = Arc::new(Mutex::new(Vec::new()));
    let filesystem_find_bounded_contexts_repository = FilesystemFindBoundedContextsRepository;
    let find_bounded_contexts_presenter = FindBoundedContextsPresenter::new({
        let caught_bounded_contexts = Arc::clone(&caught_bounded_contexts);
        move |bounded_contexts| {
            *caught_bounded_contexts.lock().unwrap() = bounded_contexts;
        }
    });
    let find_bounded_contexts_use_case = FindBoundedContextsUseCase::new(
        &filesystem_find_bounded_contexts_repository,
        &find_bounded_contexts_presenter
    );
    find_bounded_contexts_use_case.interact(FindBoundedContextsRequestModel).await;
    let bounded_contexts = caught_bounded_contexts.lock().unwrap();
    if bounded_contexts.is_empty() {
        error!("No bounded contexts found");
        return Err(anyhow::anyhow!("No bounded contexts found"));
    }
    let selection = Select::new()
        .with_prompt("Please choose a bounded context")
        .default(0)
        .items(&bounded_contexts)
        .interact()?;
    Ok(bounded_contexts[selection].to_string())
}

fn ask_is_custom_layers_mode() -> Result<bool> {
    let selection = Select::new()
        .with_prompt("Do you want to create custom layers?")
        .default(0)
        .item("No")
        .item("Yes")
        .interact()
        .unwrap();
    Ok(selection == 1)
}

fn ask_aggregate_layers() -> Result<Option<Vec<AddAggregateLayerRequestModel>>> {
    let layer_options = [
        AggregateLayerName::Domain,
        AggregateLayerName::Application,
        AggregateLayerName::Adapters,
        AggregateLayerName::Infrastructure,
    ];
    let layer_selections = MultiSelect::new()
        .with_prompt("Choose which layers you want to create (space to select, enter to confirm):")
        .items(&layer_options)
        .interact()?;
    let mut layers: Vec<AddAggregateLayerRequestModel> = vec![];
    for selection in layer_selections {
        let mut component_options: Vec<AggregateLayerComponent> = vec![];
        match selection {
            0 => {
                component_options = vec![
                    AggregateLayerComponent::Entities,
                    AggregateLayerComponent::Events,
                    AggregateLayerComponent::Repositories,
                    AggregateLayerComponent::Services,
                    AggregateLayerComponent::ValueObjects
                ];
            }
            1 => {
                component_options = vec![
                    AggregateLayerComponent::Commands,
                    AggregateLayerComponent::Queries,
                    AggregateLayerComponent::Subscribers
                ];
            }
            2 => {
                component_options = vec![
                    AggregateLayerComponent::Controllers,
                    AggregateLayerComponent::Presenters
                ];
            }
            3 => {
                component_options = vec![
                    AggregateLayerComponent::Repositories,
                    AggregateLayerComponent::Services
                ];
            }
            _ => {}
        }
        let component_selections = MultiSelect::new()
            .with_prompt(
                format!(
                    "Choose the components for the <{}> layer:",
                    layer_options[selection].clone()
                )
            )
            .items(&component_options)
            .interact()?;
        layers.push(AddAggregateLayerRequestModel {
            layer_name: layer_options[selection].clone(),
            components: component_selections
                .into_iter()
                .map(|i| component_options[i].clone())
                .collect(),
        });
    }
    Ok(Some(layers))
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let project_service = FilesystemProjectService;
    let filesystem_bounded_context_repository = FilesystemBoundedContextRepository;
    let args = ArpeggioCli::parse();
    match args.command {
        ArpeggioCommand::Version => {
            println!("Arpeggio CLI v{}", env!("CARGO_PKG_VERSION"));
        }
        ArpeggioCommand::New(new_command) => {
            info!("Creating project...");
            project_service.create_project(&new_command.project_name).await?;
        }
        ArpeggioCommand::Add(add_command) => {
            match add_command.component {
                AddComponentCommand::BoundedContext(command) => {
                    let create_bounded_context_presenter = CreateBoundedContextPresenter;
                    let create_bounded_context_use_case = CreateBoundedContextUseCase::new(
                        &filesystem_bounded_context_repository,
                        &create_bounded_context_presenter
                    );

                    create_bounded_context_use_case.interact(CreateBoundedContextRequestModel {
                        bounded_context_name: command.bounded_context_name,
                    }).await;
                }
                AddComponentCommand::Aggregate(aggregate_command) => {
                    let add_aggregate_presenter = AddAggregatePresenter;
                    let add_aggregate_use_case = AddAggregateUseCase::new(
                        &filesystem_bounded_context_repository,
                        &add_aggregate_presenter
                    );

                    let bounded_context_name = aggregate_command.bounded_context_name.unwrap_or(
                        ask_bounded_context().await?
                    );
                    let is_custom_layer_mode = ask_is_custom_layers_mode()?;
                    let mut aggregate_layers: Option<Vec<AddAggregateLayerRequestModel>> = None;
                    if is_custom_layer_mode {
                        aggregate_layers = ask_aggregate_layers()?;
                    }
                    add_aggregate_use_case.interact(AddAggregateRequestModel {
                        aggregate_name: aggregate_command.aggregate_name,
                        bounded_context_name,
                        aggregate_layers,
                    }).await;
                }
            }
        }
    }
    Ok(())
}
