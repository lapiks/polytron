use egui_miniquad::EguiMq;
use glam::{uvec2, vec2, vec3, Mat4, UVec2};
use miniquad::*;

use crate::{color::Color, graphics::{Rect2d, Vertex}, gui::Gui};

pub const IMAGE_RES: UVec2 = uvec2(320, 200);
pub const IMAGE_RATIO_XY: f32 = IMAGE_RES.x as f32 / IMAGE_RES.y as f32;
pub const IMAGE_RATIO_YX: f32 = IMAGE_RES.y as f32 / IMAGE_RES.x as f32;

#[derive(Clone, Copy, PartialEq)]
pub enum Primitive {
    Lines,
    Triangles,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Mode2d,
    Mode3d,
}

pub struct RendererData {
    pub draw_calls: Vec<DrawCall>,
    pub draw_calls_binding: Vec<Bindings>,
    pub draw_calls_count: usize,
    pub view_proj: Mat4,
    pub mode: Mode,
    pub viewport: Rect2d,
    pub background: Color,
}

impl RendererData {
    pub fn new() -> Self {
        Self {
            draw_calls: Vec::with_capacity(100),
            draw_calls_binding: Vec::with_capacity(100),
            draw_calls_count: 0,
            view_proj: Mat4::IDENTITY,
            mode: Mode::Mode3d,
            viewport: Rect2d {
                position: vec2(0.0, 0.0),
                size: vec2(1.0, 1.0)
            },
            background: Color::black(),
        }
    }

    pub fn begin_frame(&mut self) {
        self.draw_calls_count = 0;
    }
}

pub struct DrawCall {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i32>,
    pub model: Mat4,
    pub view_proj: Mat4,
    pub primitive: Primitive,
    pub mode: Mode,
    pub viewport: Rect2d,
    pub background: Color,
}

/// The console renderer
pub struct Renderer {
    screen_res: UVec2,
    display_pipeline: Pipeline,
    display_bind: Bindings,
    pipelines: [Pipeline; 4],
    offscreen_pass: RenderPass,
    ctx: Box<dyn RenderingBackend>,
    egui_mq: egui_miniquad::EguiMq,
}

impl Renderer {
    const TRIANGLES_PIPELINE_3D: usize = 0;
    const LINES_PIPELINE_3D: usize = 1;
    const TRIANGLES_PIPELINE_2D: usize = 2;
    const LINES_PIPELINE_2D: usize = 3;

