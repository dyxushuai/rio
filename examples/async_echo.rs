use std::{
    io,
    net::{TcpListener, TcpStream},
};

use futures::task::LocalSpawnExt;

async fn proxy(
    ring: &rio::Rio,
    a: &TcpStream,
    b: &TcpStream,
) -> io::Result<()> {
    let buf = vec![0_u8; 512];
    loop {
        let read_bytes = ring.read_at(a, &buf, 0).await?;
        if read_bytes == 0 {
            return Ok(());
        }

        let buf = &buf[..read_bytes];
        ring.write_at(b, &buf, 0).await?;
    }
}

async fn just_read(
    ring: Rc<rio::Rio>,
    a: &TcpStream,
) -> io::Result<()> {
    let buf = vec![0_u8; 8192];
    loop {
        let read_bytes = ring.read_at(a, &buf, 0).await?;
        if read_bytes == 0 {
            return Ok(());
        }
    }
}

use futures::task::LocalSpawn;
use std::rc::Rc;

fn main() {
    let mut cfg = rio::Config::default();
    cfg.sq_poll = true;
    let ring = Rc::new(cfg.start().unwrap());
    let acceptor =
        TcpListener::bind("127.0.0.1:50051").unwrap();
    println!("start");
    let mut executor = futures::executor::LocalPool::new();
    let spawner = executor.spawner();
    spawner.clone().spawn_local(async move {
        // kernel 5.5 and later support TCP accept
        loop {
            let stream =
                ring.accept(&acceptor).await.unwrap();
            let ring = ring.clone();

            spawner.clone().spawn_local(async move {
                match just_read(ring, &stream).await {
                    Ok(()) => {
                        eprintln!("client disconnected")
                    }
                    Err(e) => {
                        eprintln!("client failure: {}", e)
                    }
                }
            });
        }
    });

    executor.run();
}
