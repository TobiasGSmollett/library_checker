use std::cmp::min;
use std::io::stdin;

struct RMQ {
    size: usize,
    data: Vec<i32>,
}

impl RMQ {
    pub fn new(size: usize) -> Self {
        Self {
            size: size,
            data: vec![std::i32::MAX; size * 2 + 1],
        }
    }

    pub fn update(&mut self, k: &usize, a: &i32) {
        let a = a.clone();
        let mut k = k.clone();
        k += self.size;
        self.data[k] = a;
        while k > 1 {
            self.data[k >> 1] = min(self.data[k], self.data[k ^ 1]);
            k >>= 1;
        }
    }

    pub fn query(&self, l: &usize, r: &usize) -> i32 {
        let mut l = l.clone() + self.size;
        let mut r = r.clone() + self.size - 1;
        let mut res = std::i32::MAX;
        while l <= r {
            if (l & 1) != 0 {
                res = min(res, self.data[l]);
            }
            if (r & 1) == 0 {
                res = min(res, self.data[r]);
            }
            l = (l + 1) >> 1;
            r = (r - 1) >> 1;
        }
        res
    }
}

fn main() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    let mut nums = input.split_whitespace().map(|e| e.parse::<i32>().unwrap());
    let n = nums.next().unwrap() as usize;
    let q = nums.next().unwrap();

    let mut rmq = RMQ::new(n);

    input = String::new();
    let _ = stdin().read_line(&mut input);
    let a = input.split_whitespace().map(|e| e.parse::<i32>().unwrap());

    for (i, v) in a.enumerate() {
        rmq.update(&i, &v);
    }

    for _i in 0..q {
        input = String::new();
        let _ = stdin().read_line(&mut input);
        let mut nums = input.split_whitespace().map(|e| e.parse::<i32>().unwrap());
        let l = nums.next().unwrap() as usize;
        let r = nums.next().unwrap() as usize;
        println!("{}", rmq.query(&l, &r));
    }
}
