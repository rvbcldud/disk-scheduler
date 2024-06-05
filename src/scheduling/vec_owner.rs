use crate::Request;
pub trait VecOwner {
    fn get_vec(&mut self) -> &mut Vec<Request>;

    fn length(&mut self) -> usize { 
        self.get_vec().len()
    }
    fn remove(&mut self, index: usize) {
        self.get_vec().remove(index);
    }
    fn add(&mut self, req: Request) {
        self.get_vec().push(req);
    }
}