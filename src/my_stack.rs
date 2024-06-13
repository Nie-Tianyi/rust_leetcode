use std::slice::Iter;
use std::vec;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MyStack {
    val: Vec<i32>
}

impl MyStack {
    pub fn new() -> Self{
        MyStack{
            val: vec![],
        }
    }

    pub fn from_vec(vec: Vec<i32>) -> Self{
        MyStack{
            val: vec,
        }
    }

    pub fn top(&self) -> Option<&i32>{
        self.val.last()
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.val.pop()
    }

    pub fn push(&mut self, item: i32){
        self.val.push(item)
    }

    pub fn iter(&self) -> Iter<'_, i32> {
        self.val.iter()
    }

}

impl IntoIterator for MyStack{
    type Item = i32;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = self.val;
        vec.into_iter()
    }
}