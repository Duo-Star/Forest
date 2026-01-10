use super::math_data::MathData;
use super::rpn::RPN;
#[derive(Clone, Debug)]
pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
    // 指数运算
    // Pow,
    // 三角函数
    Sin,
    Cos,
    Tan,
    //
    LoadPara(usize),
    LoadGlobal(usize),
    Push(MathData),
    //
    CallDef(usize, Vec<RPN>)
}
