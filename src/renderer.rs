use miniquad::*;

use crate::graphics::Graphics;

/// The console renderer
pub struct Renderer {
    pipeline: Pipeline,
    bindings: Option<Bindings>,
    ctx: Box<dyn RenderingBackend>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader::VERTEX,
                    fragment: shader::FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float3),
                VertexAttribute::new("in_color", VertexFormat::Float4),
                VertexAttribute::new("in_normal", VertexFormat::Float3),
            ],
            shader,
        );

        Renderer {
            pipeline,
            bindings: None,
            ctx,
        }
    }

    pub fn prepare(&mut self, graphics: Graphics) {
        let mut vertex_buffers = Vec::new();
        for shape in graphics.shapes {
            vertex_buffers.push(
                self.ctx.new_buffer(
                    BufferType::VertexBuffer,
                    BufferUsage::Immutable,
                    BufferSource::slice(&shape.vertices),
                )
            );
        }

        let indices: [u16; 3] = [0, 1, 2];
        let index_buffer = self.ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );
        
        self.bindings = Some(
            Bindings {
                vertex_buffers: vertex_buffers,
                index_buffer: index_buffer,
                images: vec![],
            }
        );
    }

    pub fn draw(&mut self) {
        match &self.bindings {
            Some(bindings) => {
                self.ctx.begin_default_pass(Default::default());

                self.ctx.apply_pipeline(&self.pipeline);
                self.ctx.apply_bindings(bindings);
                self.ctx.draw(0, 3, 1);
                self.ctx.end_render_pass();
        
                self.ctx.commit_frame();
            },
            None => println!("Error: no bindings"),
        }        
    }
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec3 in_pos;
    attribute vec4 in_color;

    varying lowp vec4 color;

    void main() {
        gl_Position = vec4(in_pos, 1);
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
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}