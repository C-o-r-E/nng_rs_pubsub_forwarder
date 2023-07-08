use nng::{
    RawSocket, Protocol, forwarder,
};

const FRONT_URL: &str = "tcp://localhost:7778";
const BACK_URL: &str = "tcp://localhost:7779";

fn main() {
    
    let front_end: RawSocket = RawSocket::new(Protocol::Sub0).unwrap();
    let back_end:  RawSocket = RawSocket::new(Protocol::Pub0).unwrap();

    let ret = front_end.socket.listen(FRONT_URL);
    if let Err(x) = ret { panic!("failed to listen on front end: {}", x) }

    let ret = back_end.socket.listen(BACK_URL);
    if let Err(x) = ret { panic!("failed to listen on back end: {}", x) }


    let ret = forwarder(
        front_end,
        back_end
    );

    if let Err(fw_err) = ret { println!("Forwarder exited with err: {}", fw_err) }

}