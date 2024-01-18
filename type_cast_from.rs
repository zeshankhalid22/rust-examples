#[derive(Debug)]
struct BoardIndex(usize);

impl BoardIndex{
    fn new() -> BoardIndex{
        BoardIndex(0)
    }
    // fn from_index(index: usize) -> BoardIndex{
    //     if index >= 3 * 3{
    //         panic!("Index out of bound");
    //     }
    //     BoardIndex(index)
    // }
}

impl From<usize> for BoardIndex{
    fn from(item: usize) -> Self{
        BoardIndex(item)
    }
}
impl Into<usize> for BoardIndex{
    fn into(self) -> usize{
        self.0
    }
}

fn main() {
    let index = 123;
    let index2 = 32;
    let bi = BoardIndex::from(index);
    let di:BoardIndex = index2.into();

    println!("{} {}",bi.0,di.0);

    let u2:usize = BoardIndex::new().into();
}
