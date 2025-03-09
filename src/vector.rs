use std::slice::Iter;

pub struct MyVec<T> {
    vec: Vec<T>
}

impl<T> MyVec<T> {

    //create empty new collection
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    //add item to end of collection
    pub fn push(&mut self, item: T) {
        self.vec.push(item);
    }

    //get length of collection
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.vec.iter()
    }
}

impl<T> FromIterator<T> for MyVec<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut item = MyVec::new();
        for i in iter {
            item.push(i);
        }
        item
    }
}