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
    fn add_vec(&mut self, requests:&Vec<Request>) {
        for (_, &request) in requests.iter().enumerate() {
        // for i in 0..requests.len() {
            self.add(request);
        }

        // for request in requests {
        //     self.add(request);
        // }
    }
}