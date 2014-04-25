//! Basic rendering engine

use gl;
use hgl;
use png;
use std;
use std::rc::Rc;
use std::mem::size_of;
use std::cell::RefCell;
use gl::types::{GLfloat, GLint, GLuint};
use collections::{HashMap, DList, Deque};
use hgl::{Vao, Vbo, Program, Shader, Ebo};

pub struct Engine {
    // FIXME: implement remove or modify_iter on DList, rather than having an
    // unbounded list of Option.
    pub sprites: DList<Option<Rc<RefCell<Sprite>>>>,
    pub textures: HashMap<&'static str, Rc<Tex>>,
    /// Width of the render surface (used to normalize sprite coordinates)
    pub width: GLfloat,
    /// Height of the render surface (used to normalize sprite coordinates)
    pub height: GLfloat,
    pub vao: hgl::Vao,
    pub vbo: hgl::Vbo,
    pub ebo: hgl::Ebo,
    pub program: hgl::Program,
    pub uni_rot: GLint,
    pub uni_sprite: GLint,
    pub uni_center: GLint,
}

impl Engine {
    pub fn new(width: GLint, height: GLint) -> Engine {
        gl::Viewport(0, 0, width, height);
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        let vao = Vao::new();
        vao.bind();

        let program = Program::link(&[Shader::from_file("assets/vertex.glsl", hgl::program::VertexShader).unwrap().unwrap(),
                                     Shader::from_file("assets/fragment.glsl", hgl::program::FragmentShader).unwrap().unwrap()
                                    ]).unwrap();
        let uni_rot = program.uniform("rotation");
        let uni_sprite = program.uniform("sprite");
        let uni_center = program.uniform("center");
        program.bind_frag(0, "out_color");
        program.bind();

        gl::Uniform1i(uni_sprite, 0);
        gl::Uniform2f(program.uniform("windowsize"), width as GLfloat, height as GLfloat);

        let vbo = Vbo::new();
        vbo.bind();
        let ebo = Ebo::new();
        ebo.bind();

        vao.enable_attrib(&program, "position", gl::FLOAT, 2, 4*size_of::<GLfloat>() as i32, 0);
        vao.enable_attrib(&program, "texcoord", gl::FLOAT, 2, 4*size_of::<GLfloat>() as i32, 2 * size_of::<i32>());

        Engine {
            sprites: DList::new(),
            textures: HashMap::new(),
            width: width as GLfloat,
            height: height as GLfloat,
            program: program,
            vao: vao,
            vbo: vbo,
            ebo: ebo,
            uni_rot: uni_rot,
            uni_sprite: uni_sprite,
            uni_center: uni_center,
        }
    }

    pub fn load_texture(&mut self, name: &'static str, path: &str) -> Rc<Tex> {
        let tex = Rc::new(Tex::from_png(path));
        self.textures.insert(name, tex.clone());
        tex
    }

    pub fn add_sprite(&mut self, s: Sprite) -> Rc<RefCell<Sprite>> {
        let s = Rc::new(RefCell::new(s));
        self.sprites.push_back(Some(s.clone()));
        s
    }

    #[allow(dead_code)]
    pub fn remove_sprite(&mut self, spr: Rc<RefCell<Sprite>>) {
        let mut rem = false;
        for sprite in self.sprites.mut_iter() {
            match sprite {
                &Some(ref s) => {
                    if (s.deref() as *RefCell<Sprite> as int) == (spr.deref() as *RefCell<Sprite> as int) {
                        rem = true;
                    }
                },
                &None => ()
            }
            if rem {
                sprite.take_unwrap();
                break;
            }
        }
    }

