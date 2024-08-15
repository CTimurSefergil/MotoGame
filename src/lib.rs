mod camera;
#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod screen;
mod ui_tools;

/*use bevy::core::TaskPoolThreadAssignmentPolicy;
use bevy::tasks::available_parallelism;*/
use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};
//use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "INFINITY".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.05),
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), /*min_total_threads: 1,
                                                      max_total_threads: std::usize::MAX, // unlimited threads
                                                      io: TaskPoolThreadAssignmentPolicy {
                                                          // say we know our app is i/o intensive (asset streaming?)
                                                          // so maybe we want lots of i/o threads
                                                          min_threads: 4,
                                                          max_threads: std::usize::MAX,
                                                          percent: 0.5, // use 50% of available threads for I/O
                                                      },
                                                      async_compute: TaskPoolThreadAssignmentPolicy {
                                                          // say our app never does any background compute,
                                                          // so we don't care, but keep one thread just in case
                                                          min_threads: 1,
                                                          max_threads: 1,
                                                          percent: 0.0,
                                                      },
                                                      compute: TaskPoolThreadAssignmentPolicy {
                                                          // say we want to use at least half the CPU for compute
                                                          // (maybe over-provisioning if there are very few cores)
                                                          min_threads: available_parallelism() / 2,
                                                          // but limit it to a maximum of 8 threads
                                                          max_threads: 8,
                                                          // 1.0 in this case means "use all remaining threads"
                                                          // (that were not assigned to io/async_compute)
                                                          // (clamped to min_threads..=max_threads)
                                                          percent: 1.0,
                                                      }, */
        );

        // Add other plugins.
        app.add_plugins((
            game::plugin,
            screen::plugin,
            ui_tools::plugin,
            camera::plugin,
        ));

        //app.add_plugins(WorldInspectorPlugin::new());

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}
