use std::sync::Arc; // <--- 1. 记得引入这个

use super::env::Env;
use super::math_data::MathData;
use super::rpn::RPN;

// 行
#[derive(Debug)]
pub(crate) enum Slice {
    Var { data: MathData },
    Call { body: RPN },
    Def { para_count: usize, body: RPN },
}

impl Slice {
    pub fn clone(&self) -> Slice {
        match self {
            Slice::Var { data } => Slice::Var { data: data.clone() },
            Slice::Call { body } => Slice::Call { body: body.clone() },
            Slice::Def { para_count, body } => Slice::Def {
                para_count: *para_count,
                body: body.clone(),
            },
        }
    }

    pub fn eval(&mut self, env_data: &Vec<MathData>) -> MathData {
        match self {
            Slice::Var { data } => data.clone(),
            // 注意：因为 RPN::eval 签名变了，这里需要传空参数 &[]
            Slice::Call { body } => body.eval(env_data, &[]),
            Slice::Def { para_count, body } => {
                // 返回函数对象
                MathData::Fun {
                    para_count: *para_count,
                    // 修复错误：将 RPN 包装进 Arc
                    // 这里虽然还有一次 clone，但这是在"定义函数"时发生的（只发生一次）
                    // 在"调用函数"时已经没有 clone 了，所以是可以接受的
                    body: Arc::new(body.clone()),
                }
            }
        }
    }
}
