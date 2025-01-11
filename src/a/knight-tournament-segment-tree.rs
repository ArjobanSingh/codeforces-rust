use std::io::{self, BufWriter, Read, Write};

mod trees {
    #[derive(Debug)]
    pub struct SegmentTree {
        n: usize,
        tree: Vec<i32>,
        lazy: Vec<i32>,
    }

    impl SegmentTree {
        pub fn new(n: usize) -> Self {
            let size = n * 4;
            SegmentTree {
                n,
                tree: vec![0; size],
                lazy: vec![0; size],
            }
        }

        fn propogate(&mut self, node: usize, l: usize, r: usize) {
            if self.lazy[node] != 0 {
                self.tree[node] = self.lazy[node];
                if l != r {
                    self.lazy[node * 2] = self.lazy[node];
                    self.lazy[node * 2 + 1] = self.lazy[node];
                }
                self.lazy[node] = 0;
            }
        }

        fn update_range_inner(
            &mut self,
            node: usize,
            l: usize,
            r: usize,
            ql: usize,
            qr: usize,
            value: i32,
        ) {
            self.propogate(node, l, r);

            // No overlap
            if r < ql || l > qr {
                return;
            }

            // total overlap
            if ql <= l && r <= qr {
                self.lazy[node] = value;
                self.propogate(node, l, r);
                return;
            }

            // partial overlap
            let mid = l + (r - l) / 2;
            self.update_range_inner(node * 2, l, mid, ql, qr, value);
            self.update_range_inner(node * 2 + 1, mid + 1, r, ql, qr, value);
        }

        pub fn update_range(&mut self, ql: usize, qr: usize, value: i32) {
            self.update_range_inner(1, 1, self.n, ql, qr, value);
        }

        fn get_leaf_at_position_inner(
            &mut self,
            node: usize,
            l: usize,
            r: usize,
            pos: usize,
        ) -> i32 {
            self.propogate(node, l, r);

            // No overlap
            if r < pos || l > pos {
                return 0;
            }

            // total overlap
            if pos == l && r == pos {
                return self.tree[node];
            }

            // partial overlap
            let mid = l + (r - l) / 2;

            // In our case we need to reach the leaf nodes, so the ans will only
            // be in 1 of the 2 children trees.
            if pos <= mid {
                self.get_leaf_at_position_inner(node * 2, l, mid, pos)
            } else {
                self.get_leaf_at_position_inner(node * 2 + 1, mid + 1, r, pos)
            }
        }

        pub fn get_leaf_at_position(&mut self, pos: usize) -> i32 {
            self.get_leaf_at_position_inner(1, 1, self.n, pos)
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    let mut out = BufWriter::new(io::stdout().lock());

    let nums: Vec<usize> = lines
        .next()
        .expect("Error reading line")
        .split_ascii_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    let n = nums[0];

    let mut st = trees::SegmentTree::new(n);
    for line in lines.rev() {
        let nums: Vec<i32> = line
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let l = nums[0];
        let r = nums[1];
        let w = nums[2];

        if l < w {
            st.update_range(l as usize, w as usize - 1, w);
        }

        if w < r {
            st.update_range(w as usize + 1, r as usize, w);
        }
    }

    (1..=n).for_each(|i| {
        write!(out, "{:?} ", st.get_leaf_at_position(i)).expect("Error writing answer");
    });

    writeln!(out).expect("Error writing newline");
    out.flush().ok();
}
