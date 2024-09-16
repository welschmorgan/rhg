use std::{cell::RefCell, rc::Rc};

use glow::HasContext as _;

use rhg_core::{here, ContextRef, Error, ErrorKind, Shader, ShaderKind};

use crate::GLContext;

pub struct GLShader {
  kind: ShaderKind,
  source: String,
}

impl GLShader {
  pub fn new(kind: ShaderKind, source: String) -> Self {
    Self { kind, source }
  }

  pub fn create(&mut self, ctx: &ContextRef) -> rhg_core::Result<()> {
    unsafe {
      let gl = Rc::downcast::<GLContext>(ctx.clone()).map_err(|e| Error::new(ErrorKind::Rendering, format!("invalid GLContext"), None, here!()))?;
      let program = gl.create_program().expect("Cannot create program");

      let (vertex_shader_source, fragment_shader_source) = (
        r#"#version 100
      attribute vec2 position;
      varying vec2 frag_position;
      void main() {
          frag_position = position;
          gl_Position = vec4(position, 0.0, 1.0);
      }"#,
        r#"#version 100
      precision mediump float;
      varying vec2 frag_position;
      uniform float effect_time;
      uniform float rotation_time;

      float roundRectDistance(vec2 pos, vec2 rect_size, float radius)
      {
          vec2 q = abs(pos) - rect_size + radius;
          return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - radius;
      }

      void main() {
          vec2 size = vec2(0.4, 0.5) + 0.2 * cos(effect_time / 500. + vec2(0.3, 0.2));
          float radius = 0.5 * sin(effect_time / 300.);
          float a = rotation_time / 800.0;
          float d = roundRectDistance(mat2(cos(a), -sin(a), sin(a), cos(a)) * frag_position, size, radius);
          vec3 col = (d > 0.0) ? vec3(sin(d * 0.2), 0.4 * cos(effect_time / 1000.0 + d * 0.8), sin(d * 1.2)) : vec3(0.2 * cos(d * 0.1), 0.17 * sin(d * 0.4), 0.96 * abs(sin(effect_time / 500. - d * 0.9)));
          col *= 0.8 + 0.5 * sin(50.0 * d);
          col = mix(col, vec3(0.9), 1.0 - smoothstep(0.0, 0.03, abs(d) ));
          gl_FragColor = vec4(col, 1.0);
      }"#,
      );

      let shader_sources = [
        (glow::VERTEX_SHADER, vertex_shader_source),
        (glow::FRAGMENT_SHADER, fragment_shader_source),
      ];

      let mut shaders = Vec::with_capacity(shader_sources.len());

      for (shader_type, shader_source) in shader_sources.iter() {
        let shader = gl
          .create_shader(*shader_type)
          .expect("Cannot create shader");
        gl.shader_source(shader, shader_source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
          panic!("{}", gl.get_shader_info_log(shader));
        }
        gl.attach_shader(program, shader);
        shaders.push(shader);
      }

      gl.link_program(program);
      if !gl.get_program_link_status(program) {
        panic!("{}", gl.get_program_info_log(program));
      }

      for shader in shaders {
        gl.detach_shader(program, shader);
        gl.delete_shader(shader);
      }
    }
    Ok(())
  }
}

impl Shader for GLShader {
  fn kind(&self) -> ShaderKind {
    todo!()
  }

  fn source(&self) -> String {
    todo!()
  }

  fn create(&mut self, ctx: &ContextRef) -> rhg_core::Result<()> {
    todo!()
  }

  fn destroy(&mut self, ctx: &ContextRef) -> rhg_core::Result<()> {
    todo!()
  }
}
