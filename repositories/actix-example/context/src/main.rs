use actix::prelude::*;

#[derive(Debug)]
struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // configure mailbox parameters.
        ctx.set_mailbox_capacity(1); // default: 16
    }
}

// `Message` trait is to defaine a result type.
// Ping message needs to return usize value.
#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

// `Handler<Ping>` trait to accept Ping hand handle it.
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        self.count += msg.0;

        // stopping me.
        if self.count > 0 {
            println!("Shutting down ping receiver.");
            ctx.stop();
        }

        self.count
    }
}

struct WhoAmI;

impl Message for WhoAmI {
    type Result = Result<actix::Addr<MyActor>, ()>;
}

impl Handler<WhoAmI> for MyActor {
    type Result = Result<actix::Addr<MyActor>, ()>;

    fn handle(&mut self, _msg: WhoAmI, ctx: &mut Context<Self>) -> Self::Result {
        // response my address.
        Ok(ctx.address())
    }
}

#[actix_rt::main]
async fn main() {
    // Create and start `Actor`.
    let addr = MyActor { count: 10 }.start();

    let res = addr.send(WhoAmI).await;
    println!("WhoAmI: {:?}", res.unwrap());

    let addr_2 = addr.clone();
    let res = addr.send(Ping(6)).await;
    match res {
        Ok(_) => {
            // actor has already stopped.
            assert!(addr_2.try_send(Ping(6)).is_err());
        }
        _ => panic!(),
    }

    // Stop runtime.
    System::current().stop();
}
