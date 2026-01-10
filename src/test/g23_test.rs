#![allow(dead_code)]
// 窗口管理
use winit::event_loop::EventLoop;

// 数学库

use super::super::math_forest::geometry::d3::linear::vec3::Vec3;


// 平面
use super::super::graph::d2::colors;
use super::super::graph::d2::common::GeoObj;
use super::super::graph::d2::main::D2Plotter;
// 三维
use super::super::graph::d3::{D3Plotter, GeoObjD3, MeshData, ParametricCurveSolver};
use super::super::graph::d3::implicit_surface::ImplicitSurfaceSolver;

pub fn main_d2() {

    let event_loop = EventLoop::new().unwrap();
    let mut d2_plotter = D2Plotter::new();

    /*
    d2_plotter.add_object(GeoObj::new_implicit(
        |x, y|
            // x.tan().powf(y.tan()),
            x/x.sin() + y/y.sin() - x*y/(x*y).sin(),
        colors::RED,
        4.0)
    );

     */

    d2_plotter.add_object(GeoObj::new_implicit(
        |x, y|
             x.tan().powf(y.tan()),
            // 1000.0*x.sin()-y,
        colors::RED,
        4.0)
    );

    /*
    d2_plotter.add_object(GeoObj::new_implicit(
        |x, y| x*x - y*y - 4.0,
        colors::RED,
        4.0)
    );
    d2_plotter.add_object(GeoObj::new_parametric(
        |t| ((3.0 * t).sin(), (2.0 * t).sin()),
        (0.0, 2.0 * std::f64::consts::PI),
        colors::BLUE,
        6.0
    ));
    d2_plotter.add_object(GeoObj::new_parametric(
        |t| (t, 1.0/t),
        (-10.0, 10.0),
        colors::CYAN,
        6.0
    ));
    d2_plotter.add_object(GeoObj::new_implicit(
        |x,y| (x*x+y*y).sin()-(x*y).cos(),
        colors::GREEN,
        4.0
    ));
    d2_plotter.add_object(GeoObj::new_explicit(
        |x| (4.0 * x).cos() * (-0.75 * x * x).exp(),
        colors::MAGENTA,
        6.0
    ));
    d2_plotter.add_object(GeoObj::new_explicit(
        |x| x.tan() * 0.5,
        colors::ORANGE,
        6.0
    ));



    app.add_object(GeoObj::new_explicit(
        |x| {
            if (x<1.0){
                x
            } else {
                x.sin()
            }
        },
        colors::ORANGE,
        6.0
    ));

     */
    event_loop.run_app(&mut d2_plotter).unwrap();

}


//
pub fn main_d3() {
    let event_loop = EventLoop::new().unwrap();
    let mut d3_plotter = D3Plotter::new();

    /*

    // 蓝色游泳圈
    let torus_mesh = MeshData::new_parametric_surface(
        |u, v| {
            let r_major = 3.0;
            let r_minor = 1.2;
            Vec3::new(
                (r_major + r_minor * v.cos()) * u.cos(),
                (r_major + r_minor * v.cos()) * u.sin(),
                r_minor * v.sin()
            ) + Vec3::new(0.0, -8.0, 0.0)
        },
        (0.0, std::f64::consts::TAU),
        (0.0, std::f64::consts::TAU),
        60, 30
    );
    d3_plotter.add_object(GeoObjD3::new_surface(torus_mesh, colors::BLUE)); // 蓝色

    // 红色球面
    let sphere_mesh = MeshData::new_parametric_surface(
        |u, v| {
            let r = 2.0;
            Vec3::new(r * v.sin() * u.cos(), r * v.sin() * u.sin(), r * v.cos()) + Vec3::new(5.0, 0.0, 0.0)
        },
        (0.0, std::f64::consts::TAU),
        (0.0, std::f64::consts::PI),
        40, 40
    );
    d3_plotter.add_object(GeoObjD3::new_surface(sphere_mesh, colors::RED));

    // 线框球体
    let spiral_mesh = MeshData::new_parametric_surface(
        |u, v| {
            let r = 2.0;
            // 偏移球心到 (5,0,0)
            Vec3::new(r * v.sin() * u.cos(), r * v.sin() * u.sin(), r * v.cos()) + Vec3::new(0.0, 0.0, 5.0)
        },
        (0.0, std::f64::consts::TAU),
        (0.0, std::f64::consts::PI),
        40, 40
    );
    d3_plotter.add_object(GeoObjD3 {
        mesh: spiral_mesh,
        color: colors::GREEN,
        topology: wgpu::PrimitiveTopology::LineList, // 线框模式
        use_lighting: false,
        is_transparent: false,
    });


    // 绿色螺旋
    let helix_curve = ParametricCurveSolver::solve(
        |t| {
            Vec3::new(t.cos() * 1.0, t.sin() * 1.0, t/4.5) + Vec3::new(0.0, 5.0, 0.0)
        },
        (0.0, 6.0 * std::f64::consts::PI), // t range
        0.05,
        15,
        200
    );
    d3_plotter.add_object(GeoObjD3::new_surface(helix_curve, colors::GREEN));

    // 扭结曲线 (Knot) 黄色管子
    let knot_curve = ParametricCurveSolver::solve(
        |t| {
            let r = 1.0;
            let x = r * (t.cos() + 2.0 * (2.0 * t).cos());
            let y = r * (t.sin() - 2.0 * (2.0 * t).sin());
            let z = r * 2.0 * (3.0 * t).sin();
            Vec3::new(x, y, z) + Vec3::new(-8.0, 0.0, 0.0)
        },
        (0.0, 2.0 * std::f64::consts::PI),
        0.3,
        16,
        300
    );
    d3_plotter.add_object(GeoObjD3::new_surface(knot_curve, colors::YELLOW));


    // x^{2}+y^{2}+z^{2}+\sin4x+\sin4y+\sin4z=a
    let gyroid_mesh = ImplicitSurfaceSolver::solve(
        &|x, y, z| x*x+y*y+z*z+(4.0*x).sin()+(4.0*y).sin()+(4.0*z).sin()-1.7,
        (-3.0, 2.0), (-3.0, 2.0), (-3.0, 2.0),
        88
    );
    d3_plotter.add_object(GeoObjD3::new_surface(gyroid_mesh, colors::PURPLE));


    let imp_2 = ImplicitSurfaceSolver::solve(
        &|x_, y_, z_|{
            let x = x_ - 5.0;
            let y = y_ - 5.0;
            let z = z_ * 2.0 - 1.5;
            (y*y+x*x).sqrt() - ((z*3.0).sin() * 0.5 + 2.0 - 0.5 * z) * 0.5
        },
        (0.0, 10.0), (0.0, 10.0), (0.0, 2.6),
        88
    );
    d3_plotter.add_object(GeoObjD3::new_surface(imp_2, colors::ORANGE));


     */
    let gyroid_mesh = ImplicitSurfaceSolver::solve(
        &|x, y, z|
            //(x*x+y*y+z*z).sin()- (x.cos()+y.cos()+z.cos()),
            //(x*x+y*y+z*z).tan()-(x*y*z).sin(),
            //(x*x+y*y+z*z).sin()-0.5,
            x*x+y*y-z*z-3.0,

        (-6.0, 6.0), (-6.0, 6.0), (-6.0, 6.0),
        80
    );
    d3_plotter.add_object(GeoObjD3::new_surface(gyroid_mesh, colors::RED));
    
    event_loop.run_app(&mut d3_plotter).unwrap();
}

//
fn run_test() {
    // main_d2();
    main_d3();
}