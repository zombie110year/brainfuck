//! BrainFuck Virtual Machine

use std::collections::HashMap;
use std::io::{Read, Write};

/// # BrainFuck Virtual Machine
#[derive(Debug)]
pub struct BFVM {
    /// Vec 数据结构可以支持 BFVM 使用尽可能长的数组
    mem: Vec<u8>,
    /// 用于记录指针位置
    p: usize,
    /// 用于标记指令执行位置
    codep: usize,
    /// 跳转标记
    jump_sign: HashMap<usize, usize>,
}

// 对外 API
impl BFVM {
    pub fn new() -> Self {
        Self {
            mem: vec![0],
            p: 0,
            codep: 0,
            jump_sign: HashMap::new(),
        }
    }

    pub fn run(&mut self, code: String) {
        let code: Vec<char> = code.chars().collect();
        let codel = code.len();
        self.scan_jump(&code);
        while self.codep < codel {
            let op = code[self.codep];
            match op {
                '>' => self.op_right(),
                '<' => self.op_left(),
                '+' => self.op_add(),
                '-' => self.op_sub(),
                '.' => self.op_out(),
                ',' => self.op_in(),
                '[' => self.op_jump(),
                ']' => self.op_jumpback(),
                _ => (),
            }
            self.codep += 1;
        }
    }
}

// 各操作的实现
impl BFVM {
    fn scan_jump(&mut self, code: &Vec<char>) {
        let mut stack = Vec::new();
        for (i, c) in code.iter().enumerate() {
            match c {
                '[' => stack.push(i),
                ']' => {
                    if let Some(j) = stack.pop() {
                        self.jump_sign.insert(j, i);
                        self.jump_sign.insert(i, j);
                    } else {
                        panic!(
                            "第 {} 个字符（指令 '{}'）：在前方缺少跳转标记 '['",
                            i + 1,
                            c
                        );
                    }
                }
                _ => (),
            }
        }
        if !stack.is_empty() {
            panic!("存在未闭合的跳转指令 '['");
        }
    }

    fn op_right(&mut self) {
        self.p += 1;
        // 由于 index 和 length 相差 1，因此这里在相等时就需要增长一位
        if self.p == self.mem.len() {
            self.mem.push(0);
        }
    }

    fn op_left(&mut self) {
        self.p -= 1;
    }

    fn op_add(&mut self) {
        self.mem[self.p] = self.mem[self.p].wrapping_add(1);
    }

    fn op_sub(&mut self) {
        self.mem[self.p] = self.mem[self.p].wrapping_sub(1);
    }

    fn op_out(&self) {
        let c = self.mem[self.p];
        std::io::stdout()
            .lock()
            .write(&[c])
            .and_then(|_| std::io::stdout().lock().flush())
            .unwrap();
    }

    // 需要一次性输入所有，否则会把回车换行也读进去
    fn op_in(&mut self) {
        let mut buf = &mut self.mem[self.p..self.p + 1];
        std::io::stdin().lock().read(&mut buf).unwrap();
    }

    fn op_jump(&mut self) {
        if self.mem[self.p] == 0 {
            if let Some(jumpto) = self.jump_sign.get(&self.codep) {
                self.codep = *jumpto;
            }
        }
    }

    fn op_jumpback(&mut self) {
        if self.mem[self.p] != 0 {
            if let Some(jumpto) = self.jump_sign.get(&self.codep) {
                self.codep = *jumpto;
            }
        }
    }
}
