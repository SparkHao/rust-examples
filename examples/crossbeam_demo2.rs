use std::time::Duration;
use std::thread;
use crossbeam::channel::bounded;


fn main() {
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);

    let n_msgs = 4;
    let n_works = 2;

    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
            }

            drop(snd1);
        });

        for _ in 0..n_works {
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                for msg in recvr.iter() {
                    println!("Worker: {:?}, received: {:?}", thread::current().id(), msg);
                    sendr.send(msg * 2).unwrap();
                }
            });
        }

        for msg in rcv2.iter() {
            println!("Sink received: {:?}, {}", thread::current().id(), msg);
        }

        // for _ in 0..n_msgs {
        //     let msg = rcv2.recv().unwrap();
        //     println!("Sink received: {:?}, {}", thread::current().id(), msg);
        // }
    }).unwrap();
}