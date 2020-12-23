//! Demonstrates loading prefabs using the Amethyst engine.

use std::collections::HashMap;

use amethyst::{
    assets::{
        prefab::{register_component_type, ComponentRegistry, Prefab},
        AssetStorage, DefaultLoader, Handle, Loader, LoaderBundle,
    },
    core::transform::TransformBundle,
    ecs::query,
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
<<<<<<< HEAD
=======
        rendy::{
            hal::command::ClearColor,
            mesh::{Normal, Position, TexCoord},
        },
>>>>>>> origin/legion_v2
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    Error,
};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;
use type_uuid::TypeUuid;
#[derive(TypeUuid, Serialize, Deserialize, SerdeDiff, Clone, Default)]
#[uuid = "f5780013-bae4-49f0-ac0e-a108ff52fec0"]
struct Position2D {
    position: Vec<f32>,
}

register_component_type!(Position2D);
// type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

struct AssetsExample {
    prefab_handle: Option<Handle<Prefab>>,
}

impl SimpleState for AssetsExample {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let StateData { resources, .. } = data;
        // let prefab_handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        //     loader.load("prefab/example.prefab", RonFormat, ())
        // });
        // data.world.create_entity().with(prefab_handle).build();
        let loader = resources.get_mut::<DefaultLoader>().unwrap();
        let prefab_handle: Handle<Prefab> = loader.load("prefab/test.prefab");
        self.prefab_handle = Some(prefab_handle);
    }
    fn update(&mut self, data: &mut StateData<'_, GameData>) -> SimpleTrans {
        let StateData {
            world, resources, ..
        } = data;

        if self.prefab_handle.is_none() {
            log::info!("No prefab");
            return Trans::None;
        }

        let component_registry = resources.get_mut::<ComponentRegistry>().unwrap();
        let prefab_storage = resources.get_mut::<AssetStorage<Prefab>>().unwrap();
        if let Some(opened_prefab) = prefab_storage.get(self.prefab_handle.as_ref().unwrap()) {
            let mut clone_impl_result = HashMap::default();
            let mut spawn_impl =
                component_registry.spawn_clone_impl(&resources, &mut clone_impl_result);
            let mappings = world.clone_from(
                &opened_prefab.prefab.world,
                &query::any(),
                &mut spawn_impl,
                // &mut component_registry, // .spawn_clone_impl(resources, &opened_prefab.prefab_to_world_mappings),
            );
            log::info!("{:?}", mappings);
        };
        Trans::None
    }
}

/// Wrapper around the main, so we can return errors easily.
fn main() -> Result<(), Error> {
    {
        let mut config = amethyst::LoggerConfig::default();
        // config.log_file = Some(std::path::PathBuf::from("asset_loading.log"));
        config.level_filter = amethyst::LogLevelFilter::Info;
        config.module_levels.push((
            "amethyst_assets".to_string(),
            amethyst::LogLevelFilter::Debug,
        ));
        config.module_levels.push((
            "atelier_daemon".to_string(),
            amethyst::LogLevelFilter::Debug,
        ));
        config.module_levels.push((
            "atelier_loader".to_string(),
            amethyst::LogLevelFilter::Trace,
        ));
        amethyst::start_logger(config);
    }
    // amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    // Add our meshes directory to the asset loader.
    let assets_dir = app_root.join("examples/prefab/assets");

    let display_config_path = app_root.join("examples/prefab/config/display.ron");

<<<<<<< HEAD
    let mut dispatcher_builder = DispatcherBuilder::default();
    dispatcher_builder
        // with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .add_bundle(LoaderBundle)
        .add_bundle(TransformBundle)
=======
    let mut game_data = DispatcherBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .add_bundle(TransformBundle::new())?
>>>>>>> origin/legion_v2
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?.with_clear(ClearColor {
                        float32: [0.34, 0.36, 0.52, 1.0],
                    }),
                )
                .with_plugin(RenderShaded3D::default()),
        );

<<<<<<< HEAD
    let mut game = Application::new(
        assets_dir,
        AssetsExample {
            prefab_handle: None,
        },
        dispatcher_builder,
    )?;
=======
    let game = Application::build(assets_dir, AssetsExample)?.build(game_data)?;
>>>>>>> origin/legion_v2
    game.run();
    Ok(())
}
