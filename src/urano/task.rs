

#[derive(Clone,Debug)]
pub struct Task {
    pub id: usize,
    pub content: String,

}

impl Task {
    pub fn new(cont: String) -> Task {
        Task {
            id: 0,
            content: cont,
        }
    }

}