extern crate rhg_engine;

// use slint::SharedString;

use std::{
  any::Any,
  cell::{Ref, RefCell},
  fmt::{Debug, Display},
  ops::{Deref, DerefMut},
  rc::Rc,
  time::Instant,
};

use glow::{Buffer, HasContext as _, VertexArray};
use rhg_engine::{
  err, here, Error, ErrorKind, GLRenderer, Renderable, Vec3f32, Vec4f32, Vertex, VertexBuffer,
};
use slint::{ComponentHandle, GraphicsAPI, RenderingState, SharedString, Weak};

slint::include_modules!();

pub type ProgramPtr = Rc<RefCell<glow::Program>>;

pub struct Ptr<T: ?Sized>(Rc<RefCell<T>>);

impl<T> Ptr<T> {
  pub fn new(val: T) -> Self {
    Self(Rc::new(RefCell::new(val)))
  }
}

impl<T> Clone for Ptr<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T: Default> Default for Ptr<T> {
  fn default() -> Self {
    Self::new(T::default())
  }
}

impl<T: Debug> Debug for Ptr<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("Ptr").field(&self.0).finish()
  }
}

impl<T: Display> Display for Ptr<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.borrow().fmt(f)
  }
}

impl<T> Deref for Ptr<T> {
  type Target = Rc<RefCell<T>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for Ptr<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

pub struct App(Rc<RefCell<AppInner>>);

pub struct AppInner {
  launcher_window: LauncherWindow,
  game_window: GameWindow,
  renderer: Option<Ptr<GLRenderer>>,
}

impl App {
  pub fn new() -> Self {
    let ret = Self(Rc::new(RefCell::new(AppInner {
      launcher_window: LauncherWindow::new().unwrap(),
      game_window: GameWindow::new().unwrap(),
      renderer: None,
    })));
    let inner = ret.0.clone();
    let launcher_weak = ret.0.borrow().launcher_window.as_weak();
    ret.0.borrow().launcher_window.set_model(LauncherModel {
      engine_version: SharedString::from(rhg_engine::VERSION),
    });
    ret.0.borrow().launcher_window.on_launchGame(move || {
      println!("Launching game ...");
      setup_rendering(inner.clone());
      inner.borrow().game_window.show().unwrap();
      launcher_weak.unwrap().window().hide().unwrap();
    });
    ret
  }

  pub fn renderer(&self) -> Ptr<GLRenderer> {
    self.0.borrow().renderer.as_ref().unwrap().clone()
  }

