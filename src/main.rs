#[macro_use]
extern crate glium;
extern crate cgmath;

mod support;

fn main() {
    use glium::{glutin, Surface};
    use std::path::Path;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let model_file = "/Users/mtg2/Projects/glium-scratch/bird.obj";

    let (vertex_buffer, scale) = support::load_wavefront(&display, Path::new(&model_file));

    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;
                uniform float scaling;
                in vec3 position;
                in vec3 normal;
                in vec3 color_diffuse;
                in vec4 color_specular;
                out vec3 v_position;
                out vec3 v_normal;
                out vec3 v_color_diffuse;
                out vec4 v_color_specular;
                void main() {{
                    v_position = position;
                    v_normal = normal;
                    v_color_diffuse = color_diffuse;
                    v_color_specular = color_specular;
                    gl_Position = persp_matrix * view_matrix * vec4(v_position * scaling, 1.0);
                }}
            ",
            fragment: "
                #version 140
                uniform vec3 eye_pos;
                uniform vec3 light_dir;
                in vec3 v_position;
                in vec3 v_normal;
                in vec3 v_color_diffuse;
                in vec4 v_color_specular;
                out vec4 f_color;
                void main() {
                    vec3 normal = v_normal;
                    // If we don't have normals, use derivative of position to compute
                    if (dot(normal, normal) < 0.001) {
                        normal = normalize(cross(dFdx(v_position), dFdy(v_position)));
                    }
                    vec3 l = normalize(-light_dir);
                    vec3 view_dir = normalize(eye_pos - v_position);
                    float n_dot_l = clamp(dot(normal, l), 0.0, 1.0);
                    vec3 color = (0.1 + n_dot_l * 0.5) * v_color_diffuse;
                    vec3 half_vec = normalize(l + view_dir);
                    float n_dot_h = clamp(dot(normal, half_vec), 0.0, 1.0);
                    if (n_dot_h > 0.0) {
                        color += 0.5 * pow(n_dot_h, v_color_specular.a) * v_color_specular.rgb;
                    }
                    f_color = vec4(color, 1.0);
                }
            ",
        },
    ).unwrap();

    let mut camera = support::camera::CameraState::new();
    let mut mouse_pressed = [false; 3];
    let mut mouse_pos = (0.0, 0.0);

    let mut closed = false;
    while !closed {
        camera.update();

        let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
            scaling: scale,
            eye_pos: camera.get_position(),
            light_dir: camera.get_direction(),
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => ()
                },
                _ => (),
            }
        });
    }
}