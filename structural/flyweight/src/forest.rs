use std::{collections::HashSet, rc::Rc};

use draw::Canvas;

use crate::forest::tree::{Tree, TreeKind};

pub use self::tree::TreeColor;

mod tree;

/// Forest implements an internal cache that is hidden behind the public API.
///
/// The point is having an opaque cache implementation. It can use a hash set,
/// FIFO, or even a simple vector.
///
/// Here are the key points:
/// - `cache` is of `HashSet` type, so it can hold only a single
///    instance of a `TreeKind`,
/// - `Rc` is needed to get the reference on the tree kind without
///    cloning the full structure,
/// - `TreeKind` must derive `Eq`, `PartialEq`, and `Hash` traits to be
///    used in the `HashSet`.
#[derive(Default)]
pub struct Forest {
    cache: HashSet<Rc<TreeKind>>,
    trees: Vec<Tree>,
}

impl Forest {
    pub fn plant_tree(&mut self, x: u32, y: u32, core: TreeColor, name: String, data: String) {
        let tree_kind = TreeKind::new(core, name, data);
        self.cache.insert(Rc::new(tree_kind.clone()));

        let tree = Tree::new(x, y, self.cache.get(&tree_kind).unwrap().clone());

        self.trees.push(tree);
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for tree in &self.trees {
            tree.draw(canvas);
        }
    }

    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }
}