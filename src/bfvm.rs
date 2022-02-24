//! BrainFuck Virtual Machine

/// # BrainFuck Virtual Machine
#[derive(Debug)]
pub struct BFVM {
    /// Vec 数据结构可以支持 BFVM 使用尽可能长的数组
    mem: Vec<u8>,
    /// 用于记录指针位置
    p: usize,
}

// 对外 API
impl BFVM {
    pub fn new() -> Self {
        Self { mem: vec![0], p: 0 }
    }

    pub fn run(&mut self, code: String) {
        for op in code.chars() {
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
        }
    }
}

// 各操作的实现
impl BFVM{
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
        self.mem[self.p] += 1;
    }

    fn op_sub(&mut self) {
        self.mem[self.p] -= 1;
    }

    fn op_out(&self) {
        todo!()
    }

    fn op_in(&self) {
        todo!()
    }

    fn op_jump(&self) {
        todo!()
    }

    fn op_jumpback(&self) {
        todo!()
    }
}
