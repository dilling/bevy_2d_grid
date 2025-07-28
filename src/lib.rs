mod render;

use bevy::{
    prelude::*,
    render::{
        sync_world::SyncToRenderWorld,
        view::{self, NoFrustumCulling, RenderVisibleEntities, VisibilityClass},
    },
};

pub struct InfiniteGrid2DPlugin;

impl Plugin for InfiniteGrid2DPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        render::render_app_builder(app);
    }
}

#[derive(Component, Default)]
pub struct InfiniteGrid2D;

#[derive(Component, Copy, Clone)]
#[require(VisibilityClass)]
#[component(on_add = view::add_visibility_class::<InfiniteGrid2DSettings>)]
pub struct InfiniteGrid2DSettings {
    pub x_axis_color: Color,
    pub y_axis_color: Color,
    pub line_color: Color,
    pub grid_size: f32,
    pub sort_key: f32,
}

impl Default for InfiniteGrid2DSettings {
    fn default() -> Self {
        Self {
            x_axis_color: Color::srgb(1.0, 0.2, 0.2),
            y_axis_color: Color::srgb(0.2, 1.0, 0.2),
            line_color: Color::srgb(0.25, 0.25, 0.25),
            grid_size: 100.0,
            sort_key: 0.0,
        }
    }
}

#[derive(Bundle, Default)]
pub struct InfiniteGrid2DBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub settings: InfiniteGrid2DSettings,
    pub grid: InfiniteGrid2D,
    pub visibility: Visibility,
    pub view_visibility: ViewVisibility,
    pub inherited_visibility: InheritedVisibility,
    pub shadow_casters: RenderVisibleEntities,
    pub no_frustum_culling: NoFrustumCulling,
    pub sync_to_render_world: SyncToRenderWorld,
}
