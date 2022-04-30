use actix::prelude::*;

struct SumActor {}

impl Actor for SumActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Value(usize, usize);

impl Handler<Value> for SumActor {
    type Result = usize;

    fn handle(&mut self, msg: Value, _ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}

struct DisplayActor {}

impl Actor for DisplayActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct Display(usize);

impl Handler<Display> for DisplayActor {
    type Result = ();

    fn handle(&mut self, msg: Display, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Got {:?}", msg.0);
    }
}

fn main() {
    let system = System::new("single-arbiter-example");

    // 実行フローをfutureで定義
    let execution = async {
        // `Actor::start`でActorを現在のArbiterに生成 (この場合はSystem)
        let sum_addr = SumActor {}.start();
        let dis_addr = DisplayActor {}.start();

        // `Addr::send`ではFutureを実装しているため、awaitで受ける
        let sum_result = sum_addr.send(Value(6, 7)).await;

        match sum_result {
            Ok(res) => dis_addr.send(Display(res)).await.unwrap(),
            Err(e) => eprintln!("Encoutered mailbox error: {:?}", e),
        }
    };

    // 上のfutureを現在のArbiter (イベントループ) で生成
    Arbiter::spawn(execution);

    // Systemを停止させることで Arbiter -> Actor context -> Actor の順で停止する
    System::current().stop();

    system.run().unwrap();
}
