use std::{ffi::{CStr, CString}, ptr::{null, null_mut}};
use gl::types::{GLuint, GLint, GLchar, GLenum};

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Self, String> {
        let id =unsafe {gl::CreateShader(kind)};

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);
        }

        let mut seccess: GLint = 1;
        unsafe {gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut seccess);}

        if seccess == 0 {
            let mut len: GLint = 0;
            unsafe {gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len)}
            let error = create_whitespace_cstring_with_len(len as usize);

           unsafe { gl::GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar)};

           return Err(error.to_string_lossy().into_owned())
        };

        Ok(Shader {id})
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {gl::DeleteShader(self.id)}
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len+1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe {CString::from_vec_unchecked(buffer)}
}

pub struct Program {
    id: GLuint,
}

impl Program {
    fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let id = unsafe {
            gl::CreateProgram()
        };

        for shader in shaders {
            unsafe {gl::AttachShader(id, shader.id);}
        };

        unsafe {gl::LinkProgram(id)}

        let mut seccess: GLint = 1;
        unsafe {gl::GetProgramiv(id, gl::COMPILE_STATUS, &mut seccess)}

        if seccess == 0 {
            let mut len: GLint = 0;
            unsafe {gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len)}
            let error = create_whitespace_cstring_with_len(len as usize);

           unsafe { gl::GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar)};

           return Err(error.to_string_lossy().into_owned())
        };

        Ok(Program {id})
    }
}

/* Дописать!!! */

pub fn create_program() -> Result<Program, &'static str> {
    let vert_shader = Shader::from_source(&CString::new(include_str!(".vert")).unwrap(), gl::VERTEX_SHADER);
    let frag_shader = Shader::from_source(&CString::new(include_str!(".frag")).unwrap(), gl::VERTEX_SHADER);
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {gl::DeleteShader(self.id)}
    }
}