    pub fn render(&self) {
        // this is really awful, and probably going to be slow?
        // one way to do it differently is to take an Iterator<Rc<Sprite>>,
        // and create the vbo/ebo from that. Then, the Game would chain
        // together its list of sprites (by enemy type, projectile type, etc),
        // to avoid swapping out different textures all the time.
        //
        // probably not going to matter though.
        let mut data = Vec::new();
        let mut indices = Vec::new();
        let mut base = 0 as GLuint;

        gl::Clear(gl::COLOR_BUFFER_BIT);

        for sprite in self.sprites.iter().filter_map(|x| x.as_ref()) {
            let &Sprite { x, y, height, width, .. } = &*sprite.borrow();
            let height = 2.0 * height;
            let width = 2.0 * width;
            // points of the rectangle that makes up this sprite, ccw
            let sdata: &[GLfloat] = &[
                 x, y, 0., 1.,
                 x + width, y, 1., 1.,
                 x + width, y + height, 1., 0.,
                 x, y + height, 0., 0.
            ];
            data.extend(sdata.iter().map(|&x| x));

            let new_indices = &[base, base+1, base+2,
                                base+2, base+3, base];
            indices.extend(new_indices.iter().map(|&x| x));
            base += 4;
        }

        debug!("VBO: {}", data);
        debug!("EBO: {}", indices);

        debug_assert!({ let max = data.len(); indices.iter().all(|&x| (x as uint) < max)});

        self.vbo.load_data(data.as_slice(), hgl::buffer::DynamicDraw);
        self.ebo.load_data(indices.as_slice(), hgl::buffer::DynamicDraw);

        let mut first = true;
        for (idx, sprite) in self.sprites.iter().filter_map(|x| x.as_ref()).enumerate() {
            let sprite = sprite.borrow();
            let height = 2.0 * sprite.height;
            let width = 2.0 * sprite.width;
            debug!("Going to be printing sprite {}", *sprite);
            let tex = &sprite.texture;

            tex.texture.activate(0);
            gl::Uniform1f(self.uni_rot, sprite.rot);
            gl::Uniform2f(self.uni_center,
                          (2.0 * sprite.x + width ) / 2.0,
                          (2.0 * sprite.y + height) / 2.0);

            // eugh
            let start = if first { first = false; 0 } else { (idx * 6) };

            debug!("Drawing {} indices starting at {}", 6, start);
            self.vao.draw_elements(hgl::Triangles, start as GLint * 4, 6);
        }
    }
}

/// A sprite; a textured rectangle. The origin (x, y) is the bottom left. The
/// top right is (x + width, y + height).
pub struct Sprite {
    pub x: GLfloat,
    pub y: GLfloat,
    pub rot: GLfloat,
    pub height: GLfloat,
    pub width: GLfloat,
    pub texture: Rc<Tex>
}

impl std::fmt::Show for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f.buf, "Sprite \\{ x: {}, y: {}, rot: {}, height: {}, width: {}, texture: {} \\}",
               self.x, self.y, self.rot, self.height, self.width, "NOPE")
    }
}

impl Sprite {
    pub fn new(x: GLfloat, y: GLfloat, height: GLfloat, width: GLfloat, rot: GLfloat, texture: Rc<Tex>) -> Sprite {
        Sprite { x: x, y: y, height: height, width: width, rot: rot, texture: texture }
    }
}

pub struct Tex {
    texture: hgl::Texture
}

impl Tex {
    pub fn from_png(p: &str) -> Tex {
        let path = Path::new(format!("assets/{}", p));
        let img = match png::load_png(&path) {
            Ok(i) => i,
            Err(s) => fail!("Could not load png {}: {}", p, s)
        };

        let fmt = match img.color_type {
            png::RGBA8 => { info!("loaded rgba8 png file {}", p); hgl::texture::pixel::RGBA },
            t => fail!("unsupported color type {:?} in png", t),
        };

        let ii = hgl::texture::ImageInfo::new()
            .pixel_format(fmt).pixel_type(hgl::texture::pixel::UNSIGNED_BYTE)
            .width(img.width as GLint).height(img.height as GLint);

        let gltex = hgl::Texture::new(hgl::texture::Texture2D, ii, img.pixels.as_slice().as_ptr());
        gltex.gen_mipmaps();
        gltex.filter(hgl::texture::NearestMipmapNearest);
        gltex.wrap(hgl::texture::Repeat);

        Tex { texture: gltex }
    }
}
