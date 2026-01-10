#![allow(dead_code)]
// tril.rs
use super::vec3::Vec3;
use std::f64::consts::PI;

/// 汆 (Tril)
/// 空间中共起点而不共面的三个射线构成的几何体
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tril {
    pub p: Vec3, // 顶点 (O)
    pub a: Vec3, // 棱 A (单位向量)
    pub b: Vec3, // 棱 B (单位向量)
    pub c: Vec3, // 棱 C (单位向量)
}

impl Tril {
    /// 构造一个新的汆角
    /// 会自动将输入的射线向量归一化
    pub fn new(p: Vec3, v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        Tril {
            p,
            a: v1.unit(), // 对应 Dart: v1?.unit
            b: v2.unit(),
            c: v3.unit(),
        }
    }

    /// 标准正交汆 (直汆)
    pub fn standard() -> Self {
        Tril {
            p: Vec3::ZERO,
            a: Vec3::I,
            b: Vec3::J,
            c: Vec3::K,
        }
    }

    // ================== 核心线类 (Cuan Lines) ==================

    /// 衡棱线 (Balance Arris Vector)
    /// 定义：方向垂直于底面三角形 ABC，在单位汆中指向底面外心
    /// Dart: (c - a).cross(b - a)
    pub fn balance_arris_v(&self) -> Vec3 {
        (self.c - self.a).cross(self.b - self.a)
    }

    /// 衡面线 (Balance Plane Vector)
    /// 定义：到三个面距离相等的射线
    /// 汆论定理：一个汆的衡面线，与其子汆的衡棱线重合
    pub fn balance_plane_v(&self) -> Vec3 {
        self.child().balance_arris_v()
    }

    /// 贯线 (Through Vector)
    /// 定义：指向底面重心
    /// Dart: a + b + c
    pub fn through_v(&self) -> Vec3 {
        self.a + self.b + self.c
    }

    // ================== 汆的分娩 (Duality) ==================

    /// 子汆 (Child Tril / Dual Tril)
    /// 生成逻辑：棱方向垂直于母汆的面
    /// Dart: c.cross(b), b.cross(a), a.cross(c)
    pub fn child(&self) -> Self {
        // 注意：这里保留了 Dart 代码中的叉积顺序
        // 数学上这对应了一组对偶基（可能带有手性翻转，取决于坐标系定义）
        Tril::new(
            self.p,
            self.c.cross(self.b), // A' 垂直于面 OBC
            self.b.cross(self.a), // B' 垂直于面 OAB (注：Dart逻辑如此，通常可能是 AxC)
            self.a.cross(self.c), // C' 垂直于面 OCA
        )
    }

    /// 孙汆 (Grandson Tril)
    /// 理论性质：应当与原汆形状一致（方向可能反向）
    pub fn grandson(&self) -> Self {
        self.child().child()
    }

    // ================== 汆的度量 (Metrics) ==================

    /// 获取三个棱角 (面角) alpha, beta, gamma
    /// 返回值: (angle_BOC, angle_AOC, angle_AOB)
    pub fn face_angles(&self) -> (f64, f64, f64) {
        let alpha = self.b.dot(self.c).acos(); // a 对应的对面角
        let beta = self.a.dot(self.c).acos(); // b 对应的对面角
        let gamma = self.a.dot(self.b).acos(); // c 对应的对面角
        (alpha, beta, gamma)
    }

    /// 获取三个二面角 (Dihedral Angles)
    /// 利用子汆的棱角互补性质计算: Dihedral + Child_Face_Angle = PI
    pub fn dihedral_angles(&self) -> (f64, f64, f64) {
        let (ca_alpha, ca_beta, ca_gamma) = self.child().face_angles();
        (PI - ca_alpha, PI - ca_beta, PI - ca_gamma)
    }

    /// 计算立体角 (Solid Angle) - 汆的大小
    /// 使用吕利耶定理 (L'Huilier's Theorem)
    pub fn solid_angle(&self) -> f64 {
        let (a, b, c) = self.face_angles();
        let s = (a + b + c) / 2.0;

        let t_s = (s / 2.0).tan();
        let t_sa = ((s - a) / 2.0).tan();
        let t_sb = ((s - b) / 2.0).tan();
        let t_sc = ((s - c) / 2.0).tan();

        let root = (t_s * t_sa * t_sb * t_sc).sqrt();
        4.0 * root.atan()
    }

    /// 计算单位平行六面体体积 (Gram Determinant 开根号)
    /// 这也是 "汆的正弦值"
    pub fn volume_parallelepiped(&self) -> f64 {
        self.a.triple_product(self.b, self.c).abs()
    }
}

// ================== 验证逻辑 (Verification) ==================
impl Tril {
    /// 验证：衡棱线是否垂直于底面 (几何定义)
    /// 返回点积结果，应接近 0.0
    pub fn verify_balance_arris_geometry(&self) -> f64 {
        let h = self.balance_arris_v(); // 衡棱线方向
        let ab = self.b - self.a; // 底面向量 AB
        let ac = self.c - self.a; // 底面向量 AC

        // 如果垂直，点积应为0
        h.dot(ab).abs() + h.dot(ac).abs()
    }

