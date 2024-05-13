use std::{ path::Path, str::FromStr };
use anyhow::Result;
use std::fs;
use async_trait::async_trait;
use crate::{
    cli::bounded_context::domain::{
        entities::{ aggregate::Aggregate, bounded_context::BoundedContext },
        enums::component_type::ComponentType,
        repositories::bounded_context_repository::{
            BoundedContextRepository,
            BoundedContextRepositoryError,
        },
        value_objects::{
            aggregate_layer::{ AggregateLayer, AggregateLayerValue },
            component_name::ComponentName,
            layer_component::{ LayerComponent, LayerComponentValue },
        },
    },
    core::domain::models::{
        entity::Entity,
        identity_object::IdentityObject,
        value_object::ValueObject,
    },
};

pub struct FilesystemBoundedContextRepository;

impl FilesystemBoundedContextRepository {
    const SOURCE_DIR: &'static str = "./src";

    fn check_directory(&self, directory_path: &str) -> Result<bool> {
        let path = Path::new(&directory_path);
        if path.exists() {
            return Ok(true);
        }
        Ok(false)
    }

    fn create_directory(&self, path: &str) -> Result<()> {
        let exists = self.check_directory(path)?;
        if exists {
            return Ok(());
        }
        fs::create_dir(path)?;
        Ok(())
    }

    fn initialize_directory(&self, path: &str, mut modules: Vec<String>) -> Result<()> {
        let index_file_path = format!("{}/mod.rs", path);
        modules.extend(self.get_dir_names(path)?);
        modules.sort();
        modules.dedup();
        let mut module_exports = modules
            .iter()
            .map(|module| format!("pub mod {};", module))
            .collect::<Vec<_>>()
            .join("\n");
        module_exports.push('\n');
        fs::write(index_file_path, module_exports)?;
        Ok(())
    }

    fn get_dir_names(&self, path: &str) -> Result<Vec<String>> {
        let directory_names = fs
            ::read_dir(path)
            .map_err(|e| BoundedContextRepositoryError::ReadError(e.to_string()))?
            .filter_map(Result::ok)
            .filter(|e|
                e
                    .file_type()
                    .map(|t| t.is_dir())
                    .unwrap_or(false)
            )
            .filter_map(|e| e.file_name().into_string().ok())
            .filter(|name| name != "core")
            .collect::<Vec<_>>();
        Ok(directory_names)
    }

    fn get_dir_file_names(&self, path: &str) -> Result<Vec<String>> {
        let mut file_names: Vec<String> = fs
            ::read_dir(path)?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path()
                        .file_stem()
                        .and_then(|stem| stem.to_str().map(|s| s.to_string()))
                })
            })
            .filter(|file_name| file_name != "mod")
            .collect();
        file_names.sort();
        Ok(file_names)
    }
}

#[async_trait]
impl BoundedContextRepository for FilesystemBoundedContextRepository {
    async fn write_bounded_context(
        &self,
        bounded_context: &BoundedContext
    ) -> Result<(), BoundedContextRepositoryError> {
        let bounded_context_path = format!(
            "{}/{}",
            Self::SOURCE_DIR,
            &bounded_context.get_id().to_string()
        );
        self
            .create_directory(&bounded_context_path)
            .map_err(|e| BoundedContextRepositoryError::WriteError(e.to_string()))?;
        bounded_context.aggregates.iter().for_each(|aggregate| {
            let aggregate_name = &aggregate.get_id();
            let aggregate_path = format!("{}/{}", bounded_context_path, aggregate_name.get_value());
            self.create_directory(&aggregate_path).unwrap();

            let aggregate_layers = aggregate.get_layers();
            aggregate_layers.iter().for_each(|layer| {
                let layer_path = format!("{}/{}", aggregate_path, layer.get_value().name);
                self.create_directory(&layer_path).unwrap();

                let components = &layer.get_value().components;
                components.iter().for_each(|component| {
                    let component_path = format!(
                        "{}/{}",
                        layer_path,
                        component.get_value().component_type
                    );
                    let component_file_path = format!(
                        "{}/{}.rs",
                        component_path,
                        component.get_value().component_name.get_value()
                    );
                    self.create_directory(&component_path).unwrap();
                    if !Path::new(&component_file_path).exists() {
                        fs::write(component_file_path, "").unwrap();
                    }
                    let component_definitions = self.get_dir_file_names(&component_path).unwrap();
                    self.initialize_directory(&component_path, component_definitions).unwrap();
                });
                let component_directories = components
                    .iter()
                    .map(|c| c.get_value().component_type.to_string())
                    .collect::<Vec<_>>();
                self.initialize_directory(&layer_path, component_directories).unwrap();
            });

            self.initialize_directory(
                &aggregate_path,
                aggregate_layers
                    .iter()
                    .map(|layer| layer.get_value().name.to_string())
                    .collect::<Vec<_>>()
            ).unwrap();
        });
        self.initialize_directory(
            &bounded_context_path,
            bounded_context.aggregates
                .iter()
                .map(|aggregate| aggregate.get_id().to_string())
                .collect::<Vec<_>>()
        ).unwrap();
        Ok(())
    }

