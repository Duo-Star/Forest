#![allow(dead_code)]

// line3.rs
use super::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Line3 {
    pub origin: Vec3,    // 起点
    pub direction: Vec3, // 方向 (建议归一化)
}

impl Line3 {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Line3 {
            origin,
            direction: direction.unit(),
        }
    }

    /// 构造：两点确定一直线
    pub fn from_points(p1: Vec3, p2: Vec3) -> Self {
        Self::new(p1, p2 - p1)
    }

    /// 求直线上的一点 P = O + t * d
    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    /// 计算两条空间直线 (self 和 other) 的最近点参数 (t1, t2)
    /// 如果相交，距离为0；如果异面，则是公垂线段的端点
    /// 返回: (t_self, t_other)
    pub fn closest_points_params(&self, other: &Line3) -> (f64, f64) {
        let n1 = self.direction;
        let n2 = other.direction;
        let n1_dot_n2 = n1.dot(n2);

        let det = 1.0 - n1_dot_n2 * n1_dot_n2;
        let p21 = other.origin - self.origin;

        // 平行线处理 (行列式接近0)
        if det.abs() < 1e-9 {
            return (p21.dot(n1), 0.0);
        }

        let d1 = p21.dot(n1);
        let d2 = p21.dot(n2);

        let t1 = (d1 - n1_dot_n2 * d2) / det;
        let t2 = (n1_dot_n2 * d1 - d2) / det;

        (t1, t2)
    }

    /// 获取两条直线的“交点”
    /// 注意：在空间中两条线往往不严格相交（异面）。
    /// 此函数返回公垂线段的中点作为“最佳逼近交点”。
    /// 对于几何中心求解（如内心），理论上是严格相交的，此方法精确有效。
    pub fn intersection(&self, other: &Line3) -> Vec3 {
        let (t1, t2) = self.closest_points_params(other);
        let p1 = self.point_at(t1);
        let p2 = other.point_at(t2);

        // 返回中点
        (p1 + p2) * 0.5
    }

    /// 距离测试
    pub fn distance_to_point(&self, p: Vec3) -> f64 {
        let v = p - self.origin;
        let proj = v.project_vec(self.direction);
        (v - proj).len()
    }
}
