use std::clone;
use rand_distr::num_traits::real::Real;
use super::math_data::MathData;
use super::op::Op;

#[derive(Clone, Debug)]
#[derive(Default)]
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

    const MAX_STACK_SIZE: usize = 32;
    pub fn eval(&self, env_data: &[MathData], args: &[MathData]) -> MathData {
        // println!("--- 开始运行 ---");
        // 1. 使用定长数组替代 Vec。
        // 要求 MathData 实现了 Default（例如默认是 Number(0.0)）
        let mut stack: [MathData; Self::MAX_STACK_SIZE] = Default::default();
        let mut top: usize = 0; // 栈顶指针

        // 2. 直接遍历指令引用
        for instruction in &self.op {
            //  使用 unsafe 块处理指令
            unsafe {
                match instruction {
                    Op::Push(val) => {
                        stack[top] = val.clone();
                        top += 1;
                    }

                    // 运算指令：改为直接操作数组和索引
                    Op::Add => {
                        top -= 1;
                        let rhs = std::mem::take(stack.get_unchecked_mut(top));
                        top -= 1;
                        let lhs = std::mem::take(stack.get_unchecked_mut(top));
                        *stack.get_unchecked_mut(top) = lhs + rhs;
                        top += 1;
                    }
                    Op::Sub => {
                        top -= 1;
                        let rhs = std::mem::take(stack.get_unchecked_mut(top));
                        top -= 1;
                        let lhs = std::mem::take(stack.get_unchecked_mut(top));
                        *stack.get_unchecked_mut(top) = lhs - rhs;
                        top += 1;
                    }
                    Op::Mul => {
                        top -= 1;
                        let rhs = std::mem::take(stack.get_unchecked_mut(top));
                        top -= 1;
                        let lhs = std::mem::take(stack.get_unchecked_mut(top));
                        *stack.get_unchecked_mut(top) = lhs * rhs;
                        top += 1;
                    }
                    Op::Div => {
                        top -= 1;
                        let rhs = std::mem::take(stack.get_unchecked_mut(top));
                        top -= 1;
                        let lhs = std::mem::take(stack.get_unchecked_mut(top));
                        *stack.get_unchecked_mut(top) = lhs / rhs;
                        top += 1;
                    }
                    Op::Sin => {
                        top -= 1;
                        let val = std::mem::take(stack.get_unchecked_mut(top));
                        *stack.get_unchecked_mut(top) = val.sin();
                        top += 1;
                    }
                    Op::Cos => {
                        top -= 1;
                        let val = std::mem::take(stack.get_unchecked_mut(top));
                        *stack.get_unchecked_mut(top) = val.cos();
                        top += 1;
                    }
                    Op::Tan => {
                        top -= 1;
                        let val = std::mem::take(stack.get_unchecked_mut(top));
                        *stack.get_unchecked_mut(top) = val.tan();
                        top += 1;
                    }

                    Op::LoadGlobal(gi) => {
                        stack[top] = env_data[*gi].clone();
                        top += 1;
                    }

                    Op::LoadPara(pi) => {
                        stack[top] = args[*pi].clone();
                        top += 1;
                    }

                    Op::CallDef(index, para_rpns) => {
                        // 假设函数参数最多 8 个
                        let mut call_args: [MathData; 8] = Default::default();

                        for (i, p_rpn) in para_rpns.iter().enumerate() {
                            if i < 8 {
                                call_args[i] = p_rpn.eval(env_data, args);
                            }
                        }

                        if let MathData::Fun { para_count, body } = &env_data[*index] {
                            // 传递切片 &[MathData] 而不是 Vec
                            stack[top] = body.eval(env_data, &call_args[..*para_count]);
                            top += 1;
                        }
                    }
                    // ... 其他指令
                }
            }
        }

        // 3. 转移所有权返回结果（使用 std::mem::take 避免 clone 数组中的最后一个元素）
        std::mem::take(&mut stack[0])
    }



    // 辅助函数：处理二元运算
    // 现在变成关联函数，显式接收 stack 可变引用
    fn binary_op<F>(stack: &mut Vec<MathData>, op: F)
    where
        F: Fn(MathData, MathData) -> MathData,
    {
        // 栈底在 0，栈顶在末尾
        // pop() 第一次出来的是右操作数 (Right Hand Side)
        let rhs = unsafe { stack.pop().unwrap_unchecked() };
        let lhs = unsafe { stack.pop().unwrap_unchecked() };
        let result = op(lhs, rhs);
        stack.push(result);

    }
    // 辅助函数：处理一元运算
    // 现在变成关联函数，显式接收 stack 可变引用
    fn unary_op<F>(stack: &mut Vec<MathData>, op: F)
    where
        F: Fn(MathData) -> MathData,
    {
        let operand = stack.pop();
        let result = op(unsafe { operand.unwrap_unchecked() });
        stack.push(result);
    }
}
