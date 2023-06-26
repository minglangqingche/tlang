use super::op::*;

pub struct Chunk {
    ops: Vec<Opcode>,
    vals: Vec<f64>,
    line: Vec<(usize, u32)>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            vals: Vec::new(),
            line: Vec::new(),
        }
    }

    pub fn get_line(&self, op_index: usize) -> Option<u32> {
        let mut start = 0;
        let mut end = self.line.len() - 1;

        let get_index = |x| {
            let (index, _) = self.line[x];
            index
        };

        let get_line = | x | {
            let (_, line) = self.line[x];
            line
        };

        loop {
            let mid = (start + end) / 2;
            if op_index < get_index(mid) {
                end = mid - 1;
            }else if mid == self.line.len()-1 || op_index < get_index(mid+1) {
                return Some(get_line(mid));
            }else {
                start = mid + 1;
            }
        }
    }

    pub fn get_op(&self, index: usize) -> Option<&Opcode> {
        self.ops.get(index)
    }

    pub fn push_op(&mut self, op: Opcode, line: u32) {
        self.ops.push(op);

        if let Some(&(_, old_line)) = self.line.last() {
            if old_line != line {
                self.line.push((self.ops.len()-1, line));
            }
        }else {
            self.line.push((self.ops.len()-1, line));
        }
    }

    pub fn push_val(&mut self, val: f64) -> usize {
        self.vals.push(val);
        self.vals.len() - 1
    }

    pub fn get_val(&self, index: usize) -> Option<&f64> {
        self.vals.get(index)
    }

    pub fn op_len(&self) -> usize {
        self.ops.len()
    }
}

