use glam::Mat4;
use miniquad::*;

use crate::graphics::Vertex;

pub struct RendererData {
    pub draw_calls: Vec<DrawCall>,
    pub draw_calls_binding: Vec<Bindings>,
    pub draw_calls_count: usize,
    pub view_proj: Mat4,
}

impl RendererData {
    pub fn new() -> Self {
        Self {
            draw_calls: Vec::with_capacity(100),
            draw_calls_binding: Vec::with_capacity(100),
            draw_calls_count: 0,
            view_proj: Mat4::IDENTITY,
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
}

/// The console renderer
pub struct Renderer {
    display_pipeline: Pipeline,
    display_bind: Bindings,
    offscreen_pipeline: Pipeline,
    offscreen_pass: RenderPass,
    ctx: Box<dyn RenderingBackend>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let color_img = ctx.new_render_texture(TextureParams {
            width: 320,
            height: 200,
            format: TextureFormat::RGBA8,
            min_filter: FilterMode::Nearest,
            mag_filter: FilterMode::Nearest,
            ..Default::default()
        });
        let depth_img = ctx.new_render_texture(TextureParams {
            width: 320,
            height: 200,
            format: TextureFormat::Depth,
            ..Default::default()
        });

        // offscreen
        let offscreen_pass = ctx.new_render_pass(color_img, Some(depth_img));

        let offscreen_shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: offscreen_shader::VERTEX,
                    fragment: offscreen_shader::FRAGMENT,
                },
                offscreen_shader::meta(),
            )
            .unwrap();

        let offscreen_pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float3),
                VertexAttribute::new("in_color", VertexFormat::Float4),
                VertexAttribute::new("in_normal", VertexFormat::Float3),
            ],
            offscreen_shader,
        );

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
            display_pipeline,
            display_bind,
            offscreen_pipeline,
            offscreen_pass,
            ctx,
        }
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
            PassAction::clear_color(0.0, 0.0, 0.0, 1.0),
        );

        self.ctx.apply_pipeline(&self.offscreen_pipeline);

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

                let vs_params = offscreen_shader::Uniforms {
                    mvp: data.view_proj * draw.model,
                };

            self.ctx.apply_bindings(bindings);
            self.ctx.apply_uniforms(UniformsSource::table(&vs_params));
            self.ctx.draw(0, draw.indices.len() as i32, 1);
        }

        self.ctx.end_render_pass();
        

        // display pass
        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.display_pipeline);
        self.ctx.apply_bindings(&self.display_bind);
        self.ctx.draw(0, 6, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}

mod offscreen_shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec3 in_pos;
    attribute vec4 in_color;

    uniform mat4 mvp;

    varying lowp vec4 color;

    void main() {
        gl_Position = mvp * vec4(in_pos, 1.0);
        color = in_color;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;

    void main() {
        gl_FragColor = color;
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat4,
    }
}

mod display_shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_uv;

    varying lowp vec2 uv;

    void main() {
        gl_Position = vec4(in_pos, 0.0, 1.0);
        uv = in_uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 uv;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, uv);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}