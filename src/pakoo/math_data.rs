use crate::math_forest::geometry::d3::linear::vec3::Vec3;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::Arc;

use super::rpn::RPN;

#[derive(Debug, Clone)]
pub enum MathData {
    None,
    Num(f64),
    Vec(Vec3),
    Fun { para_count: usize, body: Arc<RPN> },
}

// --- 核心修改：实现 Default ---
// 这样 [MathData; 32] 才能被初始化
impl Default for MathData {
    fn default() -> Self {
        MathData::Num(0.0)
    }
}

// --- 运算符重载逻辑 ---

impl Add for MathData {
    type Output = MathData;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MathData::Num(a), MathData::Num(b)) => MathData::Num(a + b),
            (MathData::Vec(a), MathData::Vec(b)) => MathData::Vec(a + b),
            (MathData::Num(_), MathData::Vec(_)) | (MathData::Vec(_), MathData::Num(_)) => {
                panic!("类型错误: 不能将 数字 和 向量 直接相加");
            }
            _ => panic!("类型错误: 运算类型不匹配"),
        }
    }
}

impl Sub for MathData {
    type Output = MathData;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MathData::Num(a), MathData::Num(b)) => MathData::Num(a - b),
            (MathData::Vec(a), MathData::Vec(b)) => MathData::Vec(a - b),
            _ => panic!("类型错误: 运算类型不匹配"),
        }
    }
}

impl Mul for MathData {
    type Output = MathData;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MathData::Num(a), MathData::Num(b)) => MathData::Num(a * b),
            (MathData::Vec(v), MathData::Num(s)) => MathData::Vec(v * s),
            (MathData::Num(s), MathData::Vec(v)) => MathData::Vec(v * s),
            (MathData::Vec(_), MathData::Vec(_)) => {
                panic!("类型错误: 向量与向量相乘需显式使用点乘或叉乘指令");
            }
            _ => panic!("类型错误: 运算类型不匹配"),
        }
    }
}

impl Div for MathData {
    type Output = MathData;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MathData::Num(a), MathData::Num(b)) => {
                if b == 0.0 { panic!("除以零！"); }
                MathData::Num(a / b)
            }
            (MathData::Vec(v), MathData::Num(s)) => {
                if s == 0.0 { panic!("向量除以零！"); }
                MathData::Vec(v * (1.0 / s))
            }
            _ => panic!("类型错误: 非法的除法运算"),
        }
    }
}

// --- 数学函数与实用方法 ---

impl MathData {
    #[inline(always)]
    pub fn sin(&self) -> MathData {
        if let MathData::Num(val) = self {
            MathData::Num(val.sin())
        } else {
            panic!("类型错误: sin 仅支持数字");
        }
    }
    #[inline(always)]
    pub fn cos(&self) -> MathData {
        if let MathData::Num(val) = self {
            MathData::Num(val.cos())
        } else {
            panic!("类型错误: cos 仅支持数字");
        }
    }
    #[inline(always)]
    pub fn tan(&self) -> MathData {
        if let MathData::Num(val) = self {
            MathData::Num(val.tan())
        } else {
            panic!("类型错误: tan 仅支持数字");
        }
    }

    // 注意：Rust 自动通过 #[derive(Clone)] 生成了 clone 方法。
    // 如果没有特殊逻辑，不需要手动实现 pub fn clone(&self)。
}