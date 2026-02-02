use std::io::{self, Write};
use std::time::Instant;
use crate::pakoo::env::Env;
use crate::pakoo::math_data::MathData;
use crate::pakoo::op::Op;
use crate::pakoo::rpn::RPN;
use crate::pakoo::slice::Slice;

mod graph;
mod math_forest;
mod test;
mod pakoo;

fn main() {
    println!("MathForest - Graph by Duo\n欢迎：663251235\n输入测试模式(d2/d3):\n");

    // 打印提示符并立即刷新到屏幕
    print!("> ");
    io::stdout().flush().unwrap();

    // 读取用户输入
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("无法读取");

    // 使用 trim() 去掉回车符，并转为小写以增加兼容性
    match input.trim().to_lowercase().as_str() {
        "d2" => {
            println!("d2 demo running");
            test::g23_test::main_d2();
        }
        "d3" => {
            println!("d3 demo running");
            test::g23_test::main_d3();
        }
        "ran_test" => {
            for i in 1..6 {
                let y: f64 = rand::random();
                println!("Random f64: {}", y);
            }
        }

        "nse" => {
            test::nse::main_();
        }
        "5" => {
            test_5() ;
        }
        "6" => {
            test_6();
        }
        
        _ => {
            println!("无效的输入 '{}'，改为'd2' 或 'd3'", input.trim());
        }
    }
}


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

    for i in 0..1000_0000 {
        let a =rpn.eval(&env.data, &[]);
    }

    let duration = start.elapsed(); // 计算耗时
    println!("10000000次循环总耗时: {:?}", duration);
    println!("平均每次耗时: {:?}", duration / 1000_0000);
}


fn test_6() {

    // f(x) = x + 2.0
    // g(x) = f(f(x)) + 1.0
    // g(2.0) + 2.0
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

    let start = Instant::now(); // 获取当前时间

    for i in 0..1000_0000 {
        let data = env.update();
    }

    let duration = start.elapsed(); // 计算耗时
    println!("test_6: 1000_0000次循环总耗时: {:?}", duration);
    println!("平均每次耗时: {:?}", duration / 1000_0000);
}


#[cfg(test)]
mod tests {

    #[test]
    fn t1() {
        let a: f64 = 2.48832;
        let b: f64 = 2.985984;
        let x = a.powf(b);
        let y = b.powf(a);
        println!("a^b, b^a : {}, {}", x, y);
        assert!(x == y);
    }
}

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

