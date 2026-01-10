use super::math_data::MathData;
use super::op::Op;
use super::rpn::RPN;
use super::slice::Slice;

#[allow(dead_code)]
pub struct Env {
    slice: Vec<Slice>,
    pub(crate) data: Vec<MathData>,
}

#[allow(dead_code)]
impl Env {
    pub fn new() -> Self {
        Self {
            slice: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn add_slice(&mut self, slice: Slice) {
        self.slice.push(slice);
    }

    pub fn get_slice(&self, index: usize) -> &Slice {
        &self.slice[index]
    }

    pub fn get_data(&self, index: usize) -> &MathData {
        &self.data[index]
    }

    pub fn update(&mut self) -> MathData {
        // 先把 slice 取出来，避免直接在 self 上调用 iter
        for i in 0..self.slice.len() {
            let result = self.slice[i].eval(&self.data);
            self.data.push(result);
        }
        self.data.last().expect("Data should not be empty").clone()
    }

    pub fn fmt(&self) -> String {
        let mut s = String::new();
        s.push_str("--slice:\n");
        for i in 0..self.slice.len() {
            s.push_str(&format!("{:?}\n", self.slice[i]));
        }
        s.push_str("--data:\n");
        for i in 0..self.data.len() {
            s.push_str(&format!("{:?}\n", self.data[i]));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;
    use crate::pakoo::op::Op;

    #[test]
    fn test_1() {
        let mut env = Env::new();
        // a = 1.0
        env.add_slice(Slice::Var {
            data: MathData::Num(1.0),
        });
        // b = 1.0 + 2.0
        env.add_slice(Slice::Call {
            body: RPN::new(vec![
                Op::Push(MathData::Num(1.0)),
                Op::Push(MathData::Num(2.0)),
                Op::Add,
            ]),
        });
        // c = b + 2.0
        env.add_slice(Slice::Call {
            body: RPN::new(vec![
                Op::LoadGlobal(1),
                Op::Push(MathData::Num(2.0)),
                Op::Add,
            ]),
        });
        let data = env.update();
        println!("last data: {:?}", data);
        println!("env:\n {}", env.fmt());
    }

    #[test]
    fn test_2() {
        let mut env = Env::new();
        // f(x) = x + 2.0
        env.add_slice(Slice::Def {
            para_count: 1,
            body: RPN::new(vec![Op::LoadPara(0), Op::Push(MathData::Num(2.0)), Op::Add]),
        });
        // f(1.0) + 2.0
        env.add_slice(Slice::Call {
            body: RPN::new(vec![
                Op::CallDef(0, vec![RPN::new(vec![Op::Push(MathData::Num(1.0))])]),
                Op::Push(MathData::Num(2.0)),
                Op::Add,
            ]),
        });
        let data = env.update();
        println!("last data: {:?}", data);
        println!("env:\n {}", env.fmt());
    }

    #[test]
    fn test_3() {
        let mut env = Env::new();
        // f(x) = x + 2.0
        env.add_slice(Slice::Def {
            para_count: 1,
            body: RPN::new(vec![Op::LoadPara(0), Op::Push(MathData::Num(2.0)), Op::Add]),
        });
        // f(f(1.0)) + 2.0 -> 7.0
        env.add_slice(Slice::Call {
            body: RPN::new(vec![
                Op::CallDef(
                    0,
                    vec![RPN::new(vec![Op::CallDef(
                        0,
                        vec![RPN::new(vec![Op::Push(MathData::Num(1.0))])],
                    )])],
                ),
                Op::Push(MathData::Num(2.0)),
                Op::Add,
            ]),
        });
        let data = env.update();
        println!("last data: {:?}", data);
        println!("env:\n {}", env.fmt());
    }

    #[test]
    fn test_4() {
        let mut env = Env::new();
        // f(x) = x + 2.0
        env.add_slice(Slice::Def {
            para_count: 1,
            body: RPN::new(vec![Op::LoadPara(0), Op::Push(MathData::Num(2.0)), Op::Add]),
        });
        // g(x) = f(f(x)) + 1.0
        env.add_slice(Slice::Def {
            para_count: 1,
            body: RPN::new(vec![
                Op::CallDef(
                    0,
                    vec![RPN::new(vec![Op::CallDef(
                        0,
                        vec![RPN::new(vec![Op::LoadPara(0)])],
                    )])],
                ),
                Op::Push(MathData::Num(1.0)),
                Op::Add,
            ]),
        });
        // g(2.0) + 2.0
        env.add_slice(Slice::Call {
            body: RPN::new(vec![
                Op::CallDef(1, vec![RPN::new(vec![Op::Push(MathData::Num(2.0))])]),
                Op::Push(MathData::Num(2.0)),
                Op::Add,
            ]),
        });
        let data = env.update();
        println!("last data: {:?}", data);
        println!("env:\n {}", env.fmt());
    }

    #[test]
    fn test_5() {
        let start = Instant::now(); // 获取当前时间

        let mut env = Env::new();

        let rpn = RPN::new(vec![
            Op::Push(MathData::Num(1.0)),
            Op::Push(MathData::Num(1.0)),
            Op::Mul,
            Op::Push(MathData::Num(2.0)),
            Op::Push(MathData::Num(2.0)),
            Op::Mul,
            Op::Add,
            Op::Sin,
            Op::Push(MathData::Num(1.0)),
            Op::Push(MathData::Num(2.0)),
            Op::Add,
            Op::Push(MathData::Num(1.0)),
            Op::Add,
            Op::Div,
        ]);

        for i in 0..1000_00 {
            let a =rpn.eval(&env.data, &[]);
        }

        let duration = start.elapsed(); // 计算耗时
        println!("1000次循环总耗时: {:?}", duration);
        println!("平均每次耗时: {:?}", duration / 1000_00);
    }
}