    /// 验证：直面汆定理 (Straight Face Theorem)
    /// 检查：如果 gamma 对应的二面角是 90度，是否 cos(gamma) == cos(alpha)*cos(beta)
    /// 返回：(是否是直二面角, 定理误差)
    pub fn verify_straight_face_theorem(&self) -> (bool, f64) {
        let (_, _, di_c) = self.dihedral_angles();

        // 判定是否为直二面角 (容差 1e-4)
        if (di_c - PI / 2.0).abs() > 1e-4 {
            return (false, 0.0);
        }

        let (alpha, beta, gamma) = self.face_angles();
        let left = gamma.cos();
        let right = alpha.cos() * beta.cos();

        (true, (left - right).abs())
    }
}

//
#[cfg(test)]
mod tests {
    use super::*; // 导入父级模块的所有内容（包括你的结构体和方法）

    #[test]
    fn test() {
        println!("=== 汆论 (Tril Theory) 验证系统 ===");

        // 1. 创建一个一般的随机汆
        let o = Vec3::ZERO;
        let a = Vec3::new(1.0, 0.2, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.5);
        let c = Vec3::new(0.3, 0.3, 1.0);

        let tril = Tril::new(o, a, b, c);
        println!("创建一个汆 O-ABC:");
        println!("  A: {}", tril.a);
        println!("  B: {}", tril.b);
        println!("  C: {}", tril.c);

        // 2. 验证衡棱线性质
        println!("\n[验证] 衡棱线 (Heng-Leng Line):");
        let hl = tril.balance_arris_v();
        println!("  向量: {}", hl);
        let error = tril.verify_balance_arris_geometry();
        println!("  与底面向量点积误差 (应为0): {:.10}", error);
        if error < 1e-9 {
            println!("  => 验证通过：衡棱线垂直于底面！");
        }

        // 3. 验证衡面线定理 (子汆关系)
        println!("\n[验证] 衡面线定理 (子汆关系):");
        let hm = tril.balance_plane_v(); // 通过 child.balanceArris 计算
                                         // 几何意义验证：衡面线到三个面的距离应该相等
                                         // 面OAB的法向量是 a x b (归一化)
        let n_ab = tril.a.cross(tril.b).unit();
        let n_bc = tril.b.cross(tril.c).unit();
        let n_ca = tril.c.cross(tril.a).unit();

        // 投影距离 = |P . n|
        let d1 = hm.dot(n_ab).abs();
        let d2 = hm.dot(n_bc).abs();
        let d3 = hm.dot(n_ca).abs();

        // 我们归一化 hm 来比较投影比例，或者直接看它们是否相等
        println!("  衡面线到各面投影距离: {:.4}, {:.4}, {:.4}", d1, d2, d3);
        let dev = (d1 - d2).abs() + (d2 - d3).abs();
        if dev < 1e-9 {
            println!("  => 验证通过：衡面线到三面距离相等！");
        } else {
            // 注意：如果 Dart 代码里的子汆定义有手性翻转，这里可能需要调整法向量方向，但距离绝对值应相等
            println!(
                "  => 警告：距离存在偏差 {:.10}，请检查子汆叉积顺序定义。",
                dev
            );
        }

        // 4. 验证直面汆定理
        println!("\n[验证] 直面汆定理:");
        // 构造一个直面汆：让面 OAC 和 面 OBC 垂直
        // 方法：让 C 沿 Z 轴，A 在 XZ 平面，B 在 YZ 平面
        let t_straight = Tril::new(
            Vec3::ZERO,
            Vec3::new(1.0, 0.0, 1.0), // XZ平面
            Vec3::new(0.0, 1.0, 1.0), // YZ平面
            Vec3::new(0.0, 0.0, 1.0), // Z轴
        );

        let (is_right, err) = t_straight.verify_straight_face_theorem();
        if is_right {
            println!("  构造了一个直二面角汆。");
            println!("  cos(gamma) - cos(alpha)cos(beta) 误差: {:.10}", err);
            if err < 1e-9 {
                println!("  => 验证通过：直面汆定理成立！");
            }
        } else {
            println!("  未能构造出直二面角汆，请检查坐标。");
        }

        // 5. 吕利耶定理度量
        println!("\n[计算] 汆的度量 (L'Huilier 立体角):");
        let omega = tril.solid_angle();
        println!("  立体角 Omega: {:.6} sr", omega);
        println!("  占全空间的比例: {:.2}%", omega / (4.0 * PI) * 100.0);

        // 6. 对偶循环验证 (孙汆)
        println!("\n[验证] 孙汆回归:");
        let grand = tril.grandson();
        // 孙汆的方向可能与原汆反向（取决于叉积次数），我们比较绝对值或平行性
        let is_same = grand.a.is_parallel(tril.a)
            && grand.b.is_parallel(tril.b)
            && grand.c.is_parallel(tril.c);
        if is_same {
            println!("  => 验证通过：孙汆的棱与原汆平行 (对偶的对偶是自身)");
        } else {
            println!("  => 验证失败：孙汆发生形变");
        }
    }
}
