use miniquad::*;

pub const VERTEX: &str = r#"
    #version 330 core

    layout(location = 0)in vec4 vertex_pos;

    uniform vec3 light_pos;
    uniform vec4 color;
    uniform mat4 m;
    uniform mat4 vp;

    out vec4 world_position;
    out vec4 vertex_color;
    out vec3 light_position;

    void main()
    {
        world_position = m * vertex_pos;
        gl_Position = vp * world_position;
        vertex_color = color;
        light_position = light_pos;
    }
"#;

pub const FRAGMENT: &str = r#"
    #version 330 core
    in vec4 vertex_color;
    in vec4 world_position;
    in vec3 light_position;
    out vec4 fragment_color;

    float gaussian(float distance, float intensity, float falloff)
    {
        return (distance <= intensity) ? 1.0 : exp(-((distance - intensity) * (distance - intensity)) / (2.0 * falloff * falloff));
    }

    void main()
    {
        float distance = length(world_position.xyz - light_position);
        fragment_color = vertex_color * gaussian(distance, 24.0, 3);
    }
"#;

pub fn meta() -> ShaderMeta
{
    ShaderMeta {
        images: vec![],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("m", UniformType::Mat4),
                UniformDesc::new("vp", UniformType::Mat4),
                UniformDesc::new("color", UniformType::Float4),
                UniformDesc::new("light_pos", UniformType::Float3)
            ]
        },
    }
}

#[repr(C)]
pub struct Uniforms {
    pub m: glam::Mat4,
    pub vp: glam::Mat4,
    pub color: glam::Vec4,
    pub light_pos: glam::Vec3
}