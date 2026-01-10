use crate::math_forest::geometry::d3::linear::vec3::Vec3;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::Arc;

use super::rpn::RPN;

#[derive(Debug, Clone)]
pub enum MathData {
    Num(f64),
    Vec(Vec3),
    Fun { para_count: usize, body: Arc<RPN> },
}

// --- 核心逻辑：在这里处理类型匹配 ---

impl Add for MathData {
    type Output = MathData;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MathData::Num(a), MathData::Num(b)) => MathData::Num(a + b),
            (MathData::Vec(a), MathData::Vec(b)) => MathData::Vec(a + b), // 假设 Vec3 实现了 Add
            (MathData::Num(_), MathData::Vec(_)) | (MathData::Vec(_), MathData::Num(_)) => {
                panic!("类型错误: 不能将 数字 和 向量 直接相加");
            }
            _ => {
                panic!("类型错误: 运算类型不匹配");
            }
        }
    }
}

impl Sub for MathData {
    type Output = MathData;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MathData::Num(a), MathData::Num(b)) => MathData::Num(a - b),
            (MathData::Vec(a), MathData::Vec(b)) => MathData::Vec(a - b),
            _ => {
                panic!("类型错误: 运算类型不匹配");
            }
        }
    }
}

impl Mul for MathData {
    type Output = MathData;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MathData::Num(a), MathData::Num(b)) => MathData::Num(a * b),
            // 向量数乘： v * 2.0
            (MathData::Vec(v), MathData::Num(s)) => MathData::Vec(v * s),
            // 向量数乘交换律： 2.0 * v
            (MathData::Num(s), MathData::Vec(v)) => MathData::Vec(v * s),
            // 两个向量相乘？通常可能有歧义，这里先panic，或者定义为逐分量相乘
            (MathData::Vec(_), MathData::Vec(_)) => {
                panic!("类型错误: 向量与向量不能直接使用 * (请使用 Dot 或 Cross 指令)");
            }
            _ => {
                panic!("类型错误: 运算类型不匹配");
            }
        }
    }
}

impl Div for MathData {
    type Output = MathData;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MathData::Num(a), MathData::Num(b)) => {
                if b == 0.0 {
                    panic!("除以零！");
                }
                MathData::Num(a / b)
            }
            // 向量除以标量： v / 2.0
            (MathData::Vec(v), MathData::Num(s)) => {
                if s == 0.0 {
                    panic!("向量除以零！");
                }
                MathData::Vec(v * (1.0 / s)) // 乘倒数
            }
            (MathData::Num(_), MathData::Vec(_)) => panic!("类型错误: 不能用数字除以向量"),
            (MathData::Vec(_), MathData::Vec(_)) => panic!("类型错误: 向量不能除以向量"),
            _ => {
                panic!("类型错误: 运算类型不匹配");
            }
        }
    }
}

impl MathData {
    pub fn sin(&self) -> MathData {
        match self {
            MathData::Num(val) => MathData::Num(val.sin()),
            _ => {
                panic!("类型错误: 不能对非数字进行 sin 运算");
            }
        }
    }
    pub fn cos(&self) -> MathData {
        match self {
            MathData::Num(val) => MathData::Num(val.cos()),
            _ => {
                panic!("类型错误: 不能对非数字进行 cos 运算");
            }
        }
    }
    pub fn tan(&self) -> MathData {
        match self {
            MathData::Num(val) => MathData::Num(val.tan()),
            _ => {
                panic!("类型错误: 不能对非数字进行 tan 运算");
            }
        }
    }
}

impl MathData {
    pub fn clone(&self) -> MathData {
        match self {
            MathData::Num(val) => MathData::Num(*val),
            MathData::Vec(vec) => MathData::Vec(vec.clone()),
            MathData::Fun { para_count, body } => MathData::Fun {
                para_count: *para_count,
                body: body.clone(),
            },
        }
    }
    //
    pub fn eval(&self, env_data: &Vec<MathData>) -> MathData {
        match self {
            MathData::Num(val) => MathData::Num(*val),
            MathData::Vec(vec) => MathData::Vec(vec.clone()),
            MathData::Fun { para_count, body } => {
                // 函数求值返回函数本身
                MathData::Fun {
                    para_count: *para_count,
                    body: body.clone(),
                }
            }
        }
    }
}
