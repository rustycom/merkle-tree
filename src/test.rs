#[cfg(test)]
mod tests {
    use crate::merkeltree::MerkelTree;

    #[test]
    fn get_index_works() {
        let result = MerkelTree::get_index(3,3);
        assert_eq!(result, 10);
    }

    #[test]
    fn get_depth_offset_works(){
        let result = MerkelTree::get_depth_and_offset(6);
        assert_eq!(result, (2, 3));
    }

    #[test]
    fn get_parrent_works(){
        let result = MerkelTree::get_parent_index(6);
        assert_eq!(result, 2);
    }


    #[test]
    fn get_merkel_tree(){
        let initial_value = String::from("0xab");
        let mut result = MerkelTree::new(2, initial_value);
        println!("{:?}", result);  
        
        let proof = MerkelTree::proof(result.clone(), 6); 
        println!("{:?}", proof.clone());   

        let leaf = String::from("0xab");
        let root = MerkelTree::verify(leaf, proof);
        print!("{}", root);  

        let leaf = String::from("0xcd");
        result = MerkelTree::set(2, 3, result, leaf);
        println!("{:?}", result);  
    }
}
