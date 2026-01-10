use std::clone;
use rand_distr::num_traits::real::Real;
use super::math_data::MathData;
use super::op::Op;

#[derive(Clone, Debug)]
pub(crate) struct RPN {
    op: Vec<Op>,
}

impl RPN {
    pub fn none() -> Self {
        RPN { op: Vec::new() }
    }

    pub fn new(op: Vec<Op>) -> Self {
        RPN { op }
    }

    // 修改签名：
    // 1. &mut self -> &self (不需要修改自身状态)
    // 2. env_data 类型放宽为 slice (兼容 &Vec)
    // 3. 新增 args 参数，用于传入当前上下文的参数
    pub fn eval(&self, env_data: &[MathData], args: &[MathData]) -> MathData {
        // println!("--- 开始运行 ---");

        // 1. 创建局部栈（替代原本结构体内的 stack）
        let mut stack: Vec<MathData> = Vec::with_capacity(16);

        // 2. 不需要 clone self.op，直接遍历引用
        for instruction in &self.op {
            match instruction {
                // 数据压栈
                Op::Push(val) => stack.push(val.clone()),

                // 运算指令：传入局部栈进行操作
                Op::Add => Self::binary_op(&mut stack, |a, b| a + b),
                Op::Sub => Self::binary_op(&mut stack, |a, b| a - b),
                Op::Mul => Self::binary_op(&mut stack, |a, b| a * b),
                Op::Div => Self::binary_op(&mut stack, |a, b| a / b),
                //
                // Op::Pow => Self::binary_op(&mut stack, |a, b| a.powf(b)),
                Op::Sin => Self::unary_op(&mut stack, |a| a.sin()),
                Op::Cos => Self::unary_op(&mut stack, |a| a.cos()),
                Op::Tan => Self::unary_op(&mut stack, |a| a.tan()),



                // 全局变量加载
                Op::LoadGlobal(gi) => stack.push(env_data[*gi].clone()),

                // 参数加载：直接从传入的 args 读取，无需中间存储
                Op::LoadPara(pi) => {
                    // 这里的 args 是调用 eval 时传入的参数列表
                    stack.push(args[*pi].clone());
                }

                // 函数调用
                Op::CallDef(index, para_rpns) => {
                    // 1. 计算传递给子函数的实参
                    let mut call_args = Vec::with_capacity(para_rpns.len());
                    for p_rpn in para_rpns {
                        // 递归调用 eval，注意这里传入当前的 env 和 args
                        // 这样支持 f(x + 1) 这种参数里包含表达式的情况
                        let arg_val = p_rpn.eval(env_data, args);
                        call_args.push(arg_val);
                    }

                    // 2. 获取函数对象（使用引用，避免 clone MathData）
                    let f = &env_data[*index];

                    match f {
                        MathData::Fun { para_count, body } => {
                            // 检查参数数量
                            if call_args.len() != *para_count {
                                panic!(
                                    "参数数量不匹配: 期望 {}, 实际 {}",
                                    para_count,
                                    call_args.len()
                                );
                            }

                            // 3. 核心优化：直接对 body (RPN) 的引用调用 eval
                            // 传入刚才计算好的 call_args
                            // 彻底消除了 body.clone() 的开销
                            let result = body.eval(env_data, &call_args);
                            stack.push(result);
                        }
                        _ => panic!("索引 {} 处的值不是函数", index),
                    }
                }
            }
        }

        // 返回栈顶元素
        stack.pop().expect("栈是空的，没有结果！")
    }

    // 辅助函数：处理二元运算
    // 现在变成关联函数，显式接收 stack 可变引用
    fn binary_op<F>(stack: &mut Vec<MathData>, op: F)
    where
        F: Fn(MathData, MathData) -> MathData,
    {
        // 栈底在 0，栈顶在末尾
        // pop() 第一次出来的是右操作数 (Right Hand Side)
        if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
            let result = op(lhs, rhs);
            stack.push(result);
        } else {
            panic!("非法操作：栈里的数字不够运算！");
        }
    }
    // 辅助函数：处理一元运算
    // 现在变成关联函数，显式接收 stack 可变引用
    fn unary_op<F>(stack: &mut Vec<MathData>, op: F)
    where
        F: Fn(MathData) -> MathData,
    {
        // 栈底在 0，栈顶在末尾
        // pop() 第一次出来的是操作数 (Operand)
        if let Some(operand) = stack.pop() {
            let result = op(operand);
            stack.push(result);
        } else {
            panic!("非法操作：栈里的数字不够运算！");
        }
    }
}
