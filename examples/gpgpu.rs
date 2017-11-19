#[macro_use]
extern crate glium;
extern crate rand;
use glium::glutin;

fn main() {
    /*let context = glutin::HeadlessRendererBuilder::new(1024, 1024).build().unwrap();
    let display = glium::HeadlessRenderer::new(context).unwrap();*/
    let events_loop = glutin::EventsLoop::new();    
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    

    let program = glium::program::ComputeShader::from_source(&display, r#"\

            #version 430
            layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

            layout(std430) buffer MyBlock {
                float power2;
                float padding1;
                float padding2;
                float padding3;
                float values[];
            };

            void main() {
                float val = values[gl_GlobalInvocationID.x];
                values[gl_GlobalInvocationID.x] = pow(val, power2);
            }

        "#).unwrap();

    #[repr(C)]
    struct Data {
        power2: f32,
        padding1: f32,
        padding2: f32,
        padding3: f32,
        values: [f32],
    }

    implement_buffer_content!(Data);
    implement_uniform_block!(Data, power2, padding1, padding2, padding3, values);

    const NUM_VALUES: usize = 4096;

    let mut buffer: glium::uniforms::UniformBuffer<Data> =
              glium::uniforms::UniformBuffer::empty_unsized(&display, 4 + 3*4 + 4 * NUM_VALUES).unwrap(); //4 bytes power, 3*4 bytes padding, 4*NUM_VALUES for values

    {
        let mut mapping = buffer.map();
        mapping.power2 = 3.0;
        for val in mapping.values.iter_mut() {
            *val = 2.0;//rand::random();
        }
    }

    program.execute(uniform! { MyBlock: &*buffer }, NUM_VALUES as u32, 1, 1);

    {
        let mapping = buffer.map();
        println!("Power is: {:?}", mapping.power2);
        for val in mapping.values.iter() {
            print!("{:?}", *val);
        }
        println!(".");
    }
}
