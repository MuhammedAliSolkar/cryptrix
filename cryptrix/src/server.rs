use message_io::node::{self};
use message_io::network::{NetEvent, Transport, Endpoint};
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr};
use bincode;

pub fn run(addr: SocketAddr) {
    let (handler, listener) = node::split::<()>();
    let dot_inv: [[i32; 3]; 3] = [[4, -5, -2],
    [5, -6, -2],
    [-8, 9, 3]];

    handler.network().listen(Transport::FramedTcp, addr).unwrap();

    listener.for_each(move |event| match event.network() {
        NetEvent::Connected(_, _) => unreachable!(), 
        NetEvent::Accepted(_endpoint, _listener) => println!("Client connected"), 
        NetEvent::Message(endpoint, data) => {
            let data_deser: Vec<i32> = bincode::deserialize(&data).unwrap();
            let mut data_vec = vec![vec![0i32; dot_inv.len()]; data_deser.len()/dot_inv.len()];
            for i in 0..data_deser.len() {
                data_vec[i/dot_inv.len()][i%dot_inv.len()] = data_deser[i]
            }
            let mut result = vec![vec![0i32; dot_inv.len()]; data_vec.len()];
            // println!("{:?}", data_vec);
            for q in 0..data_vec.len() {
                for w in 0..dot_inv.len() {
                    for colmn in 0..dot_inv.len() {
                        result[q][w] += data_vec[q][colmn]*dot_inv[colmn][w]
                    }
                }
            }
            println!("Received: {:?}", result);
            let mut message = String::new();
            for h in result.iter() {
                for j in h.iter() {
                    unsafe {message.push(std::char::from_u32_unchecked((*j).try_into().unwrap()))};
                }
             }
             print!("\nDecrypted message: {}", message.trim_matches(char::from(0)));
        },
        NetEvent::Disconnected(_endpoint) => println!("Client disconnected"), 
    });
}