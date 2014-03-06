//! Basic rendering engine

use gl;
use hgl;
use png;
use glfw;
use std::rc::Rc;
use std::mem::size_of;
use std::cell::RefCell;
use collections::HashMap;
use gl::types::{GLfloat, GLint};
use hgl::{Vao, Vbo, Program, Shader, Ebo};

pub struct Engine {
    sprites: ~[Rc<RefCell<Sprite>>],
    textures: HashMap<&'static str, Rc<Tex>>,
    /// Width of the render surface (used to normalize sprite coordinates)
    width: GLint,
    /// Height of the render surface (used to normalize sprite coordinates)
    height: GLint,
    vao: hgl::Vao,
    vbo: hgl::Vbo,
    ebo: hgl::Ebo,
}

impl Engine {
    pub fn new(width: GLint, height: GLint) -> Engine {
        gl::load_with(glfw::get_proc_address);
        gl::Viewport(0, 0, width, height);
        let vao = Vao::new();
        vao.bind();
        let program = Program::link(&[Shader::from_file("vertex.glsl", hgl::program::VertexShader).unwrap().unwrap(),
                                     Shader::from_file("fragment.glsl", hgl::program::FragmentShader).unwrap().unwrap()
                                    ]).unwrap();
        program.bind_frag(0, "out_color");
        program.bind();

        gl::Uniform2f(program.uniform("windowsize"), width as GLfloat, height as GLfloat);

        vao.enable_attrib(&program, "position", 2, 4*size_of::<i32>() as i32, 0);
        vao.enable_attrib(&program, "texcoord", 2, 4*size_of::<i32>() as i32, 2 * size_of::<i32>());

        Engine { sprites: ~[], textures: HashMap::new(), width: width, height: height, vao: vao, vbo: Vbo::new(), ebo: Ebo::new() }
    }

    pub fn load_texture(&mut self, name: &'static str, path: &str) {
        let tex = Rc::new(Tex::from_png(path));
        self.textures.insert(name, tex);
    }

    pub fn add_sprite(&mut self, s: Sprite) -> Rc<RefCell<Sprite>> {
        let s = Rc::new(RefCell::new(s));
        self.sprites.push(s.clone());
        s
    }

    pub fn render(&self) {
        // this is really awful, and probably going to be slow?
        let mut data = ~[];
        let mut indices = ~[];
        for sprite in self.sprites.iter() {
            let sprite = (*sprite).borrow().borrow().get();
            let base = data.len();
            // points of the rectangle that makes up this sprite, ccw
            let sdata = &[
                 sprite.x, sprite.y, 0, 0,
                 sprite.x + sprite.width, sprite.y, 1, 0,
                 sprite.x + sprite.width, sprite.y + sprite.height, 1, 1,
                 sprite.x, sprite.y + sprite.height, 0, 1
            ];
            data.extend(&mut sdata.iter().map(|&x| x));
            let new_indices = &[base, base+1, base+2,
                               base+2, base+3, base];
            indices.extend(&mut new_indices.iter().map(|&x| x));
        }

        self.vbo.load_data(data, hgl::buffer::DynamicDraw);
        self.ebo.load_data(indices, hgl::buffer::DynamicDraw);

        for (idx, sprite) in self.sprites.iter().enumerate() {
            let sprite = (*sprite).borrow().borrow().get();
            let tex = sprite.texture.borrow();
            tex.texture.activate(0);
            self.vao.draw_elements(hgl::Triangles, ((idx * 6) - 1) as GLint, 6);
        }
    }
}

/// A sprite; a textured rectangle. The origin (x, y) is the bottom left. The
/// top right is (x + width, y + height).
pub struct Sprite {
    x: int,
    y: int,
    height: int,
    width: int,
    texture: Rc<Tex>
}

impl Sprite {
    pub fn new(x: int, y: int, height: int, width: int, texture: Rc<Tex>) -> Sprite {
        Sprite { x: x, y: y, height: height, width: width, texture: texture }
    }
}

pub struct Tex {
    texture: hgl::Texture
}

impl Tex {
    pub fn from_png(p: &str) -> Tex {
        let path = Path::new(p);
        let img = match png::load_png(&path) {
            Ok(i) => i,
            Err(s) => fail!("Could not load png: {}", s)
        };

        let fmt = match img.color_type {
            png::RGB8 => hgl::texture::pixel::RGB,
            png::RGBA8 => hgl::texture::pixel::RGBA,
            t => fail!("unsupported color type {:?} in png", t),
        };

        let ii = hgl::texture::ImageInfo::new()
            .pixel_format(fmt).pixel_type(hgl::texture::pixel::BYTE)
            .width(img.width as GLint).height(img.height as GLint);

        let gltex = hgl::Texture::new(hgl::texture::Texture2D, ii, img.pixels.as_slice().as_ptr());

        Tex { texture: gltex }
    }
}
