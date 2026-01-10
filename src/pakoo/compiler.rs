// src/parser/compiler.rs
use super::token::{Lexer, Token};
use crate::core::symbol_table::SymbolTable;
use crate::pakoo::math_data::MathData;
use crate::pakoo::op::Op; // 假设 Op 定义在这里

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Sum,     // + -
    Product, // * /
    Power,   // ^
    Prefix,  // -X (负号)
    Call,    // myFunc(x)
}

// 编译结果：包含字节码和依赖关系
pub struct CompileResult {
    pub ops: Vec<Op>,
    pub dependencies: Vec<usize>, // 这个公式依赖了哪些全局 ID
}

pub struct Compiler<'a> {
    lexer: Lexer<'a>,
    symbol_table: &'a mut SymbolTable,
}

impl<'a> Compiler<'a> {
    pub fn new(input: &'a str, table: &'a mut SymbolTable) -> Self {
        Self {
            lexer: Lexer::new(input),
            symbol_table: table,
        }
    }

    pub fn compile(&mut self) -> CompileResult {
        let mut output_queue: Vec<Op> = Vec::new();
        let mut op_stack: Vec<(Token, Precedence)> = Vec::new(); // 存操作符和优先级
        let mut dependencies: Vec<usize> = Vec::new();

        let mut token = self.lexer.next_token();

        // 简单的状态机，用于区分一元减号和减法
        let mut expect_operand = true;

        while token != Token::EOF {
            match token {
                Token::Number(val) => {
                    output_queue.push(Op::Push(MathData::Num(val)));
                    expect_operand = false;
                }
                Token::Identifier(ref name) => {
                    // 预读下一个 token 判断是变量还是函数调用
                    // 注意：这里的 Lexer 实现比较简单，实际上可能需要 peek
                    // 假设我们在 identifier 后如果遇到 LParen 则是函数

                    // 暂时简化：如果是内置函数（如 sin），生成 Op::Sin (如果不只是 CallDef)
                    // 如果是普通变量：
                    let id = self.symbol_table.get_or_create_id(name);

                    // 这里有一个歧义处理：Desmos 中 f(x) 是调用，x*y 是乘法
                    // 我们简化处理：如果是标识符，先当做 LoadGlobal
                    // 如果后面跟着 '('，Shunting Yard 的逻辑会处理成 Call

                    // 在纯 Shunting Yard 中，标识符通常直接入输出队列（作为变量）
                    // 或者入栈（作为函数）。我们需要区分。
                    // 为了简化，这里假设所有 Identifier 都是 LoadGlobal
                    // 真正的函数调用处理需要在遇到 '(' 时回溯或特殊标记

                    // 修正逻辑：先不推入输出队列，看栈顶？
                    // 更好的方式：Identifier 入栈或者直接入队？

                    // 采用标准做法：
                    // 1. 如果是变量 -> 输出队列
                    // 2. 如果是函数名 -> 压入操作符栈

                    // 由于我们不知道它是不是函数，我们得看后面有没有 '('。
                    // 但标准的 Shunting Yard 处理函数比较麻烦。

                    // 【关键策略】：所有标识符视为 LoadGlobal(id)
                    // 如果是函数定义的名字，这在 Op::Call 逻辑里处理

                    output_queue.push(Op::LoadGlobal(id));
                    dependencies.push(id);
                    expect_operand = false;
                }
                Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Caret => {
                    let curr_prec = self.get_precedence(&token, expect_operand);

                    // 处理一元运算符 (-5)
                    // 如果是 Minus 且 expect_operand 为 true，这是一元负号
                    // 可以将其视为特殊操作符，或者 0 - x

                    while let Some((top_op, top_prec)) = op_stack.last() {
                        if top_op == &Token::LParen {
                            break;
                        }
                        if *top_prec >= curr_prec {
                            self.pop_op_to_queue(op_stack.pop().unwrap().0, &mut output_queue);
                        } else {
                            break;
                        }
                    }
                    op_stack.push((token.clone(), curr_prec));
                    expect_operand = true;
                }
                Token::LParen => {
                    op_stack.push((token.clone(), Precedence::Lowest));
                    expect_operand = true;
                }
                Token::RParen => {
                    let mut found_paren = false;
                    while let Some((op, _)) = op_stack.pop() {
                        if op == Token::LParen {
                            found_paren = true;
                            break;
                        }
                        self.pop_op_to_queue(op, &mut output_queue);
                    }
                    if !found_paren {
                        panic!("Mismatched parentheses");
                    }

                    // 如果栈顶是函数，也要弹出函数并加入 Apply 指令
                    // (当前简化版暂未实现函数名入栈，视所有 ident 为变量)
                    expect_operand = false;
                }
                Token::Comma => {
                    // 函数参数分隔符
                    while let Some((top_op, _)) = op_stack.last() {
                        if top_op == &Token::LParen {
                            break;
                        }
                        self.pop_op_to_queue(op_stack.pop().unwrap().0, &mut output_queue);
                    }
                    expect_operand = true;
                }
                _ => {}
            }
            token = self.lexer.next_token();
        }

        while let Some((op, _)) = op_stack.pop() {
            if op == Token::LParen {
                panic!("Mismatched parentheses");
            }
            self.pop_op_to_queue(op, &mut output_queue);
        }

        CompileResult {
            ops: output_queue,
            dependencies,
        }
    }

    fn get_precedence(&self, token: &Token, is_unary: bool) -> Precedence {
        match token {
            Token::Plus | Token::Minus => {
                if is_unary {
                    Precedence::Prefix
                } else {
                    Precedence::Sum
                }
            }
            Token::Star | Token::Slash => Precedence::Product,
            Token::Caret => Precedence::Power,
            Token::LParen => Precedence::Call, // 函数调用优先级最高
            _ => Precedence::Lowest,
        }
    }

    fn pop_op_to_queue(&self, token: Token, queue: &mut Vec<Op>) {
        match token {
            Token::Plus => queue.push(Op::Add),
            Token::Minus => queue.push(Op::Sub),
            Token::Star => queue.push(Op::Mul),
            Token::Slash => queue.push(Op::Div),
            // 注意：Power 等需要自行实现 Op::Pow
            _ => {}
        }
    }
}
