use std::{
    cmp,
    io::{self, BufWriter, Read, Write},
};

#[derive(Debug)]
struct DSU {
    parent: Vec<usize>,
    rank: Vec<u32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
            rank: vec![0; n + 1],
        }
    }

    fn get_parent(&mut self, element: usize) -> usize {
        if self.parent[element] == element {
            return element;
        }

        let parent = self.get_parent(self.parent[element]);
        self.parent[element] = parent; // path compression
        return parent;
    }

    fn union(&mut self, element_1: usize, element_2: usize) {
        let parent_1 = self.get_parent(element_1);
        let parent_2 = self.get_parent(element_2);

        if self.rank[parent_1] > self.rank[parent_2] {
            self.parent[parent_2] = parent_1;
        } else if self.rank[parent_1] < self.rank[parent_2] {
            self.parent[parent_1] = parent_2;
        } else {
            // if both are equal, for consistency we'll make the
            // lower idx as parent.
            let low = cmp::min(parent_1, parent_2);
            let high = cmp::max(parent_1, parent_2);
            self.parent[high] = low;
            self.rank[low] += 1;
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

    let t: i64 = lines
        .next()
        .expect("Error reading t")
        .trim()
        .parse()
        .expect("Error in parsing t");

    for _ in 0..t {
        let arr: Vec<usize> = lines
            .next()
            .expect("Error going through n")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let n = arr[0];
        let f_edges_count = arr[1];
        let g_edges_count = arr[2];

        let mut f_edges: Vec<(usize, usize)> = Vec::with_capacity(f_edges_count);
        let mut g_edges: Vec<(usize, usize)> = Vec::with_capacity(g_edges_count);

        for _ in 0..f_edges_count {
            let edges: Vec<usize> = lines
                .next()
                .expect("Error going through n")
                .split_ascii_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();

            f_edges.push((edges[0], edges[1]));
        }

        let mut dsu_g = DSU::new(n);
        for _ in 0..g_edges_count {
            let edges: Vec<usize> = lines
                .next()
                .expect("Error going through n")
                .split_ascii_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();

            g_edges.push((edges[0], edges[1]));
            dsu_g.union(edges[0], edges[1]);
        }

        let mut result = 0;
        // now iterate over f edges and don't add those edges which aren't connected in g
        let mut dsu_f = DSU::new(n);
        for edge in f_edges {
            if dsu_g.get_parent(edge.0) != dsu_g.get_parent(edge.1) {
                result += 1;
            } else {
                dsu_f.union(edge.0, edge.1);
            }
        }

        // now for all the connected edges of g, check if they fall under same set/parent
        // in f union. Else incrase count and add them in same set to match g.
        for (edge_1, edge_2) in g_edges {
            if dsu_f.get_parent(edge_1) != dsu_f.get_parent(edge_2) {
                dsu_f.union(edge_1, edge_2);
                result += 1;
            }
        }

        writeln!(out, "{result}").expect("Error writing empty line");
    }

    out.flush().ok();
}
