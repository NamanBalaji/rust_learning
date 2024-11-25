use std::collections::HashMap;
use std::net::{IpAddr, Shutdown, SocketAddr, TcpListener, TcpStream};
use std::result;
use std::io::{Read, Write};
use std::fmt;
use std::sync::Arc;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::time::{Duration, SystemTime};
use std::str;

type Result<T> = result::Result<T, ()>;

const SAFE_MODE: bool = true;
const BAN_LIMIT: Duration = Duration::from_secs(10*60);
const MESSAGE_RATE: Duration = Duration::from_secs(1);
const STRIKE_LIMIT: i32 = 10;
struct Sensitive<T>(T);

impl<T: fmt::Display> fmt::Display for Sensitive<T>  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        if SAFE_MODE {
            writeln!(f, "[REDACTED]")
        }else {
            inner.fmt(f)
        }
    }
}

enum Message {
    ClientConnected(Arc<TcpStream>),
    ClientDisconnected(SocketAddr),
    NewMessage {
        author_addr: SocketAddr,
        bytes: Vec<u8>,
    }
}

struct Client {
    conn: Arc<TcpStream>,
    last_message: SystemTime,
    strike_count: i32,
}

fn server(messages: Receiver<Message>) -> Result<()> {
    let mut clients: HashMap<SocketAddr, Client> = HashMap::new();
    let mut banned_mfs: HashMap<IpAddr, SystemTime> = HashMap::new();
    loop{
        let message = messages.recv().expect("The server reciever is not hung up");
        match message{
            Message::ClientConnected(author) => {
                let author_addr = author.peer_addr().expect("TODO: cache the peer address of the connection");
                let mut banned_at = banned_mfs.remove(&author_addr.ip());
                let now = SystemTime::now();

                banned_at = banned_at.and_then(|banned_at| {
                    let diff = now.duration_since(banned_at).expect("TODO: dont crash if the clock went backwards");
                    if diff >= BAN_LIMIT {
                        None
                    } else {
                        Some(banned_at)
                    }
                });

                if let Some(banned_at) = banned_at {
                    let diff = now.duration_since(banned_at).expect("TODO: dont crash if the clock went backwards");
                    banned_mfs.insert(author_addr.ip().clone(), banned_at);
                    let mut author = author.as_ref();
                    let _ = writeln!(author, "You are banned mf: {secs} secs left", secs = (BAN_LIMIT - diff).as_secs_f32());
                    let _ = author.shutdown(Shutdown::Both);
                } else {
                    clients.insert(author_addr.clone(), Client{
                        conn: author.clone(),
                        last_message: now,
                        strike_count: 0,
                    });
                }
                
            },
            Message::ClientDisconnected(author_addr) => {
                clients.remove(&author_addr);
            },
            Message::NewMessage{author_addr, bytes} => {
                if let Some(author) = clients.get_mut(&author_addr) {
                    let now = SystemTime::now();
                    let diff = now.duration_since(author.last_message).expect("TODO: dont crash if the clock went backwards");
                    if diff >= MESSAGE_RATE {
                        if let Ok(text) = str::from_utf8(&bytes) {
                            for(addr, client) in clients.iter(){
                                if *addr != author_addr {
                                    let _ = writeln!(client.conn.as_ref(), "{text}");
                                }
                            }
                        } else {
                            author.strike_count += 1;
                            if author.strike_count >= STRIKE_LIMIT {
                                banned_mfs.insert(author_addr.ip().clone(), now);
                                let _ = writeln!(author.conn.as_ref(), "You are banned MF");
                                let _ = author.conn.shutdown(Shutdown::Both);
                            } 
                        }
                    } else {
                        author.strike_count += 1;
                        if author.strike_count >= STRIKE_LIMIT {
                            banned_mfs.insert(author_addr.ip().clone(), now);
                            let _ = writeln!(author.conn.as_ref(), "You are banned MF");
                            let _ = author.conn.shutdown(Shutdown::Both);
                        } 
                    }
                   
                }
            },
        }
    }
}

fn client(stream: Arc<TcpStream>, messages: Sender<Message>) -> Result<()>{
    let author_addr = stream.peer_addr().map_err(|err| {
        eprintln!("ERROR: could not get peer address: {err}")
    })?;
    messages.send(Message::ClientConnected(stream.clone())).map_err(|err| {
        eprintln!("ERROR: could not send message to the server thread: {err}")
    })?;
    let mut buffer = Vec::new();
    buffer.resize(64, 0);

    loop {
        let n = stream.as_ref().read(&mut buffer).map_err(|err| {
            eprintln!("ERROR: could not read message from client: {err}");
            let _ = messages.send(Message::ClientDisconnected(author_addr));
        })?;
        if n > 0 {
            let bytes = buffer[0..n].to_vec();
            messages.send(Message::NewMessage{
                bytes,
                author_addr,
            }).map_err(|err| {
                eprintln!("ERROR: could not send message to the server thread: {err}")
            })?;
        } else {
            let _ = messages.send(Message::ClientDisconnected(author_addr)).map_err(|err| {
                eprintln!("ERROR: could not send message to the server thread: {err}")
            });
            break;
        }
    }
    Ok(())
} 

fn main() -> Result<()> {
    let address = "127.0.0.1:6969";
    let listener = TcpListener::bind(address).map_err(|err| {
        eprintln!("ERROR: could not bind {address}: {}", Sensitive(err))
    })?;
    println!("INFO: listening at {address}");

    let (message_sender, message_receiver) = channel();

    thread::spawn(|| server(message_receiver));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = Arc::new(stream);
                let message_sender = message_sender.clone();
                thread::spawn(|| client(stream, message_sender));
            }
            Err(err) => {
                eprintln!("ERROR: could not accept connection: {}", Sensitive(err));
            }
        }
    }
    Ok(())
}