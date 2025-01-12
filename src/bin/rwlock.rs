// cargo run --bin rwlock

// リーダーライターロックをを提供するプリミティブ
// このプリミティブは、複数のリーダーが同時にデータにアクセスできるようにする一方で、
// ライターがデータにアクセスする際には排他制御を行うことができる。

// おy見取りスレッドが存在する間は、書き込みアクセスはブロックされる。

use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(0));

    // 複数のリーダースレッドを生成
    let mut readers = vec![];

    for _ in 0..10 {
        // Arc::clone で Arc の参照カウントを増やす。
        let data = Arc::clone(&data);

        // スレッドを作成して、読み込みロックを取得する。
        readers.push(thread::spawn(move || {
            let read_guard = data.read().unwrap();
            println!("Reader: {}", *read_guard);
        }))
    }

    // ライタースレッドを生成
    let writer = {
        let data_clone = Arc::clone(&data);
        thread::spawn(move || {
            println!("Writer: Trying to write");
            let mut write_guard = data_clone.write().unwrap();
            *write_guard = 100;
            println!("Writer: {}", *write_guard);
        })
    };

    for reader in readers {
        reader.join().unwrap();
    }
    writer.join().unwrap();

    // デッドロックの例
    let t = thread::spawn(move || {
        let d = *data.read().unwrap();
        let _ = d; // ここでdのライフタイムを終了すると、書き込みが消えるため、デッドロックが起きない
        *data.write().unwrap() = 100;
        println!("Data: {}", d);
    });

    t.join().unwrap();
}
