#[cfg(feature = "webgpu")]
pub mod webgpu_preview {
    use wgpu::*;
    use winit::window::Window;

    pub struct WebGPURenderer {
        device: Device,
        queue: Queue,
        surface: Surface<'static>,     
        config: SurfaceConfiguration,
        render_pipeline: RenderPipeline,
    }

    impl WebGPURenderer {
        pub async fn new(window: std::sync::Arc<Window>) -> Self {
            let instance = Instance::new(InstanceDescriptor {
                backends: Backends::all(),
                dx12_shader_compiler: Default::default(),
                flags: InstanceFlags::default(),
                gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
            });

            // Using wgpu version ~0.19/0.20 Surface::create_surface syntax
            let surface = instance.create_surface(window.clone()).unwrap();

            let adapter = instance.request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }).await.unwrap();

            let (device, queue) = adapter.request_device(
                &DeviceDescriptor {
                    label: Some("Daithon PRISMA Device"),
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                },
                None,
            ).await.unwrap();

            let size = window.inner_size();
            let config = SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_capabilities(&adapter).formats[0],
                width: size.width,
                height: size.height,
                present_mode: PresentMode::Fifo,
                alpha_mode: CompositeAlphaMode::Auto,
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };

            surface.configure(&device, &config);

            let shader_source = "
                @vertex
                fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                    let x = f32(i32(in_vertex_index) - 1);
                    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
                    return vec4<f32>(x, y, 0.0, 1.0);
                }
                @fragment
                fn fs_main() -> @location(0) vec4<f32> {
                    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
                }
            ";

            let shader = device.create_shader_module(ShaderModuleDescriptor {
                label: Some("PBR Shader"),
                source: ShaderSource::Wgsl(shader_source.into()),
            });

            let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

            let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(ColorTargetState {
                        format: config.format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    polygon_mode: PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

            Self {
                device,
                queue,
                surface,
                config,
                render_pipeline,
            }
        }

        pub fn render(&mut self, _mesh: &crate::prisma::vertex::mesh_generator::HighQualityMesh) -> Result<(), SurfaceError> {
            let output = self.surface.get_current_texture()?;
            let view = output.texture.create_view(&TextureViewDescriptor::default());

            let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(Color {
                                r: 0.1,
                                g: 0.1,
                                b: 0.1,
                                a: 1.0,
                            }),
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.draw(0..3, 0..1);
            }

            self.queue.submit(std::iter::once(encoder.finish()));
            output.present();

            Ok(())
        }
    }
}
