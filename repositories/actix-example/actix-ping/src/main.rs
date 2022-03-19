use actix::prelude::*;

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

// `Message` trait is to defaine a result type.
// Ping message needs to return usize value.
#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

// `Handler<Ping>` trait to accept Ping hand handle it.
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Self::Context) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

#[actix_rt::main]
async fn main() {
    // Create and start `Actor`.
    let addr = MyActor { count: 10 }.start();

    // Send message through `Addr` struct and get result.
    let res = addr.send(Ping(10)).await;

    println!("Result: {}", res.unwrap() == 20);

    // Stop runtime.
    System::current().stop();
}
