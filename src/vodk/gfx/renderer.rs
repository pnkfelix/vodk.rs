
pub struct Vec2 { x: f32, y:f32 }
pub struct Vec3 { x: f32, y:f32, z:f32 }
pub struct Vec4 { x: f32, y:f32, z:f32, w: f32 }
pub struct Mat3 {
    _00: f32, _10:f32, _20:f32,
    _01: f32, _11:f32, _21:f32,
    _02: f32, _12:f32, _22:f32,
}
pub struct Mat4 {
    _00: f32, _10:f32, _20:f32, _30:f32,
    _01: f32, _11:f32, _21:f32, _31:f32,
    _02: f32, _12:f32, _22:f32, _32:f32,
    _03: f32, _13:f32, _23:f32, _33:f32,
}

pub enum BackendType {
    GL_BACKEND,
    INVALID_NACKEND,
}

pub enum ShaderType {
    FRAGMENT_SHADER,
    VERTEX_SHADER,
    GEOMETRY_SHADER,
}

pub type TextureFlags = i32;
pub static TEXTURE_REPEAT_S          : TextureFlags = 1;
pub static TEXTURE_REPEAT_T          : TextureFlags = 2;
pub static TEXTURE_REPEAT            : TextureFlags = 3;
pub static TEXTURE_CLAMP_S           : TextureFlags = 4;
pub static TEXTURE_CLAMP_T           : TextureFlags = 8;
pub static TEXTURE_CLAMP             : TextureFlags = 12;
pub static TEXTURE_MIN_FILTER_LINEAR : TextureFlags = 16;
pub static TEXTURE_MAG_FILTER_LINEAR : TextureFlags = 32;
pub static TEXTURE_FILTER_LINEAR     : TextureFlags = 48;
pub static TEXTURE_MIN_FILTER_NEAREST: TextureFlags = 64;
pub static TEXTURE_MAG_FILTER_NEAREST: TextureFlags = 128;
pub static TEXTURE_SAMPLE_NEAREST    : TextureFlags = 192;

pub enum DrawMode {
    LINES,
    LINE_LOOP,
    LINE_STRIP,
    TRIANGLES,
    TRIANGLE_STRIP
}

pub enum Feature {
    FRAGMENT_SHADING,
    VERTEX_SHADING,
    GEOMETRY_SHADING,
    RENDER_TO_TEXTURE,
    MULTIPLE_RENDER_TARGETS,
}

pub enum AttributeType {
    F32,
    F64,
    I32,
    U32,
}

pub enum PixelFormat {
    FORMAT_R8G8B8A8,
    FORMAT_R8G8B8X8,
    FORMAT_B8G8R8A8,
    FORMAT_B8G8R8X8,
    FORMAT_A8,
}

pub enum UpdateHint {
    STATIC_UPDATE,
    STREAM_UPDATE,
    DYNAMIC_UPDATE
}

pub enum ShaderResult {
    COMPILE_SUCCESS,
    COMPILE_ERROR(~str),
}

pub enum ProgramResult {
    LINK_SUCCESS,
    LINK_ERROR(~str),
}

pub trait RenderingContext {
    fn is_supported(&mut self, f: Feature) -> bool;
    fn set_viewport(&mut self, x:i32, y:i32, w:i32, h:i32);
    fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32);
    fn make_current(&mut self) -> bool;

    fn create_texture(&mut self) -> Texture;
    fn destroy_texture(&mut self, tex: Texture);
    fn set_texture_flags(&mut self, tex: Texture, flags: TextureFlags);
    fn upload_texture_data(&mut self, dest: Texture,
                           data: &[u8], format: PixelFormat,
                           w:u32, h:u32, stride: u32);
    /**
     * Tells about the texture's size and format
     * Does not need to be called if some data will be uploaded
     * through upload_texture_data.
     */
    fn allocate_texture(&mut self, dest: Texture,
                        format: PixelFormat,
                        w:u32, h:u32, stride: u32);

    fn create_shader(&mut self, t: ShaderType) -> Shader;
    fn destroy_shader(&mut self, s: Shader);
    fn compile_shader(&mut self, shader: Shader, src: &str) -> ShaderResult;

    fn create_shader_program(&mut self) -> ShaderProgram;
    fn destroy_shader_program(&mut self, s: ShaderProgram);
    fn bind_shader_program(&mut self, p: ShaderProgram); // TODO maybe useless
    fn unbind_shader_program(&mut self, p: ShaderProgram); // TODO maybe useless
    fn link_shader_program(&mut self, p: ShaderProgram) -> ProgramResult;
    fn attach_shader(&mut self, p: ShaderProgram, s: Shader);

    fn create_vertex_buffer(&mut self) -> VertexBuffer;
    fn destroy_vertex_buffer(&mut self, buffer: VertexBuffer);
    fn bind_vertex_buffer(&mut self, buffer: VertexBuffer);
    fn unbind_vertex_buffer(&mut self, buffer: VertexBuffer);
    fn upload_vertex_data(&mut self, buffer: VertexBuffer,
                          data: &[u8], update: UpdateHint);

    fn get_uniform_location(&mut self, shader: Shader, name: &str) -> i32;

    fn define_vertex_attribute(attrib_index: u32,
                               attrib_type: AttributeType,
                               component_per_vertex: i32,
                               stride: i32, // zero means tightly packed attributes
                               offset: i32);

    fn draw_arrays(&mut self, mode: DrawMode, first: i32, count: i32);
    fn draw(&mut self, mode: DrawMode,
            p: ShaderProgram,
            inputs: &Iterator<ShaderInput>);
}

type Handle = u32;

pub struct Shader { handle: Handle }
pub struct ShaderProgram { handle: Handle }
pub struct Texture { handle: Handle }
pub struct VertexBuffer { handle: Handle }
pub struct ElementBuffer { handle: Handle }
pub struct GeometryBuffer { handle: Handle }

pub enum ShaderInputValue {
    INPUT_FLOATS(~[f32]),
    INPUT_TEXTURE(Texture),
}

pub struct ShaderInput {
    location: i32,
    value: ShaderInputValue,
}

/*

fn draw(...) {
    for input in shader_input {
        match input.value {
            INPUT_F(f) => { gl::Uniform1f(input.location, f); }
            INPUT_VEC2(v) => { gl::Uniform2f(input.location, v.x, v.y); }
        }
    }
}

*/