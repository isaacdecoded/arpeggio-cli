mod core;
mod cli;
use clap::Parser;
use anyhow::Result;
use std::sync::{ Arc, Mutex };
use dialoguer::{ MultiSelect, Select };
use core::application::use_case_input_port::UseCaseInputPort;
use crate::cli::bounded_context::{
    adapters::presenters::{
        add_aggregate_presenter::AddAggregatePresenter,
        add_component_presenter::AddComponentPresenter,
        create_bounded_context_presenter::CreateBoundedContextPresenter,
        find_bounded_contexts_presenter::FindBoundedContextsPresenter,
    },
    application::{
        commands::{
            add_aggregate_use_case::{
                AddAggregateLayerRequestModel,
                AddAggregateRequestModel,
                AddAggregateUseCase,
            },
            add_component_use_case::{
                AddComponentRequestModel,
                AddComponentUseCase,
                ComponentRequestModel,
            },
            create_bounded_context_use_case::{
                CreateBoundedContextRequestModel,
                CreateBoundedContextUseCase,
            },
        },
        queries::find_bounded_contexts_use_case::{
            BoundedContextReadModel,
            FindBoundedContextsRequestModel,
            FindBoundedContextsUseCase,
        },
    },
    domain::{
        enums::{ component_type::ComponentType, layer_name::LayerName },
        services::project_service::ProjectService,
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
    Controller(ComponentCommand),
    Presenter(ComponentCommand),
    Command(ComponentCommand),
    Query(ComponentCommand),
    Subscriber(ComponentCommand),
    Entity(ComponentCommand),
    Event(ComponentCommand),
    Repository(ComponentCommand),
    Service(ComponentCommand),
    ValueObject(ComponentCommand),
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
struct ComponentCommand {
    component_name: String,
    aggregate_name: Option<String>,
    bounded_context_name: Option<String>,
}

#[derive(Parser)]
struct ArpeggioCli {
    #[clap(subcommand)]
    command: ArpeggioCommand,
}

fn ask_option_selection(options: Vec<String>) -> Result<String> {
    let selection = Select::new()
        .with_prompt("Please choose an option")
        .default(0)
        .items(&options)
        .interact()?;
    Ok(options[selection].to_string())
}

async fn get_bounded_contexts() -> Result<Vec<BoundedContextReadModel>> {
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
    let bounded_contexts = caught_bounded_contexts.lock().unwrap().to_vec();
    Ok(bounded_contexts)
}

fn ask_is_default_layers_mode() -> Result<bool> {
    let selection = Select::new()
        .with_prompt(
            "Do you want to initialize default layers? (domain, application, adapters and infrastructure)"
        )
        .default(0)
        .item("Yes")
        .item("No")
        .interact()
        .unwrap();
    Ok(selection == 0)
}

fn ask_aggregate_layers() -> Result<Option<Vec<AddAggregateLayerRequestModel>>> {
    let layer_options = [
        LayerName::Domain,
        LayerName::Application,
        LayerName::Adapters,
        LayerName::Infrastructure,
    ];
    let layer_selections = MultiSelect::new()
        .with_prompt("Choose which layers you want to create (space to select, enter to confirm):")
        .items(&layer_options)
        .interact()?;
    let mut layers: Vec<AddAggregateLayerRequestModel> = vec![];
    for selection in layer_selections {
        layers.push(AddAggregateLayerRequestModel {
            layer_name: layer_options[selection].clone(),
            components: vec![],
        });
    }
    Ok(Some(layers))
}

async fn add_component(
    component_command: ComponentCommand,
    component_type: ComponentType,
    bounded_contexts: Vec<BoundedContextReadModel>,
    bounded_context_repository: &FilesystemBoundedContextRepository
) -> Result<()> {
    if bounded_contexts.is_empty() {
        return Err(anyhow::anyhow!("No bounded contexts found"));
    }
    let add_component_use_case = AddComponentUseCase::new(
        bounded_context_repository,
        &AddComponentPresenter
    );
    let bounded_context_name = component_command.bounded_context_name.unwrap_or(
        ask_option_selection(
            bounded_contexts
                .iter()
                .map(|bc| bc.name.clone())
                .collect()
        )?
    );
    let aggregate_name = component_command.aggregate_name.unwrap_or(
        ask_option_selection(
            bounded_contexts
                .iter()
                .find(|bc| bc.name == bounded_context_name)
                .unwrap()
                .aggregates.to_vec()
        )?
    );
    add_component_use_case.interact(AddComponentRequestModel {
        bounded_context_name,
        aggregate_name,
        component: ComponentRequestModel {
            component_type,
            component_name: component_command.component_name,
        },
    }).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let project_service = FilesystemProjectService;
    let filesystem_bounded_context_repository = FilesystemBoundedContextRepository;
    let args = ArpeggioCli::parse();
    let bounded_contexts = get_bounded_contexts().await?;
    match args.command {
        ArpeggioCommand::Version => {
            println!("Arpeggio CLI v{}", env!("CARGO_PKG_VERSION"));
        }
        ArpeggioCommand::New(new_command) => {
            println!("Creating project...");
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
                    if bounded_contexts.is_empty() {
                        return Err(anyhow::anyhow!("No bounded contexts found"));
                    }
                    let add_aggregate_presenter = AddAggregatePresenter;
                    let add_aggregate_use_case = AddAggregateUseCase::new(
                        &filesystem_bounded_context_repository,
                        &add_aggregate_presenter
                    );
                    let bounded_context_name = aggregate_command.bounded_context_name.unwrap_or(
                        ask_option_selection(
                            bounded_contexts
                                .iter()
                                .map(|bc| bc.name.clone())
                                .collect()
                        )?
                    );
                    let is_default_layers_mode = ask_is_default_layers_mode()?;
                    let mut aggregate_layers: Option<Vec<AddAggregateLayerRequestModel>> = None;
                    if !is_default_layers_mode {
                        aggregate_layers = ask_aggregate_layers()?;
                    }
                    add_aggregate_use_case.interact(AddAggregateRequestModel {
                        aggregate_name: aggregate_command.aggregate_name,
                        bounded_context_name,
                        aggregate_layers,
                    }).await;
                }
                AddComponentCommand::Controller(component_command) => {
                    add_component(
                        component_command,
                        ComponentType::Controllers,
                        bounded_contexts,
                        &filesystem_bounded_context_repository
                    ).await?;
                }
                AddComponentCommand::Presenter(component_command) => {
                    add_component(
                        component_command,
                        ComponentType::Presenters,
                        bounded_contexts,
                        &filesystem_bounded_context_repository
                    ).await?;
                }
                AddComponentCommand::Command(component_command) => {
                    add_component(
                        component_command,
                        ComponentType::Commands,
                        bounded_contexts,
                        &filesystem_bounded_context_repository
                    ).await?;
                }
                AddComponentCommand::Query(component_command) => {
                    add_component(
                        component_command,
                        ComponentType::Queries,
                        bounded_contexts,
                        &filesystem_bounded_context_repository
                    ).await?;
                }
                AddComponentCommand::Subscriber(component_command) => {
                    add_component(
                        component_command,
                        ComponentType::Subscribers,
                        bounded_contexts,
                        &filesystem_bounded_context_repository
                    ).await?;
                }
                AddComponentCommand::Entity(component_command) => {
                    add_component(
                        component_command,
                        ComponentType::Entities,
                        bounded_contexts,
                        &filesystem_bounded_context_repository
                    ).await?;
                }
                AddComponentCommand::Event(component_command) => {
                    add_component(
                        component_command,
                        ComponentType::Events,
                        bounded_contexts,
                        &filesystem_bounded_context_repository
                    ).await?;
                }
                AddComponentCommand::Repository(component_command) => {
                    add_component(
                        component_command,
                        ComponentType::Repositories,
                        bounded_contexts,
                        &filesystem_bounded_context_repository
                    ).await?;
                }
                AddComponentCommand::Service(component_command) => {
                    add_component(
                        component_command,
                        ComponentType::Services,
                        bounded_contexts,
                        &filesystem_bounded_context_repository
                    ).await?;
                }
                AddComponentCommand::ValueObject(component_command) => {
                    add_component(
                        component_command,
                        ComponentType::ValueObjects,
                        bounded_contexts,
                        &filesystem_bounded_context_repository
                    ).await?;
                }
            }
        }
    }
    Ok(())
}