    pub fn new() -> Renderer {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let color_img = ctx.new_render_texture(TextureParams {
            width: IMAGE_RES.x,
            height: IMAGE_RES.y,
            format: TextureFormat::RGBA8,
            min_filter: FilterMode::Nearest,
            mag_filter: FilterMode::Nearest,
            ..Default::default()
        });
        let depth_img = ctx.new_render_texture(TextureParams {
            width: IMAGE_RES.x,
            height: IMAGE_RES.y,
            format: TextureFormat::Depth,
            ..Default::default()
        });

        // offscreen
        let offscreen_pass = ctx.new_render_pass(color_img, Some(depth_img));

        let shader_3d = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader_3d::VERTEX,
                    fragment: shader_3d::FRAGMENT,
                },
                shader_3d::meta(),
            )
            .unwrap();

        let shader_2d = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader_2d::VERTEX,
                    fragment: shader_2d::FRAGMENT,
                },
                shader_2d::meta(),
            )
            .unwrap();

        let triangles_pipeline_3d = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float3),
                VertexAttribute::new("in_color", VertexFormat::Float4),
                VertexAttribute::new("in_normal", VertexFormat::Float3),
            ],
            shader_3d,
            PipelineParams {
                primitive_type: PrimitiveType::Triangles,
                depth_write: true,
                depth_test: Comparison::LessOrEqual,
                ..Default::default()
            }
        );

        let lines_pipeline_3d = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float3),
                VertexAttribute::new("in_color", VertexFormat::Float4),
                VertexAttribute::new("in_normal", VertexFormat::Float3),
            ],
            shader_3d,
            PipelineParams {
                primitive_type: PrimitiveType::Lines,
                ..Default::default()
            }
        );

        let triangles_pipeline_2d = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float3),
                VertexAttribute::new("in_color", VertexFormat::Float4),
                VertexAttribute::new("in_normal", VertexFormat::Float3),
            ],
            shader_2d,
            PipelineParams {
                primitive_type: PrimitiveType::Triangles,
                depth_write: true,
                depth_test: Comparison::LessOrEqual,
                ..Default::default()
            }
        );

        let lines_pipeline_2d = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float3),
                VertexAttribute::new("in_color", VertexFormat::Float4),
                VertexAttribute::new("in_normal", VertexFormat::Float3),
            ],
            shader_2d,
            PipelineParams {
                primitive_type: PrimitiveType::Lines,
                ..Default::default()
            }
        );

        let pipelines = [
            triangles_pipeline_3d, 
            lines_pipeline_3d,
            triangles_pipeline_2d, 
            lines_pipeline_2d,
        ];

        // display pass
        let quad_vertices = [
            -1.0, -1.0, 0.0, 0.0,
            -1.0, 1.0, 0.0, 1.0,
            1.0, -1.0, 1.0, 0.0,
            1.0, 1.0, 1.0, 1.0
        ] as [f32; 16];
        let quad_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&quad_vertices),
        );

        let quad_index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&[0, 1, 2, 2, 1, 3]),
        );

        let display_bind = Bindings {
            vertex_buffers: vec![quad_vertex_buffer],
            index_buffer: quad_index_buffer,
            images: vec![color_img],
        };

        let display_shader = ctx
        .new_shader(
            ShaderSource::Glsl {
                vertex: display_shader::VERTEX,
                fragment: display_shader::FRAGMENT,
            },
            display_shader::meta(),
        )
        .unwrap();

        let display_pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
            ],
            display_shader,
        );

        Renderer {
            screen_res: IMAGE_RES,
            display_pipeline,
            display_bind,
            pipelines,
            offscreen_pass,
            egui_mq: egui_miniquad::EguiMq::new(&mut *ctx),
            ctx,
        }
    }

    pub fn set_screen_resolution(&mut self, screen_res: UVec2) {
        self.screen_res = screen_res;
    }

    pub fn draw(&mut self, data: &mut RendererData) {
        for _ in 0..data.draw_calls.len() - data.draw_calls_binding.len() {
            let vertex_buffer = self.ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Immutable,
                BufferSource::empty::<Vertex>(4096),
            );

            let index_buffer = self.ctx.new_buffer(
                BufferType::IndexBuffer,
                BufferUsage::Immutable,
                BufferSource::empty::<i32>(4096),
            );

            let bindings = Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer,
                images: vec![],
            };

            data.draw_calls_binding.push(bindings);
        }

        // offscreen pass
        self.ctx.begin_pass(
            Some(self.offscreen_pass),
            PassAction::clear_color(
                0.0, 0.0, 0.0, 0.0
            ),
        );

        let mut previous_background = Color::black();

        for (draw, bindings) in data.draw_calls
            .iter()
            .zip(data.draw_calls_binding.iter()) {
                self.ctx.buffer_update(
                    bindings.vertex_buffers[0], 
                    BufferSource::slice(&draw.vertices)
                );

                self.ctx.buffer_update(
                    bindings.index_buffer, 
                    BufferSource::slice(&draw.indices)
                );

                let vs_params = shader_3d::Uniforms {
                    model: draw.model,
                    view_proj: draw.view_proj,
                };

                self.ctx.apply_pipeline(
                    &self.get_pipeline(draw.primitive, draw.mode)
                );
                self.ctx.apply_bindings(bindings);
                self.ctx.apply_uniforms(UniformsSource::table(&vs_params));

                self.ctx.apply_scissor_rect(
                    (draw.viewport.position.x * IMAGE_RES.x as f32) as i32, 
                    (draw.viewport.position.y * IMAGE_RES.y as f32) as i32,
                    (draw.viewport.size.x * IMAGE_RES.x as f32) as i32, 
                    (draw.viewport.size.y * IMAGE_RES.y as f32) as i32,
                );
                self.ctx.apply_viewport(
                    (draw.viewport.position.x * IMAGE_RES.x as f32) as i32, 
                    (draw.viewport.position.y * IMAGE_RES.y as f32) as i32,
                    (draw.viewport.size.x * IMAGE_RES.x as f32) as i32, 
                    (draw.viewport.size.y * IMAGE_RES.y as f32) as i32,
                );
                let background = draw.background;
                if previous_background != background {
                    self.ctx.clear(
                        Some((
                            background.r,
                            background.g,
                            background.b,
                            background.a    
                        )),
                        None, 
                        None
                    );
                    previous_background = background;
                }
                self.ctx.draw(0, draw.indices.len() as i32, 1);
        }

        self.ctx.end_render_pass();

        // draw to fullscreen quad
        {
            // display pass
            self.ctx.begin_default_pass(Default::default());
            self.ctx.apply_pipeline(&self.display_pipeline);
            self.ctx.apply_bindings(&self.display_bind);
            let scale = if self.screen_res.x as f32 / self.screen_res.y as f32 > IMAGE_RATIO_XY {
                vec3(
                    (self.screen_res.y as f32 / self.screen_res.x as f32) * IMAGE_RATIO_XY,
                    1.0, 
                    1.0
                )
            } else {
                vec3(
                    1.0,  
                    (self.screen_res.x as f32 / self.screen_res.y as f32) * IMAGE_RATIO_YX,
                    1.0
                )
            };
            let vs_params = display_shader::Uniforms {
                model: Mat4::from_scale(scale),
            };
            self.ctx.apply_uniforms(UniformsSource::table(&vs_params));
            self.ctx.draw(0, 6, 1);
            self.ctx.end_render_pass();
        }
    }

    pub fn draw_ui(&mut self, gui: &mut Gui) {
        let mq_texture = self.ctx.render_pass_texture(self.offscreen_pass);
        // create egui TextureId from Miniquad GL texture Id
        let raw_id = match unsafe { self.ctx.texture_raw_id(mq_texture) } {
            miniquad::RawId::OpenGl(id) => id as u64,
        };
        let egui_texture_id = egui::TextureId::User(raw_id);

        // prepare drawing the ui by clearing background
        self.ctx.begin_default_pass(
            miniquad::PassAction::clear_color(0.0, 0.0, 0.0, 1.0)
        );
        self.ctx.end_render_pass();

        // run the ui code
        self.egui_mq.run(&mut *self.ctx, |_mq_ctx, egui_ctx| {
            gui.draw(egui_ctx, egui_texture_id);
        });

        self.egui_mq.draw(&mut *self.ctx);
    }

    pub fn commit_frame(&mut self) {
        self.ctx.commit_frame();
    }

    pub fn egui_mq_mut(&mut self) -> &mut EguiMq {
        &mut self.egui_mq
    }

    fn get_pipeline(&self, primitive: Primitive, mode: Mode) -> Pipeline {
        match mode {
            Mode::Mode3d => {
                match primitive {
                    Primitive::Lines => self.pipelines[Self::LINES_PIPELINE_3D],
                    Primitive::Triangles => self.pipelines[Self::TRIANGLES_PIPELINE_3D],
                }
            },
            Mode::Mode2d => {
                match primitive {
                    Primitive::Lines => self.pipelines[Self::LINES_PIPELINE_2D],
                    Primitive::Triangles => self.pipelines[Self::TRIANGLES_PIPELINE_2D],
                }
            },
        }
        
    }
}