  fn run(self) {
    self.0.borrow().launcher_window.show().unwrap();
    slint::run_event_loop().unwrap();
  }
}

fn setup_scene(inner: Rc<RefCell<AppInner>>) {
  let buf = inner
    .borrow()
    .renderer
    .as_ref()
    .unwrap()
    .clone()
    .borrow_mut()
    .add_renderable(
      VertexBuffer::<f32>::named("cube_001").with_vertices([Vertex::new(
        Vec3f32::new(0f32, 0f32, 0f32),
        Vec4f32::new(255f32, 0f32, 0f32, 255f32),
        Vec3f32::new(0f32, 0f32, 0f32),
      )]),
    )
    .unwrap();
}

fn setup_rendering(inner: Rc<RefCell<AppInner>>) {
  let inner2 = inner.clone();
  let inner3 = inner.clone();
  if let Err(e) =
    inner
      .borrow()
      .game_window
      .window()
      .set_rendering_notifier(move |state, gfx_api| match state {
        slint::RenderingState::RenderingSetup => {
          let context = match gfx_api {
            #[cfg(not(target_arch = "wasm32"))]
            slint::GraphicsAPI::NativeOpenGL { get_proc_address } => unsafe {
              glow::Context::from_loader_function_cstr(|s| get_proc_address(s))
            },
            #[cfg(target_arch = "wasm32")]
            slint::GraphicsAPI::WebGL {
              canvas_element_id,
              context_type,
            } => {
              use wasm_bindgen::JsCast;

              let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id(canvas_element_id)
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();

              let webgl1_context = canvas
                .get_context(context_type)
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGlRenderingContext>()
                .unwrap();

              glow::Context::from_webgl1_context(webgl1_context)
            }
            _ => return,
          };
          println!("Renderer initialized: {:?}!", context.version());
          let renderer = Some(Ptr::new(GLRenderer::new(Rc::new(RefCell::new(context)))));
          inner3.borrow_mut().renderer = renderer;
        }
        slint::RenderingState::BeforeRendering => {
          inner2
            .borrow_mut()
            .renderer
            .as_mut()
            .unwrap()
            .borrow_mut()
            .render_before()
            .unwrap();
          inner2.borrow().game_window.window().request_redraw();
        }
        slint::RenderingState::AfterRendering => {
          inner2
            .borrow_mut()
            .renderer
            .as_mut()
            .unwrap()
            .borrow_mut()
            .render_after()
            .unwrap();
        }
        slint::RenderingState::RenderingTeardown => drop(inner3.borrow_mut().renderer.take()),
        _ => {}
      })
  {
    match e {
        slint::SetRenderingNotifierError::Unsupported => eprintln!("This example requires the use of the GL backend. Please run with the environment variable SLINT_BACKEND=GL set."),
        _ => unreachable!()
    }
    std::process::exit(1);
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(all(debug_assertions, target_arch = "wasm32"))]
  console_error_panic_hook::set_once();

  App::new().run()
}

/*
// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

use glow::HasContext;

struct EGLUnderlay {
    gl: glow::Context,
    program: glow::Program,
    effect_time_location: glow::UniformLocation,
    rotation_time_location: glow::UniformLocation,
    vbo: glow::Buffer,
    vao: glow::VertexArray,
    start_time: web_time::Instant,
}

impl EGLUnderlay {
    fn new(gl: glow::Context) -> Self {
        unsafe {
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
                let shader = gl.create_shader(*shader_type).expect("Cannot create shader");
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

            let effect_time_location = gl.get_uniform_location(program, "effect_time").unwrap();
            let rotation_time_location = gl.get_uniform_location(program, "rotation_time").unwrap();
            let position_location = gl.get_attrib_location(program, "position").unwrap();

            let vbo = gl.create_buffer().expect("Cannot create buffer");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let vertices = [-1.0f32, 1.0f32, -1.0f32, -1.0f32, 1.0f32, 1.0f32, 1.0f32, -1.0f32];

            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices.align_to().1, glow::STATIC_DRAW);

            let vao = gl.create_vertex_array().expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vao));
            gl.enable_vertex_attrib_array(position_location);
            gl.vertex_attrib_pointer_f32(position_location, 2, glow::FLOAT, false, 8, 0);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);

            Self {
                gl,
                program,
                effect_time_location,
                rotation_time_location,
                vbo,
                vao,
                start_time: web_time::Instant::now(),
            }
        }
    }
}

impl Drop for EGLUnderlay {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
            self.gl.delete_vertex_array(self.vao);
            self.gl.delete_buffer(self.vbo);
        }
    }
}

impl EGLUnderlay {
    fn render(&mut self, rotation_enabled: bool) {
        unsafe {
            let gl = &self.gl;

            gl.use_program(Some(self.program));

            // Retrieving the buffer with glow only works with native builds right now. For WASM this requires https://github.com/grovesNL/glow/pull/190
            // That means we can't properly restore the vao/vbo, but this is okay for now as this only works with femtovg, which doesn't rely on
            // these bindings to persist across frames.
            #[cfg(not(target_arch = "wasm32"))]
            let old_buffer =
                std::num::NonZeroU32::new(gl.get_parameter_i32(glow::ARRAY_BUFFER_BINDING) as u32)
                    .map(glow::NativeBuffer);
            #[cfg(target_arch = "wasm32")]
            let old_buffer = None;

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));

            #[cfg(not(target_arch = "wasm32"))]
            let old_vao =
                std::num::NonZeroU32::new(gl.get_parameter_i32(glow::VERTEX_ARRAY_BINDING) as u32)
                    .map(glow::NativeVertexArray);
            #[cfg(target_arch = "wasm32")]
            let old_vao = None;

            gl.bind_vertex_array(Some(self.vao));

            let elapsed = self.start_time.elapsed().as_millis() as f32;
            gl.uniform_1_f32(Some(&self.effect_time_location), elapsed);
            gl.uniform_1_f32(
                Some(&self.rotation_time_location),
                if rotation_enabled { elapsed } else { 0.0 },
            );

            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);

            gl.bind_buffer(glow::ARRAY_BUFFER, old_buffer);
            gl.bind_vertex_array(old_vao);
            gl.use_program(None);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let editor = Editor::new().unwrap();

    let mut underlay = None;

    let editor_weak = editor.as_weak();

    if let Err(error) = editor.window().set_rendering_notifier(move |state, graphics_api| {
        // eprintln!("rendering state {:#?}", state);

        match state {
            slint::RenderingState::RenderingSetup => {
                let context = match graphics_api {
                    #[cfg(not(target_arch = "wasm32"))]
                    slint::GraphicsAPI::NativeOpenGL { get_proc_address } => unsafe {
                        glow::Context::from_loader_function_cstr(|s| get_proc_address(s))
                    },
                    #[cfg(target_arch = "wasm32")]
                    slint::GraphicsAPI::WebGL { canvas_element_id, context_type } => {
                        use wasm_bindgen::JsCast;

                        let canvas = web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .get_element_by_id(canvas_element_id)
                            .unwrap()
                            .dyn_into::<web_sys::HtmlCanvasElement>()
                            .unwrap();

                        let webgl1_context = canvas
                            .get_context(context_type)
                            .unwrap()
                            .unwrap()
                            .dyn_into::<web_sys::WebGlRenderingContext>()
                            .unwrap();

                        glow::Context::from_webgl1_context(webgl1_context)
                    }
                    _ => return,
                };
                underlay = Some(EGLUnderlay::new(context))
            }
            slint::RenderingState::BeforeRendering => {
                if let (Some(underlay), Some(editor)) = (underlay.as_mut(), editor_weak.upgrade()) {
                    underlay.render(editor.get_rotation_enabled());
                    editor.window().request_redraw();
                }
            }
            slint::RenderingState::AfterRendering => {}
            slint::RenderingState::RenderingTeardown => {
                drop(underlay.take());
            }
            _ => {}
        }
    }) {
        match error {
            slint::SetRenderingNotifierError::Unsupported => eprintln!("This example requires the use of the GL backend. Please run with the environment variable SLINT_BACKEND=GL set."),
            _ => unreachable!()
        }
        std::process::exit(1);
    }

    editor.run().unwrap();
}
 */