    async fn read_bounded_context(
        &self,
        bounded_context_id: &IdentityObject
    ) -> Result<Option<BoundedContext>, BoundedContextRepositoryError> {
        let bounded_context_path = format!(
            "{}/{}",
            Self::SOURCE_DIR,
            bounded_context_id.get_value()
        );
        let exists = self
            .check_directory(&bounded_context_path)
            .map_err(|e| BoundedContextRepositoryError::ReadError(e.to_string()))?;
        if !exists {
            return Err(
                BoundedContextRepositoryError::NotFound(bounded_context_id.get_value().to_string())
            );
        }
        let aggregate_directories = fs
            ::read_dir(bounded_context_path.clone())
            .map_err(|e| BoundedContextRepositoryError::ReadError(e.to_string()))?
            .filter_map(Result::ok)
            .filter(|e|
                e
                    .file_type()
                    .map(|t| t.is_dir())
                    .unwrap_or(false)
            )
            .filter_map(|e| e.file_name().into_string().ok())
            .filter(|name| name != "core")
            .collect::<Vec<_>>();
        let aggregates = aggregate_directories
            .iter()
            .map(|aggregate_dir| {
                let aggregate_path = format!("{}/{}", bounded_context_path, aggregate_dir);
                let layer_directories = fs
                    ::read_dir(aggregate_path.clone())
                    .unwrap()
                    .filter_map(Result::ok)
                    .filter(|e|
                        e
                            .file_type()
                            .map(|t| t.is_dir())
                            .unwrap_or(false)
                    )
                    .filter_map(|e| e.file_name().into_string().ok())
                    .collect::<Vec<_>>();
                let layers = layer_directories
                    .iter()
                    .map(|layer| {
                        let layer_path = format!("{}/{}", aggregate_path, layer);
                        let layer_component_directories = fs
                            ::read_dir(layer_path.clone())
                            .unwrap()
                            .filter_map(Result::ok)
                            .filter(|e|
                                e
                                    .file_type()
                                    .map(|t| t.is_dir())
                                    .unwrap_or(false)
                            )
                            .filter_map(|e| e.file_name().into_string().ok())
                            .collect::<Vec<_>>();
                        let mut layer_components = vec![];
                        layer_component_directories
                            .iter()
                            .filter(|component_directory| {
                                ComponentType::from_str(component_directory).is_ok()
                            })
                            .for_each(|component_directory| {
                                let component_path = format!(
                                    "{}/{}",
                                    layer_path,
                                    component_directory.clone()
                                );
                                let component_files = fs
                                    ::read_dir(component_path)
                                    .unwrap()
                                    .filter_map(Result::ok)
                                    .filter(|e|
                                        e
                                            .file_type()
                                            .map(|t| t.is_file())
                                            .unwrap_or(false)
                                    )
                                    .filter_map(|e| e.file_name().into_string().ok())
                                    .collect::<Vec<_>>();
                                component_files.iter().for_each(|file| {
                                    layer_components.push(
                                        LayerComponent::new(LayerComponentValue {
                                            component_type: component_directory.parse().unwrap(),
                                            component_name: ComponentName::new(
                                                file.replace(".rs", "")
                                            ),
                                        })
                                    )
                                });
                            });
                        AggregateLayer::new(AggregateLayerValue {
                            name: layer.parse().unwrap(),
                            components: layer_components,
                        })
                    })
                    .collect::<Vec<_>>();
                Aggregate::new(IdentityObject::new(aggregate_dir.to_string()), layers)
            })
            .collect::<Vec<_>>();
        Ok(Some(BoundedContext::new(bounded_context_id.clone(), aggregates)))
    }
}
