
pub struct Node<KeyT, ValT> {
    pub key: KeyT,
    pub value: ValT,
    pub priority: u64,
    pub left: Node<KeyT, ValT>,
    pub right: Node<KeyT, ValT>,
}

impl<KeyT, ValT> Node<KeyT, ValT> {
    pub fn new(key: KeyT, value: ValT) -> Node<KeyT, ValT> {
        Node {
            key,
            value,
            priority: 0,    // nodes will start with priority 0 since they've never been accessed before
            left: None,
            right: None,
        }
    }

    pub fn increment_priority(&mut self) {
        self.priority += 1;
    }
}