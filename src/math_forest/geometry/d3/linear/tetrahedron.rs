#![allow(dead_code)]
// tetrahedron.rs
use super::line3::Line3;
use super::tril::Tril; // 引入你之前定义的 Tril
use super::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Tetrahedron {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub d: Vec3,
}

impl Tetrahedron {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> Self {
        Tetrahedron { a, b, c, d }
    }

    // ==========================================
    // 1. 基础几何计算 (Basic Calculation)
    // ==========================================

    /// 计算四面体体积
    /// 公式: V = 1/6 * |(b-a) . ((c-a) x (d-a))|
    pub fn volume(&self) -> f64 {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let ad = self.d - self.a;
        ab.triple_product(ac, ad).abs() / 6.0
    }

    /// 计算四个面的面积
    /// 返回元组: (Area_BCD, Area_ACD, Area_ABD, Area_ABC)
    /// 对应顶点: (A对面, B对面, C对面, D对面)
    pub fn areas(&self) -> (f64, f64, f64, f64) {
        let s_a = (self.c - self.b).cross(self.d - self.b).len() * 0.5; // BCD
        let s_b = (self.c - self.a).cross(self.d - self.a).len() * 0.5; // ACD
        let s_c = (self.b - self.a).cross(self.d - self.a).len() * 0.5; // ABD
        let s_d = (self.b - self.a).cross(self.c - self.a).len() * 0.5; // ABC
        (s_a, s_b, s_c, s_d)
    }

    /// 总表面积
    pub fn surface_area(&self) -> f64 {
        let (s1, s2, s3, s4) = self.areas();
        s1 + s2 + s3 + s4
    }

    // ==========================================
    // 2. 汆获取方法 (Get Trils)
    // ==========================================

    /// 获取顶点 A 处的汆 (O=A, 射向B, C, D)
    pub fn tril_a(&self) -> Tril {
        Tril::new(self.a, self.b - self.a, self.c - self.a, self.d - self.a)
    }

    /// 获取顶点 B 处的汆 (O=B, 射向A, C, D)
    /// 注意：向量方向必须是从顶点向外射出
    pub fn tril_b(&self) -> Tril {
        Tril::new(self.b, self.a - self.b, self.c - self.b, self.d - self.b)
    }

    /// 获取顶点 C 处的汆
    pub fn tril_c(&self) -> Tril {
        Tril::new(self.c, self.a - self.c, self.b - self.c, self.d - self.c)
    }

    /// 获取顶点 D 处的汆
    pub fn tril_d(&self) -> Tril {
        Tril::new(self.d, self.a - self.d, self.b - self.d, self.c - self.d)
    }

    // ==========================================
    // 3. 汆论高级应用 (Incenter & Sphere Centers)
    // ==========================================

    /// 计算内切球球心 (Incenter)
    ///
    /// 《汆论》应用：
    /// 衡面线 (Balance Plane Line) 是到三个面距离相等的点的轨迹。
    /// 四面体的内心即为所有顶点衡面线的交点。
    /// 我们只需计算 Tril_A 和 Tril_B 的衡面线交点即可。
    pub fn incenter(&self) -> Vec3 {
        // 1. 获取 A 和 B 处的汆
        let ta = self.tril_a();
        let tb = self.tril_b();

        // 2. 获取各自的衡面线 (直线)
        // 注意：balance_plane_v 返回的是方向向量，我们需要构造射线
        let line_a = Line3::new(ta.p, ta.balance_plane_v());
        let line_b = Line3::new(tb.p, tb.balance_plane_v());

        // 3. 计算交点
        line_a.intersection(&line_b)
    }

    /// 计算内切球半径
    /// r = 3V / S_total
    pub fn inradius(&self) -> f64 {
        3.0 * self.volume() / self.surface_area()
    }

    /// [实验性] 衡棱线交点
    ///
    /// 《汆论》应用：
    /// 衡棱线 (Balance Arris Line) 是到三条棱距离相等的点的轨迹。
    /// 在正四面体或等面四面体中，它们交于外接球球心或“切棱球”相关中心。
    /// 对于一般四面体，这可以作为一个特殊的几何中心参考点。
    pub fn balance_arris_center(&self) -> Vec3 {
        let ta = self.tril_a();
        let tb = self.tril_b();

        let line_a = Line3::new(ta.p, ta.balance_arris_v());
        let line_b = Line3::new(tb.p, tb.balance_arris_v());

        // 在一般四面体中，四条衡棱线不一定共点（异面）
        // 这里返回 A 和 B 衡棱线的公垂线中点作为近似
        line_a.intersection(&line_b)
    }
}

//

#[cfg(test)]
mod tests {
    use super::*; // 导入父级模块的所有内容
    use rand::prelude::*;
    use rand::Rng;

    #[test]
    fn test_tetrahedron() {
        println!("\n=== 四面体与汆论应用验证 ===");

        // 构造一个正四面体 (顶点在立方体交错点上，方便验证)
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(1.0, -1.0, -1.0);
        let c = Vec3::new(-1.0, 1.0, -1.0);
        let d = Vec3::new(-1.0, -1.0, 1.0);

        let tet = Tetrahedron::new(a, b, c, d);

        println!("体积: {:.4}", tet.volume()); // 正四面体边长 2*sqrt(2)，体积应为 8/3 ≈ 2.6667

        // 1. 获取顶点 A 的汆
        let t_a = tet.tril_a();
        println!("顶点A的立体角: {:.4} sr", t_a.solid_angle());

        // 2. 利用汆论计算内心
        let incenter = tet.incenter();
        let theory_incenter = Vec3::ZERO; // 正四面体中心在原点
        println!("计算内心 (衡面线交点): {}", incenter);
        println!("  与理论值误差: {:.10}", incenter.dis(theory_incenter));

        // 3. 利用汆论计算衡棱线中心
        let arris_center = tet.balance_arris_center();
        println!("衡棱线中心: {}", arris_center);
        println!("  与原点误差: {:.10}", arris_center.dis(theory_incenter));

        if incenter.dis(theory_incenter) < 1e-9 {
            println!("=> 验证成功：利用衡面线(子汆衡棱线)成功求出内心！");
        }
    }
    fn sin_test() {}
}
