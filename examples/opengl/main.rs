use kiss3d::{
    camera::{ArcBall, Camera},
    context::Context,
    light::Light,
    nalgebra as na,
    resource::{Effect, Material, Mesh, ShaderAttribute, ShaderUniform},
    scene::ObjectData,
    window::Window,
};
use na::{Isometry3, Matrix3, Matrix4, Point3, Vector3};
use polyhedron_ops::*;
use std::{cell::RefCell, env, error::Error, io, io::Write, rc::Rc};

use itertools::Itertools;

fn into_mesh(polyhedron: Polyhedron) -> Mesh {
    let (face_index, points, normals) = polyhedron.to_triangle_mesh_buffers();

    Mesh::new(
        // Duplicate points per face so we can
        // match the normals per face.
        points
            .iter()
            .map(|p| na::Point3::<f32>::new(p.x, p.y, p.z))
            .collect::<Vec<_>>(),
        face_index
            .iter()
            .tuples::<(_, _, _)>()
            .map(|i| na::Point3::<u16>::new(*i.0 as _, *i.1 as _, *i.2 as _))
            .collect::<Vec<_>>(),
        Some(
            normals
                .iter()
                .map(|n| na::Vector3::new(n.x, n.y, n.z))
                .collect::<Vec<_>>(),
        ),
        None,
        false,
    )
}

//////////////////// glsl shader stuff
// A material that draws normals
pub struct NormalMaterial {
    shader: Effect,
    position: ShaderAttribute<Point3<f32>>,
    normal: ShaderAttribute<Vector3<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    transform: ShaderUniform<Matrix4<f32>>,
    scale: ShaderUniform<Matrix3<f32>>,
}

impl NormalMaterial {
    pub fn new() -> NormalMaterial {
        let mut shader = Effect::new_from_str(NORMAL_VERTEX_SRC, NORMAL_FRAGMENT_SRC);

        //////////////

        //        shader.use_program();
        let ctxt = Context::get();
        shader.use_program();

        // this stuff enables alpha (transparency)
        let _ = ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL);
        ctxt.enable(Context::BLEND);
        ctxt.blend_func_separate(
            Context::SRC_ALPHA,
            Context::ONE_MINUS_SRC_ALPHA,
            Context::ONE,
            Context::ONE_MINUS_SRC_ALPHA,
        );

        //        ctxt.disable(Context::DEPTH_TEST);
        ///////////////
        NormalMaterial {
            position: shader.get_attrib("position").unwrap(),
            normal: shader.get_attrib("normal").unwrap(),
            transform: shader.get_uniform("transform").unwrap(),
            scale: shader.get_uniform("scale").unwrap(),
            view: shader.get_uniform("view").unwrap(),
            proj: shader.get_uniform("proj").unwrap(),
            shader,
        }
    }
}

impl Material for NormalMaterial {
    fn render(
        &mut self,
        pass: usize,
        transform: &Isometry3<f32>,
        scale: &Vector3<f32>,
        camera: &mut dyn Camera,
        _: &Light,
        _: &ObjectData,
        mesh: &mut Mesh,
    ) {
        self.shader.use_program();
        self.position.enable();
        self.normal.enable();

        /*
         *
         * Setup camera and light.
         *
         */
        camera.upload(pass, &mut self.proj, &mut self.view);

        /*
         *
         * Setup object-related stuffs.
         *
         */
        let formated_transform = transform.to_homogeneous();
        let formated_scale = Matrix3::from_diagonal(&Vector3::new(scale.x, scale.y, scale.z));

        self.transform.upload(&formated_transform);
        self.scale.upload(&formated_scale);

        mesh.bind_coords(&mut self.position);
        mesh.bind_normals(&mut self.normal);
        mesh.bind_faces();

        Context::get().draw_elements(
            Context::TRIANGLES,
            mesh.num_pts() as i32,
            Context::UNSIGNED_SHORT,
            //VERTEX_INDEX_TYPE,
            0,
        );

        mesh.unbind();

        self.position.disable();
        self.normal.disable();
    }
}

static NORMAL_VERTEX_SRC: &str = include_str!("vert.glsl");
static NORMAL_FRAGMENT_SRC: &str = include_str!("frag.glsl");
//////////////////// end glsl shader stuff

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    //        let mut polyx = Polyhedron::icosidodecahedron();
    //    let mut poly =  Polyhedron::dodecahedron();
    let mut poly = Polyhedron::pentagonal_rotunda();
    //  let mut polyx = Polyhedron::rhombicosidodecahedron();
    //	let mut poly = Polyhedron::truncated_cuboctahedron();
    poly.normalize();
    //	poly.reverse();
    let distance = 2.0f32;
    let eye = Point3::new(distance, distance, distance);
    let at = Point3::origin();
    let mut arc_ball = ArcBall::new(eye, at);

    let mut window = Window::new("Polyhedron Operations");
    //        let ctxt = Context::get();
    // 	ctxt.enable(Context::BLEND);
    //	let mut s = window.add_sphere(1.0);
    //    s.set_material(material.clone());

    let mesh = Rc::new(RefCell::new(into_mesh(poly.clone())));
    let material = Rc::new(RefCell::new(
        Box::new(NormalMaterial::new()) as Box<dyn Material + 'static>
    ));

    let mut c = window.add_mesh(mesh.clone(), Vector3::new(1.0, 1.0, 1.0));
    c.set_material(material);
    c.enable_backface_culling(false);
    c.set_points_size(10.);

    window.set_light(Light::StickToCamera);
    window.set_framerate_limit(Some(60));

    let path = dirs::home_dir().unwrap();

    println!("{}", poly.name());
    //    io::stdout().flush().unwrap();

    while !window.should_close() {
        // rotate the arc-ball camera.
        let curr_yaw = arc_ball.yaw();
        arc_ball.set_yaw(curr_yaw + 0.001);

        // Polyhedron
        window.render_with_camera(&mut arc_ball);

        // x y z axes
        window.draw_line(
            &Point3::new(-1.618, 0.0, 0.0),
            &Point3::new(1.618, 0.0, 0.0),
            &Point3::new(1.618, 0.0, 0.0),
        );
        window.draw_line(
            &Point3::new(0.0, -1.618, 0.0),
            &Point3::new(0.0, 1.618, 0.0),
            &Point3::new(0.0, 1.618, 0.0),
        );
        window.draw_line(
            &Point3::new(0.0, 0.0, -1.618),
            &Point3::new(0.0, 0.0, 1.618),
            &Point3::new(0.0, 0.0, 1.618),
        );

        // polyhedron edges

        /*        window.set_line_width(1.618);

                for e in poly.to_edges() {
            let p1 = poly.positions()[e[0] as usize];
            let p2 = poly.positions()[e[1] as usize];
        let k = 1.0001220703125;
            let kp1 = Point3::new(k*p1.x, k*p1.y, k*p1.z);
            let kp2 = Point3::new(k*p2.x, k*p2.y, k*p2.z);
            let color = Point3::new(1.0, 1.0, 1.0);
            window.draw_line(&kp1, &kp2, &color);
            //        println!("{:?} {:?} {:?}",kp1, kp2,color );
        }*/
    }

    Ok(())
}
