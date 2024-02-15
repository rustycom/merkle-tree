use crate::utils::hash_string_sha3;

#[derive(Clone, Default, Debug)]
pub struct MerkelTree {
    layers: Vec<Vec<String>>,
}

impl MerkelTree {
    pub fn new(depth:u32, initial_leaf: String) -> MerkelTree {
        Self::merkel_tree(depth, initial_leaf)
    }

    pub fn get_index(depth:u32, offset:u32) -> u32 {
        if depth > 0 {
            (1 << depth) + offset - 1
        }else {
            0
        }
    }

    pub fn get_depth_and_offset(index:u32) -> (u32, u32) {
        let depth = (index + 1).ilog2();
        let offset = index + 1 - (1 << depth);
        (depth , offset)
    }
    
    pub fn get_parent_index(index: u32) -> u32 {
        let (depth , offset) = Self::get_depth_and_offset(index);
        Self::get_index(depth - 1, offset/2)
    }

    fn get_left_child(index: u32) ->u32 {
        let (depth , offset) = Self::get_depth_and_offset(index);
        Self::get_index(depth + 1, offset *2)
    }
    
    fn merkel_tree(depth:u32, initial_leaf: String) -> MerkelTree {
        let mut node = hash_string_sha3(&initial_leaf);
        let mut tree: MerkelTree = MerkelTree::default();
        let layer: Vec<String> = vec![node.clone(); 1 << depth as usize];
        tree.layers.push(layer);
        for i in (0..depth).rev() {
            node = hash_string_sha3(&format!("{}{}", node, node));
            let layer: Vec<String> = vec![node.clone(); 1 << i as usize];
            tree.layers.push(layer);
        }
        tree
    }
    
    pub fn set(depth:u32, offset:u32, mtree:MerkelTree, leaf:String) -> MerkelTree{
        let mut tree = mtree;
        let mut recacl_node = hash_string_sha3(&leaf);
        let mut child_offset = offset;
        for n in (0..(depth+1)).rev() {
            tree.layers[(depth-n) as usize][child_offset as usize] = recacl_node.clone();
            if n > 0 {
                let index = Self::get_index(depth, child_offset);
                let parent_index = Self::get_parent_index(index);
                let (_d1, o1) = Self::get_depth_and_offset(parent_index);
                child_offset = o1;
                let str = format!("{}{}", tree.layers[(depth - n)as usize][(o1*2) as usize], tree.layers[(depth - n)as usize][(o1*2 + 1) as usize] );
                let new_node = hash_string_sha3(&str);
                recacl_node = new_node;
            }
        }
        tree
    }
    
    pub fn proof(tree:MerkelTree, leaf:u32) -> Vec<String> {
        let mut proof = vec![];
        let n = tree.layers.len() - 1;
        let mut leaf_index = leaf;
        for i in 0..n{
            let parent_index = Self::get_parent_index(leaf_index);
            let left_child_index = Self::get_left_child(parent_index);
            let (_depth, offset) = Self::get_depth_and_offset(left_child_index);
            if left_child_index == leaf_index {
                proof.push(tree.layers[i][(offset + 1) as usize].clone());
                leaf_index = parent_index;
            }else {
                proof.push(tree.layers[i][(offset) as usize].clone());
                leaf_index = parent_index;
            }
        }
        proof
    }
    
    pub fn verify(leaf: String, proof: Vec<String>) -> String{
        let mut initial_leaf =  hash_string_sha3(&leaf);
        let len = proof.len();
        for i in 0..len {
            let str = format!("{}{}", proof[i], proof[i]);
            let node = hash_string_sha3(&str);
            initial_leaf = node;
        }
        initial_leaf
    }
}