use miniquad::*;
use transform::Transform;

mod cube_shader;
mod cube;
mod transform;

struct App
{
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
}

impl App
{
    pub fn new() -> App
    {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();
        log::info!("OpenGL Version: {}", ctx.info().gl_version_string);

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&cube::VERTICES),
        );
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&cube::INDICES),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                match ctx.info().backend {
                    Backend::OpenGl => ShaderSource::Glsl {
                        vertex: cube_shader::VERTEX,
                        fragment: cube_shader::FRAGMENT,
                    },
                    Backend::Metal => panic!("Metal not supported!"),
                },
                cube_shader::meta(),
            ).expect("Could not compile passthrough_shader");

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("vertex_pos", VertexFormat::Float4)
            ],
            shader,
            PipelineParams::default(),
        );

        App {
            pipeline,
            bindings,
            ctx,
        }
    }
}

impl EventHandler for App
{
    fn update(&mut self)
    {

    }

    fn draw(&mut self)
    {
        // Clear the screen with nice defaults.
        self.ctx.begin_default_pass(PassAction::Clear {
            color: Some((0.12, 0.12, 0.12, 1.0)),
            depth: Default::default(),
            stencil: Default::default()
        });

        // Calculate the camera's matrixes.
        let (width, height) = window::screen_size();
        let camera_projection = glam::Mat4::perspective_rh_gl(
            40.0f32.to_radians(), 
            width / height, 
            0.01, 50.0
        );
        let camera_view = glam::Mat4::look_at_rh(
            glam::vec3(30.0, 30.0, 0.0),
            glam::vec3(0.0, 10.0, -10.0),
            glam::vec3(0.0, 1.0, 0.0),
        );
        let vp = camera_projection * camera_view;

        // Apply our pipelines.
        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        
        // Draw the first cube.
        let transform_first = Transform{
            translation: glam::vec3(0., 0., 0.),
            scaling: glam::vec3(1., 1., 1.),
            rotation: glam::vec3(0., 0., 0.)
        };
        self.ctx.apply_uniforms(UniformsSource::table(&cube_shader::Uniforms {
            m: transform_first.get_matrix(), vp,
            color: glam::vec4(1., 1., 1., 1.),
            light_pos: glam::vec3(0., 0., 0.)
        }));
        self.ctx.draw(0, 36, 1);

        // Draw the second cube.
        let transform_second = Transform{
            translation: glam::vec3(0., 20., -20.),
            scaling: glam::vec3(1., 1., 1.),
            rotation: glam::vec3(0., 0., 0.)
        };
        self.ctx.apply_uniforms(UniformsSource::table(&cube_shader::Uniforms {
            m: transform_second.get_matrix(), vp,
            color: glam::vec4(0., 0., 1., 1.),
            light_pos: glam::vec3(0., 0., 0.)
        }));
        self.ctx.draw(0, 36, 1);

        self.ctx.end_render_pass(); // End draw batch.

        // Execute the drawings.
        self.ctx.commit_frame();
    }
}

fn main()
{
    // Init logging.
    env_logger::init();

    // Prepare window.
    let mut conf = conf::Conf::default();
    conf.window_title = "Grafica".to_string();
    conf.window_width = 640;
    conf.window_height = 480;
    conf.window_resizable = false;
    conf.platform.swap_interval = Some(1);

    // Use Metal on Apple devices if it has been given as an argument.
    /*let metal = std::env::args().nth(1).as_deref() == Some("metal");
    conf.platform.apple_gfx_api = if metal {
        conf::AppleGfxApi::Metal
    } else {
        conf::AppleGfxApi::OpenGl
    };*/

    // Start window.
    miniquad::start(conf, move || Box::new(App::new()));
}
