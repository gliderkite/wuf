//! This is the documentation for `wuf`.
//! 
//! A fast weighted implementation to the union-find problem with path
//! compression.
//! 
//! # Examples
//! ```
//! let n_nodes = 10;
//! let mut graph = wuf::Graph::new(n_nodes);
//! let node_id1 = 0;
//! let node_id2 = 1;
//! if !graph.connected(node_id1, node_id2) {
//!     graph.connect(node_id1, node_id2);
//! }
//! ```

pub struct Graph {
    nodes: Vec<usize>,  // list of nodes' ids.
    sizes: Vec<usize>   // number of nodes in the tree which root is sizes[i]
}

impl Graph {

    /// Returns a new Graph with the given number of nodes.
    /// 
    /// # Arguments
    /// * `n` Number of nodes belonging to the graph.
    /// 
    pub fn new(n: usize) -> Graph {
        Graph {
            nodes: (0..n).map(|x| x).collect(),
            sizes: vec![0; n]
        }
    }

    /// Returns the number of nodes.
    /// 
    /// # Example
    /// ```
    /// let graph = wuf::Graph::new(10);
    /// println!("Number of nodes: {}", graph.count());
    /// ```
    pub fn count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true only if the two given nodes are connected,
    /// otherwise returns false.
    /// 
    /// # Arguments
    /// * `a` ID of the first node.
    /// * `b` ID of the second node.
    /// 
    /// # Example
    /// ```
    /// let mut graph = wuf::Graph::new(10);
    /// let node1 = 0;
    /// let node2 = 1;
    /// println!("Are nodes connected? {}", graph.connected(node1, node2));
    /// ```
    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        // check if the two nodes have the same root
        self.root(a) == self.root(b)
    }

    /// Connects the two given nodes.
    /// 
    /// # Arguments
    /// * `a` ID of the first node.
    /// * `b` ID of the second node.
    /// 
    /// # Example
    /// ```
    /// let mut graph = wuf::Graph::new(10);
    /// let node1 = 0;
    /// let node2 = 1;
    /// graph.connect(node1, node2);
    /// ```
    pub fn connect(&mut self, a: usize, b: usize) {
        let a_root = self.root(a);
        let b_root = self.root(b);
        if a_root == b_root {
            // already connected
            return;
        }
        // balance by linking root of smaller tree to root of larger tree
        if self.sizes[a_root] < self.sizes[b_root] {
            self.nodes[a_root] = b_root;
            self.sizes[b_root] += self.sizes[a_root];
        } else {
            self.nodes[b_root] = a_root;
            self.sizes[a_root] += self.sizes[b_root];
        }
    }

    /// Returns the root of the given node.
    /// 
    /// # Arguments
    /// * `id` ID of the child node.
    fn root(&mut self, id: usize) -> usize {
        let mut root = id;
        while root != self.nodes[id] {
            // make every other node in path point to its grandparent
            self.nodes[root] = self.nodes[self.nodes[root]];
            root = self.nodes[id];
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_count() {
        let n = 10;
        let graph = Graph::new(n);
        assert_eq!(n, graph.count());
    }

    #[test]
    fn should_connect() {
        let mut graph = Graph::new(10);
        assert!(graph.connected(0, 0));
        assert!(!graph.connected(0, 1));
        graph.connect(0, 1);
        assert!(graph.connected(0, 1));
    }
}
