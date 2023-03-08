use std::{path::Path, fmt::Error, io::{BufReader, Read}, fs::File, sync::mpsc::channel};

use ring::digest::{Digest, Context, SHA256};
use threadpool::ThreadPool;

fn main(){
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    for entry in walkdir::WalkDir::new("./")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir()) {
            let path = entry.path().to_owned();
            let tx = tx.clone();
            pool.execute(move || {
                let digest = compute_digest(path);
                tx.send(digest).expect("Could not send data!");
            });
        }
    
    drop(tx);
    for t in rx.iter() {
        let (sha, path) = t.unwrap();
        println!("{:?} {:?}", sha, path);
    }
}

fn compute_digest<P: AsRef<Path>>(file_path: P) -> Result<(Digest, P), Error> {
    let mut buf_reader = BufReader::new(File::open(&file_path).unwrap());
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];
    loop {
        let count = buf_reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok((context.finish(), file_path))
}