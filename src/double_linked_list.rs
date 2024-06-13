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

    /// 从给定位置插入一个值
    pub fn insert(&mut self, index: usize, val: i32) -> Result<(),&str> {
        if index > self.len{
            return Err("invalid index")
        }
        // 初始化指向头节点的指针
        let mut current_node = &mut self.head_node;
        // 移动指针
        for _ in 0..index {
            current_node = current_node.next_node.as_mut().unwrap();
        }
        let new_node = Node {
            prev_node: Some(Box::new(current_node)),
            next_node: current_node.next_node.take(),
            val,
        };
        current_node.next_node = Some(Box::new(new_node));
        Ok(())
    }

    pub fn delete(&mut self, index: usize, val: i32) -> Result<(),&str>  {
        unimplemented!()
    }
}