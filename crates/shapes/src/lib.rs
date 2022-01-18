use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{MaterialPipeline, SpecializedMaterial},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_graph::RenderGraph,
        render_resource::{
            self,
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer,
            BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages,
            RenderPipelineDescriptor, ShaderStages,
        },
        renderer::RenderDevice,
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin, Mesh2dHandle},
};

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct ColorMaterial {
    color: Color,
}

#[derive(Clone)]
pub struct GpuColorMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

/*
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127f"]
pub struct CircleGaugeMaterial {
    pub color: Color,
    pub ratio: f32,
}*/

pub struct ShapeMeshes {
    pub quad2x2: Handle<Mesh>,
    pub mat_white: Handle<ColorMaterial>,
    pub mat_orange: Handle<ColorMaterial>,
    pub mat_fuchsia: Handle<ColorMaterial>,
    pub mat_green: Handle<ColorMaterial>,
    pub mat_gray: Handle<ColorMaterial>,
    //pub mat_circle_gauge: Handle<CircleGaugeMaterial>,
}

pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_shapes)
            .add_plugin(Material2dPlugin::<ColorMaterial>::default());
        //            .add_asset::<CircleGaugeMaterial>();
    }
}

pub fn init_shapes(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_color: ResMut<Assets<ColorMaterial>>,
    mut materials_bevy: ResMut<Assets<bevy::prelude::ColorMaterial>>,
    //mut materials_circle_gauge: ResMut<Assets<CircleGaugeMaterial>>,
) {
    // Watch for changes
    asset_server.watch_for_changes().unwrap();

    let m = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(2f32, 2f32),
        flip: false,
    }));
    commands.insert_resource(ShapeMeshes {
        quad2x2: //meshes.add(Mesh::from(shape::Cube { size: 1.0 })).into(),
        m.into(),
        mat_white: materials_color.add(ColorMaterial {
            color: Color::WHITE,
        }),
        // mat_green: materials_bevy.add(bevy::prelude::ColorMaterial::from(Color::GREEN)), /*
        mat_green: materials_color.add(ColorMaterial {
            color: Color::GREEN,
        }), // */
        mat_orange: materials_color.add(ColorMaterial {
            color: Color::ORANGE_RED,
        }),
        mat_fuchsia: materials_color.add(ColorMaterial {
            color: Color::FUCHSIA,
        }),
        mat_gray: materials_color.add(ColorMaterial { color: Color::GRAY }),
        /*mat_circle_gauge: materials_circle_gauge.add(CircleGaugeMaterial {
            ratio: 0.5f32,
            color: Color::BEIGE,
        }),*/
    })
}

impl RenderAsset for ColorMaterial {
    type ExtractedAsset = ColorMaterial;
    type PreparedAsset = GpuColorMaterial;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let color = Vec4::from_slice(&extracted_asset.color.as_linear_rgba_f32());
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: color.as_std140().as_bytes(),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
            layout: &material_pipeline.material2d_layout,
        });

        Ok(GpuColorMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl Material2d for ColorMaterial {
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        #[cfg(not(target_arch = "wasm32"))]
        return Some(asset_server.load("../../logic/assets/shaders/custom_material.wgsl"));
        #[cfg(target_arch = "wasm32")]
        return Some(asset_server.load("assets/shaders/custom_material.wgsl"));
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
                },
                count: None,
            }],
            label: None,
        })
    }
}
