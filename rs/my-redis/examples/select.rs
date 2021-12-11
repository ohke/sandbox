use tokio::sync::{mpsc, oneshot};

async fn some_operation() -> String {
    "done".to_string()
}

async fn computation1() -> String {
    "computation1 done".to_string()
}

async fn computation2() -> String {
    "computation2 done".to_string()
}

#[tokio::main]
async fn main() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                // oneshot::ReceiverのDropでは、Sender側へクローズ通知を送信する。
                // some_operationはキャンセルされて、tx1はドロップ。
            }
        }
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    // 2つのchannelをawaitして、返り値をvalへ束縛する。
    // 完了しなかった方はドロップ。
    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }

    // 全分岐のhandlerは同じ型で評価されないといけない
    let out = tokio::select! {
        res1 = computation1() => res1,
        res2 = computation2() => res2,
    };

    println!("Got = {}", out);

    let (mut tx1, mut rx1) = mpsc::channel(128);
    let (mut tx2, mut rx2) = mpsc::channel(128);

    tokio::spawn(async move {
        tx1.send("tx1 send".to_string());
        tx2.send("tx2 send".to_string());
    });

    tokio::select! {
        Some(v) = rx1.recv() => {println!("Got {:?} from rx1", v);}
        Some(v) = rx2.recv() => {println!("Got {:?} from rx2", v);}
        // チャネルがクローズされた場合、recvはNoneを返すため、elseが評価される
        else => {println!("Both channels closed");}
    };

    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    let mut out = String::new();

    tokio::spawn(async move {
        tx1.send("tx1 send".to_string());
        tx2.send("tx2 send".to_string());
    });

    // どれか1つだけが実行されることを保証する (= 同一タスク上ですべての分岐を並行に実行する) ので、各ハンドラでoutの可変参照も取れる
    tokio::select! {
        _ = rx1 => {
            out.push_str("rx1 completed");
        }
        _ = rx2 => {
            out.push_str("rx2 completed");
        }
    }

    println!("{}", out);
}
