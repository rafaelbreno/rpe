use glium::Surface;

#[macro_use]
extern crate glium;

extern crate image;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

mod teapot;

implement_vertex!(Vertex, position, tex_coords);

fn main() {
    use glium::glutin;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES).unwrap();
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = -0.5;

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }

        t += 0.002;
        if t > 0.5 {
            t = -0.5;
        }

        // static
        //let uniforms = uniform! {
            //matrix: [
                //[1.0, 0.0, 0.0, 0.0],
                //[0.0, 1.0, 0.0, 0.0],
                //[0.0, 0.0, 1.0, 0.0],
                //[0.0, 0.0, 0.0, 1.0f32],
            //],
        //};

        // moving left to right
        //let uniforms = uniform! {
            //matrix: [
                //[1.0, 0.0, 0.0, 0.0],
                //[0.0, 1.0, 0.0, 0.0],
                //[0.0, 0.0, 1.0, 0.0],
                //[t, 0.0, 0.0, 1.0f32],
            //]
        //};

        // rotating
        //let uniforms = uniform! {
            //matrix: [
                //[t.cos(), t.sin(), 0.0, 0.0],
                //[-t.sin(), t.cos(), 0.0, 0.0],
                //[0.0, 0.0, 1.0, 0.0],
                //[0.0, 0.0, 0.0, 1.0f32],
            //]
        //};
        //
        let matrix = [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
        ];

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(
                (&positions, &normals), 
                &indices,
                &program, 
                &uniform! { matrix: matrix },
                &Default::default()
            ).unwrap();
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() + 
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}
