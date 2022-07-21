use std::ffi::{CStr, CString};
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowEvent, WindowHint};
use glad_gl::gl;
use glad_gl::gl::GLuint;

extern "system" fn debug_callback(
	source: gl::GLenum, type_: gl::GLenum, id: gl::GLuint,
	severity: gl::GLenum, length: gl::GLsizei,
	message: *const gl::GLchar, param: *mut std::ffi::c_void) {
	let message = unsafe { CStr::from_ptr(message) };
	eprintln!("GL log: {}", message.to_str().unwrap());
}

fn main() {
	// Initialize GLFW
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	// create a window with OpenGL 4.5 core context
	glfw.window_hint(WindowHint::ContextVersion(4, 5));
	glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
	let (mut window, events) = glfw.create_window(
		800,
		600,
		"Game",
		glfw::WindowMode::Windowed
	).expect("Failed to create window");

	// Make the window's OpenGL context current (it needs to be current before calling
	// any OpenGL functions.
	window.make_current();
	// Make GLFW automatically poll for keys without needing to have a manual callback.
	window.set_key_polling(true);

	// Use GLAD and GLFW to load OpenGL functions before we use them.
	gl::load(|name| glfw.get_proc_address_raw(name));

	unsafe {
		// fn(source: GLenum, type_: GLenum, id: GLuint,
		// severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut raw::c_void)
		gl::DebugMessageCallback(debug_callback, std::ptr::null());
		gl::Enable(gl::DEBUG_OUTPUT);
		gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
	}

	// Set background clear color to black
	unsafe {
		gl::ClearColor(0f32, 0f32, 0f32, 0f32);
	}

	// Vertex shader written in GLSL version 450 core.
	// The vertex shader runs on every vertex of the data we are going to draw.
	let vertex_shader_source = r#"#version 450 core
	// At vertex attribute 0 we are going to have 3 floats to represent the position.
    layout (location = 0) in vec3 pos;

    // Needed for some OpenGL drivers.
    out gl_PerVertex {
        vec4 gl_Position;
    };

    void main() {
        // We assign the vertex position to GLSL internal vertex position variable.
        // (when its first converted into a vec4 with the last member being 1)
        gl_Position = vec4(pos, 1);
    }
    "#;

	// The fragment shader is going to be run on every pixel of the rasterized mesh.
	let fragment_shader_source = r#"#version 450 core
		// The final color we are going to be outputting is going to be at attribute zero.
		layout (location = 0) out vec4 color;

		void main() {
			// Set the color of the pixel red.
			color = vec4(1, 0, 0, 1);
		}
	"#;

	// A shader program is a combination of shaders "linked" together.
	let shader_program = unsafe { gl::CreateProgram() };
	{
		// Create the shaders.
		let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
		let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

		// Add the shader sources to them.
		unsafe {
			// Create C strings of the sources.
			let vertex_shader_source_c = CString::new(vertex_shader_source).unwrap();
			let fragment_shader_source_c = CString::new(fragment_shader_source).unwrap();
			// we can pass null to length to make OpenGL calculate it automatically.
			gl::ShaderSource(
				vertex_shader,
				1,
				&vertex_shader_source_c.as_ptr(),
				std::ptr::null());
			gl::ShaderSource(
				fragment_shader,
				1,
				&fragment_shader_source_c.as_ptr(),
				std::ptr::null()
			);
		}

		// Compile the shaders.
		unsafe {
			gl::CompileShader(vertex_shader);
			let mut status: gl::GLint = 0;
			gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut status);
			// If the compilation was unsuccessful get the error log and print it.
			if status != 1 {
				let mut length: gl::GLint = 0;
				gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut length);
				let mut buffer = vec![0 as gl::GLchar; length as usize];
				gl::GetShaderInfoLog(vertex_shader, length, &mut length, buffer.as_mut_ptr());
				let string = CStr::from_ptr(buffer.as_ptr());
				eprintln!("gl vertex shader: {}", string.to_str().unwrap());
			}

			gl::CompileShader(fragment_shader);
			status = 0;
			gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut status);
			// If the compilation was unsuccessful get the error log and print it.
			if status != 1 {
				let mut length: gl::GLint = 0;
				gl::GetShaderiv(fragment_shader, gl::INFO_LOG_LENGTH, &mut length);
				let mut buffer = vec![0 as gl::GLchar; length as usize];
				gl::GetShaderInfoLog(fragment_shader, length, &mut length, buffer.as_mut_ptr());
				let string = CStr::from_ptr(buffer.as_ptr());
				eprintln!("gl vertex shader: {}", string.to_str().unwrap());
			}
		}

		// Attach the shaders to the program, link it and delete the shaders after that.
		unsafe {
			gl::AttachShader(shader_program, vertex_shader);
			gl::AttachShader(shader_program, fragment_shader);
			gl::LinkProgram(shader_program);

			let mut status: gl::GLint = 0;
			gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut status);
			if status != 1 {
				let mut length: gl::GLint = 0;
				gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut length);
				let mut buffer = vec![0 as gl::GLchar; length as usize];
				gl::GetProgramInfoLog(shader_program, length, &mut length, buffer.as_mut_ptr());
				let string = CStr::from_ptr(buffer.as_ptr());
				eprintln!("gl shader link: {}", string.to_str().unwrap());
			}

			gl::ValidateProgram(shader_program);
			gl::GetProgramiv(shader_program, gl::VALIDATE_STATUS, &mut status);
			if status != 1 {
				let mut length: gl::GLint = 0;
				gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut length);
				let mut buffer = vec![0 as gl::GLchar; length as usize];
				gl::GetProgramInfoLog(shader_program, length, &mut length, buffer.as_mut_ptr());
				let string = CStr::from_ptr(buffer.as_ptr());
				eprintln!("gl shader validate: {}", string.to_str().unwrap());
			}

			gl::DeleteShader(vertex_shader);
			gl::DeleteShader(fragment_shader);
		}
	}

	// Make a vertex buffer and an index buffer (element buffer in OpenGL)
	let mut vertex_buffer: GLuint = 0;
	let mut index_buffer: GLuint = 0;
	unsafe {
		gl::CreateBuffers(1,  &mut vertex_buffer);
		gl::CreateBuffers(1, &mut index_buffer);
	}

	let vertices = [
		-0.5 as f32, -0.5, 0.0,
		0.0, 0.5, 0.0,
		0.5, -0.5, 0.0
	];

	// Vertices 0, 1, 2.
	let indices = [
		0 as u32, 1, 2
	];

	// Put the data to the buffers. This differs from https://learnopengl.org because this uses
	// NamedBufferData from the GL_ARB_direct_state_access extension
	// which avoids the need to bind the buffer.
	unsafe {
		gl::NamedBufferData(
			vertex_buffer,
			(vertices.len() * std::mem::size_of::<f32>()) as gl::GLsizeiptr,
			vertices.as_ptr() as *const _, gl::STATIC_DRAW);

		gl::NamedBufferData(
			index_buffer,
			(indices.len() * std::mem::size_of::<u32>()) as gl::GLsizeiptr,
			indices.as_ptr() as *const _, gl::STATIC_DRAW);
	}

	// Create a vertex array object, which holds the information on how to interpret the vertices in
	// the vertex buffer along with the index buffer. This is also different from https://learnopengl.org.
	let mut vao: gl::GLuint = 0;
	unsafe {
		gl::CreateVertexArrays(1, &mut vao);

		// Set the vertex buffer and specify how much space each vertex takes.
		gl::VertexArrayVertexBuffer(
			vao,
			0,
			vertex_buffer,
			0,
			(3 * std::mem::size_of::<f32>()) as gl::GLsizei);

		// Set the index buffer.
		gl::VertexArrayElementBuffer(vao, index_buffer);

		// The first attribute is going to be the position (3 floats) with offset 0
		// because its the first attribute.
		gl::VertexArrayAttribFormat(
			vao,
			0,
			3,
			gl::FLOAT,
			gl::FALSE,
			0);
		// Enable the first attribute.
		gl::EnableVertexArrayAttrib(vao, 0);
		// Needed for some drivers. Binding index can always be zero for simple use-cases.
		gl::VertexArrayAttribBinding(vao, 0, 0);
	}

	while !window.should_close() {
		// Poll for events and process them.
		glfw.poll_events();
		for (_, event) in glfw::flush_messages(&events) {
			match event {
				WindowEvent::Key(Key::Escape, _scancode, Action::Press, _modifiers) => {
					window.set_should_close(true);
				},
				_ => {}
			}
		}

		// Clear the window color buffer with the previously defined background color.
		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		// Draw a triangle
		unsafe {
			// Use the shader program and bind the vertex array.
			gl::UseProgram(shader_program);
			gl::BindVertexArray(vao);

			// Draw the indices. The last parameter needs to be null in case we have an index buffer
			// in the vertex array object.
			gl::DrawElements(
				gl::TRIANGLES,
				indices.len() as gl::GLsizei,
				gl::UNSIGNED_INT,
				std::ptr::null());
		}

		// swap the front and back buffers so what we drew previously appears on the screen.
		window.swap_buffers();
	}
}
