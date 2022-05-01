use actix::prelude::*;

struct MySyncActor;

#[derive(Message)]
#[rtype(result = "usize")]
struct Value(usize, usize);

impl Actor for MySyncActor {
    // CPUバウンドなワークロードなど向けに、SyncArbiterでは (Arbiterのスレッド上ではなく) OSのスレッドプールでActorの複数のインスタンスを起動する。
    // SyncArbiterを使う場合、ContextではなくSyncContextにする。
    type Context = SyncContext<Self>;
}

impl Handler<Value> for MySyncActor {
    type Result = usize;

    fn handle(&mut self, msg: Value, _ctx: &mut SyncContext<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}

#[actix_rt::main]
async fn main() {
    // スレッド数を指定する (変更不可)
    let addr = SyncArbiter::start(2, || MySyncActor);

    // あとはArbiterの場合と使い方は同じ
    let res = addr.send(Value(6, 7)).await;

    println!("result: {}", res.unwrap());
}