mod shader_3d {
    use glam::Mat4;
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 140
        in vec3 in_pos;
        in vec4 in_color;
        in vec3 in_normal;

        uniform mat4 model;
        uniform mat4 view_proj;

        flat out lowp vec4 polygon_color;

        void main() {
            vec3 light_color = vec3(1.0);
            vec3 light_dir = normalize(-vec3(-1.0, -1.0, -1.0));
            vec3 ambient = 0.2 * light_color;
            vec3 world_normal = mat3(transpose(inverse(model))) * in_normal;  
            vec3 diffuse = max(dot(world_normal, light_dir), 0.0) * light_color;
            polygon_color = vec4((ambient + diffuse) * in_color.xyz, 1.0);
            gl_Position = view_proj * model * vec4(in_pos, 1.0);
        }"#;

    pub const FRAGMENT: &str = r#"#version 140
        flat in lowp vec4 polygon_color;
        out vec4 color;

        void main() {
            color = polygon_color;
        }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("model", UniformType::Mat4),
                    UniformDesc::new("view_proj", UniformType::Mat4),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub model: Mat4,
        pub view_proj: Mat4,
    }
}

mod shader_2d {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 140
        in vec3 in_pos;
        in vec4 in_color;

        flat out lowp vec4 polygon_color;

        void main() {
            polygon_color = in_color;
            gl_Position = vec4(in_pos, 1.0);
        }"#;

    pub const FRAGMENT: &str = r#"#version 140
        flat in lowp vec4 polygon_color;
        out vec4 color;

        void main() {
            color = polygon_color;
        }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![],
            },
        }
    }
}

mod display_shader {
    use glam::Mat4;
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 140
    in vec2 in_pos;
    in vec2 in_uv;

    out lowp vec2 uv;

    uniform mat4 model;

    void main() {
        gl_Position = model * vec4(in_pos, 0.0, 1.0);
        uv = in_uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 140
    in lowp vec2 uv;

    uniform sampler2D tex;

    out vec4 color;

    void main() {
        color = texture2D(tex, uv);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout { 
                uniforms: vec![
                    UniformDesc::new("model", UniformType::Mat4),
                ] 
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub model: Mat4,
    }
}