use std::io::stdin;

type NodeID = usize;

struct UnionFind {
    pub data: Vec<i32>
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self { data: vec![-1; size] }
    }

    pub fn unite(&mut self, x: NodeID, y: NodeID) {
        let (mut x, mut y) = (self.root(x), self.root(y));
        if x == y { return; }
        if self.data[y] < self.data[x] {
            std::mem::swap(&mut x, &mut y);
        }
        self.data[x] += self.data[y];
        self.data[y] = x as i32;
    }

    pub fn is_same(&mut self, x: NodeID, y: NodeID) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn size(&mut self, x: NodeID) -> usize {
        let root_x = self.root(x) as NodeID;
        self.data[root_x].abs() as usize
    }

    fn root(&mut self, x: NodeID) -> NodeID {
        if self.data[x] < 0 {
            return x;
        }
        self.data[x] = self.root(self.data[x] as NodeID) as i32;
        self.data[x] as NodeID
    }
}

fn main() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    let mut nums = input.split_whitespace().map(|e| e.parse::<i32>().unwrap());
    let n = nums.next().unwrap() as usize;
    let q = nums.next().unwrap();

    let mut union_find = UnionFind::new(n);

    for _i in 0..q {
        input = String::new();
        let _ = stdin().read_line(&mut input);
        let mut nums = input.split_whitespace().map(|e| e.parse::<i32>().unwrap());
        let t = nums.next().unwrap();
        let u = nums.next().unwrap() as usize;
        let v = nums.next().unwrap() as usize;
        if t == 0 {
            union_find.unite(u, v);
        } else {
            println!("{}", union_find.is_same(u, v) as i32);
        }
    }
}
