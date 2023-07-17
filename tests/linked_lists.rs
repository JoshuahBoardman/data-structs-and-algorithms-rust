use data_structs_and_algorithms_rust::data_structs::{linked_list, LinkedList, Node};

#[test]
fn test_linked_list() -> () {
    let list: LinkedList<i32> = linked_list();
    // TODO: finish creating the tests
    list.append(5);
    list.append(7);
    list.append(9);
}
