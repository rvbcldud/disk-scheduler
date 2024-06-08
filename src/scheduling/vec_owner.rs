use crate::Request;

use super::request;
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
    fn remove_duplicates(&mut self) {

        let vec = self.get_vec();
    
        // First, sort the vector by location to ensure dedup_by_key works correctly
        vec.sort_by(|req1, req2| req1.location.cmp(&req2.location));

        // Then, remove duplicates by location
        vec.dedup_by_key(|req| req.location);

        // self.get_vec().de

        // let mut remove: Vec<usize> = Vec::new();

        // let vec = self.get_vec();

        // for (i, &req1) in vec.iter().enumerate() {
        //     for (j, &req2) in vec.iter().enumerate() {
        //         if req1.location == req2.location && i != j {
        //             println!("i: {}, j: {}", i, j);
        //             remove.push(j);
        //         }
        //     }
        // }

        // for (_, &index) in remove.iter().enumerate() {
        //     vec.remove(index);
        // }

    }
    // fn remove_duplicates(&mut self) {
    //     // self.get_vec()
    //     //     .into_iter()
    //     //     .filter(
    //     //             |&req|
    //     //             self.contains_location(req.location)
    //     //     );

    //     // let vec = self.get_vec();
    //     // vec.retain(|req| !self.contains_location(req.location));
    // }
}