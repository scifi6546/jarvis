mod backend;
use jarvis::prelude::*;
use wasm_bindgen::JsCast;

const WORLD_VERT_SHADER: &'static str = r#"#version 300 es
        in vec3 position;
        in vec2 uv;
        in vec3 normal;
        out vec2 o_uv;
        out vec3 o_normal;
        uniform mat4 camera;
        uniform mat4 model;
        void main() {
            gl_Position = camera*model*vec4(position,1.0);
            o_normal = normal;
            o_uv = uv;
        }
    "#;
const WORLD_FRAG_SHADER: &'static str = r#"#version 300 es
        precision highp float;
        out vec4 color;
        in vec2 o_uv;
        in vec3 o_normal;
        uniform vec3 sun_direction;
        uniform vec4 sun_color;
        uniform sampler2D u_texture;
        vec4 onify(vec4 v){
            return v*vec4(0.0,0.0,0.0,0.0)+vec4(1.0,1.0,1.0,1.0);
        }
        float sun(){
            return dot(-1.0*sun_direction,o_normal);
        }
        vec4 sun_vec(){
            float s = sun();
            return vec4(s,s,s,1.0)*onify(sun_color);
        }
        void main() {
            color = sun_vec()*sun_color*texture(u_texture,o_uv);
        }
    "#;
pub struct WebGL {
    gl_context: web_sys::WebGl2RenderingContext,
}
impl WebGL {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas: web_sys::HtmlCanvasElement = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into()
            .expect("failed to create canvas");
        let gl_context: web_sys::WebGl2RenderingContext = canvas
            .get_context("webgl2")
            .expect("failed to get webgl context")
            .unwrap()
            .dyn_into()
            .expect("failed to convert to webgl2");

        Self { gl_context }
    }
}
impl Plugin for WebGL {
    fn setup(&mut self, _world: &mut GameState) {
        info!("created game");
    }
    fn on_frame(&mut self, _world: &mut GameState) {
        info!("rendering");
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
