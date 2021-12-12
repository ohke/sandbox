use mini_redis::client;
use tokio_stream::StreamExt;

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // numbersチャネルへパブリッシュ
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;

    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    // numbersチャネルをサブスクライブ
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    // subscriberを消費するストリームを生成
    // - filterで長さ1のメッセージに限定
    // - mapで中身を取り出す
    // - takeで最大3メッセージのみ受信
    let messages = subscriber
        .into_stream()
        .filter(|msg| match msg {
            Ok(msg) if msg.content.len() == 1 => true,
            _ => false,
        })
        .map(|msg| msg.unwrap().content)
        .take(3);

    // Streamのnextを呼ぶためには、pinされている必要がある
    // pinされている = 値がメモリ上でそれ以上ムーブされない (pinされたデータのポインタは有効であることを確信できる)
    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(&[1, 2, 3]);

    // Noneならstreamのイテレーションが終了した
    while let Some(v) = stream.next().await {
        println!("GOT = {:?}", v);
    }

    tokio::spawn(async { publish().await });

    subscribe().await.unwrap();

    // サーバが立ち上がっている限り、終了しない
    println!("DONE");
}
