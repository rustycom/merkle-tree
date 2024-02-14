use sha3::{Digest, Sha3_256};

#[derive(Clone, Default, Debug)]
pub struct MerkelTree {
    layers: Vec<Vec<String>>,
}

pub fn hash_string_sha3(input: String) -> String {
    // Create a new SHA-3 hasher
    let mut hasher = Sha3_256::new();
    // Update the hasher with the input string
    hasher.update(input.as_bytes());
    // Finalize the hash and obtain the result as a fixed-size array
    let result = hasher.finalize();
    // Convert the result to a hexadecimal string
    let hex_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
    hex_string
}


pub fn get_index(depth:u32, offset:u32) -> u32 {
    let mut index = 0;
    if depth > 0 {
        index = 2u32.pow(depth) + offset - 1;
    }
    index
}

pub fn get_depth_and_offset(index:u32) -> (u32, u32) {
    let depth = (index + 1).ilog2();
    let offset = index + 1 - 2u32.pow(depth);
    (depth , offset)
}

pub fn get_parent_index(index: u32) -> u32 {
    let (depth , offset) = get_depth_and_offset(index);
    return get_index(depth - 1, offset/2);
}

pub fn get_left_child(index: u32) ->u32 {
    let (depth , offset) = get_depth_and_offset(index);
    return get_index(depth + 1, offset *2); 
}

pub fn merkel_tree(depth:u32, initial_leaf: String) -> MerkelTree {
    let mut node = hash_string_sha3(initial_leaf);
    let mut tree: MerkelTree = Default::default();
    let n = depth;
    let mut layer: Vec<String> = vec![];
    for _i in 0..2u32.pow(n){
        layer.push(node.clone());
    }
    tree.layers.push(layer);
    for i in (0..n).rev() {
        let mut layer: Vec<String> = vec![];
        let str = format!("{}{}", node, node);
        let leaf = hash_string_sha3(str);
        node = leaf;
        for _m in 0..2u32.pow(i){
            layer.push(node.clone());
        }
        tree.layers.push(layer); 
    }
    tree
}

pub fn set(depth:u32, offset:u32, mtree:MerkelTree, leaf:String) -> MerkelTree{
    let mut tree = mtree;
    let mut recacl_node = hash_string_sha3(leaf);
    let mut child_offset = offset;
    for n in (0..(depth+1)).rev() {
        tree.layers[(depth-n) as usize][child_offset as usize] = recacl_node.clone();
        if n > 0 {
            let index = get_index(depth, child_offset);
            let parent_index = get_parent_index(index);
            let (_d1, o1) = get_depth_and_offset(parent_index);
            child_offset = o1;
            let str = format!("{}{}", tree.layers[(depth - n)as usize][(o1*2) as usize], tree.layers[(depth - n)as usize][(o1*2 + 1) as usize] );
            let new_node = hash_string_sha3(str);
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
        let parent_index = get_parent_index(leaf_index);
        let left_child_index = get_left_child(parent_index);
        let (_depth, offset) = get_depth_and_offset(left_child_index);
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
    let mut initial_leaf =  hash_string_sha3(leaf);
    let len = proof.len();
    for i in 0..len {
        let str = format!("{}{}", proof[i], proof[i]);
        let node = hash_string_sha3(str);
        initial_leaf = node;
    }

    initial_leaf

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_index_works() {
        let result = get_index(3,3);
        assert_eq!(result, 10);
    }

    #[test]
    fn get_depth_offset_works(){
        let result = get_depth_and_offset(6);
        assert_eq!(result, (2, 3));
    }

    #[test]
    fn get_parrent_works(){
        let result = get_parent_index(6);
        assert_eq!(result, 2);
    }

    #[test]
    fn get_left_child_works(){
        let result = get_left_child(5);
        assert_eq!(result, 11);
    }

    #[test]
    fn get_merkel_tree(){
        let initial_value = String::from("0xab");
        let mut result = merkel_tree(2, initial_value);
        println!("{:?}", result);  
        
        let proof = proof(result.clone(), 6); 
        println!("{:?}", proof.clone());   

        let leaf = String::from("0xab");
        let root = verify(leaf, proof);
        print!("{}", root);  

        let leaf = String::from("0xcd");
        result = set(2, 3, result, leaf);
        println!("{:?}", result);  
    }
}
