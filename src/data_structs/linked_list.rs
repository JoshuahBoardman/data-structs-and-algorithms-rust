// TODO: Specify the life time for the structs and methods
pub struct Node<T> {
    pub value: T,
    pub prev: Option<Box<Node<T>>>,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    length: i32,
    head: Option<Node<T>>,
    tail: Option<Node<T>>,
}

impl<T> LinkedList<T> {
    // Getters
    pub fn length(&self) -> &i32 {
        &self.length
    }

    pub fn head(&self) -> &Option<Node<T>> {
        &self.head
    }

    pub fn tail(&self) -> &Option<Node<T>> {
        &self.tail
    }

    pub fn get(&self, index: i32) -> &Node<T> {
        // Walk the list to find index
    }

    // Setters
    pub fn append(self: &mut Self, node: Node<T>) {
        println!("new tail");
    }

    pub fn prepend(self: &mut Self) {
        println!("new head");
    }

    // Remover function
    pub fn remove<T>(&self) -> Option<T> {
        // remove last element and return the  value
    }

    pub fn removeAt<T>(&self, index: i32) -> Option<T> {
        // use the get method to walk the list and then remove the specified value
    }
}

pub fn linked_list<T>() -> LinkedList<T> {
    LinkedList {
        length: 0,
        head: None,
        tail: None,
    }
}
