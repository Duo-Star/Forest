use std::f64::consts::PI;
use super::super::math_forest::geometry::d2::special::hyperelliptic::Hyperelliptic;
use super::super::math_forest::geometry::d2::linear::vec2::Vec2;

pub fn main_() {
    let curve = Hyperelliptic { a: 1.0, b: 1.0, m: 0.4 };

    // 测试点
    let target_point = Vec2::new(1.1, 0.8);

    println!("目标点: {:?}", target_point);
    println!("曲线参数: a={:.1}, b={:.1}, m={:.1}", curve.a, curve.b, curve.m);

    let (min_dist, t_opt, nearest_point) = curve.find_closest_point(target_point);

    println!("--- 计算结果 ---");
    println!("最小距离: {:.6}", min_dist);
    println!("最优参数 t: {:.6} rad ({:.1} degrees)", t_opt, t_opt.to_degrees());
    println!("曲线上最近点: ({:.6}, {:.6})", nearest_point.x, nearest_point.y);

    // 验证：法线方向是否通过目标点？
    // 理论上，(P_curve - P_target) 应该平行于法向量，或者说垂直于切向量
    let dp = curve.derivative_at(t_opt);
    let vec_to_target = dp-nearest_point;
    let dot_product = vec_to_target.dot(dp);

    println!("点积接近0{:.2e}", dot_product);
}