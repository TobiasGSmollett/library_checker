use std::io::stdin;

struct BIT {
    size: usize,
    data: Vec<i64>,
}

impl BIT {
    pub fn new(size: usize) -> Self {
        Self {
            size: size + 1,
            data: vec![0; size * 2 + 1],
        }
    }

    pub fn add(&mut self, i: &usize, x: &i64) {
        let mut i = i.clone() as i64 + 1;
        while i <= (self.size as i64) {
            self.data[i as usize] += x;
            i += i & -i;
        }
    }

    pub fn sum(&self, i: &usize) -> i64 {
        let mut i = i.clone() as i64;
        let mut s = 0;
        while i > 0 {
            s += self.data[i as usize];
            i -= i & -i;
        }
        return s;
    }
}

fn main() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    let mut nums = input.split_whitespace().map(|e| e.parse::<i64>().unwrap());
    let n = nums.next().unwrap() as usize;
    let q = nums.next().unwrap();

    let mut bit = BIT::new(n);

    input = String::new();
    let _ = stdin().read_line(&mut input);
    let a = input.split_whitespace().map(|e| e.parse::<i64>().unwrap());

    for (i, v) in a.enumerate() {
        bit.add(&(i), &v);
    }

    for _i in 0..q {
        input = String::new();
        let _ = stdin().read_line(&mut input);
        let mut nums = input.split_whitespace().map(|e| e.parse::<i64>().unwrap());
        let flag = nums.next().unwrap();
        let l = nums.next().unwrap() as usize;
        let r = nums.next().unwrap() as usize;

        if flag == 0 {
            bit.add(&l, &(r as i64));
        } else {
            println!("{}", bit.sum(&(r)) - bit.sum(&l));
        }
    }
}
