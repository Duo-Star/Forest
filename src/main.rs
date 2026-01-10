use std::io::{self, Write};
use std::time::Instant;

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
        
        _ => {
            println!("无效的输入 '{}'，改为'd2' 或 'd3'", input.trim());
        }
    }
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

