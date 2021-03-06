use std::net::TcpListener;


pub struct Server {
    addr: String,
}
//Type Self and Server interchangeable
impl Server {
    //constructor
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Server listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            listener.accept();

        }
    }
}
