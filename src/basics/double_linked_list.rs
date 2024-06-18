#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    prev_node: Option<Box<Node>>,
    next_node: Option<Box<Node>>,
    val: i32,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DoubleLinkedList{
    head_node: Node, // 记录头节点
    len: usize, // 记录长度
}

impl DoubleLinkedList {
    /// 从给定值新建一个双向链表
    pub fn new(val: i32) -> Self {
        let head_node = Node {
            prev_node: None,
            next_node: None,
            val,
        };
        DoubleLinkedList{
            head_node,
            len: 1_usize,
        }
    }

    pub fn delete(&mut self, index: usize, val: i32) -> Result<(),&str>  {
        unimplemented!()
    }
}