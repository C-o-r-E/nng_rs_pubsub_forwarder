use nng::{
    Socket, RawSocket, Protocol, forwarder,
};

const FRONT_URL: &str = "tcp://localhost:7778";
const BACK_URL: &str = "tcp://localhost:7779";

fn main() {
    
    let front_end: Socket = Socket::new(Protocol::Sub0).unwrap();
    let back_end:  Socket = Socket::new(Protocol::Pub0).unwrap();

    let ret = front_end.listen(FRONT_URL);
    match ret {
        Err(x) => panic!("failed to listen on front end: {}", x),
        _ => ()
    }

    let ret = back_end.listen(BACK_URL);
    match ret {
        Err(x) => panic!("failed to listen on back end: {}", x),
        _ => ()
    }


    let ret = forwarder(
        RawSocket::try_from(front_end).unwrap(),
        RawSocket::try_from(back_end).unwrap()
    );

    match ret {
        Err(fw_err) => println!("Forwarder exited with err: {}", fw_err),
        _ => ()
    }

}