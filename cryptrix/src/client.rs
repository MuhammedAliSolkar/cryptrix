use message_io::node::{self, NodeEvent};
use message_io::network::{NetEvent, Transport, RemoteAddr };
use std::time::Duration;
use std::io;
use std::io::{Write, stdout};
use bincode;
use serde::{Serialize, Deserialize};

enum Signal {
    Greet,
}

pub fn run(remote_addr: RemoteAddr) {
    let (handler, listener) = node::split();
    let dot:[[i32; 3]; 3] = [[0, -3, -2], 
        [1, -4, -2],
        [-3, 4, 1]];
    let (server, _) = handler.network().connect(Transport::FramedTcp, remote_addr).unwrap();
    loop {
        print!("\n=> ");
        stdout().flush();
        let mut message = String::new();
        io::stdin().read_line(&mut message);
        let mut state = vec![vec![ 0i32; dot[0].len()]; message.len()/dot.len()+1];
        let mut count= 0;
        for ch in message.as_bytes().iter() {
            state[count/dot.len()][count%dot[0].len()] = *ch as i32;
            count +=1;
        }
        let mut result = vec![vec![0i32; dot.len()]; message.len()];
        for q in 0..state.len() {
            for w in 0..dot.len() {
                for colmn in 0..dot.len() {
                    result[q][w] += state[q][colmn]*dot[colmn][w]
                }
            }
        }
        let mut orig = vec![];
        for i in 0..state.len() {
            for x in 0..dot.len() {
                orig.push(result[i][x])
            }
        }
        // println!("\n{:?}\n", orig);
        let orig_u8: Vec<u8> = bincode::serialize(&orig).unwrap();
        handler.network().send(server, &orig_u8);
        handler.signals().send_with_timer(Signal::Greet, Duration::from_secs(1));
    }
}