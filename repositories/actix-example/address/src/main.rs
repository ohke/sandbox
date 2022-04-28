use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
struct OrderShipped(usize);

#[derive(Message)]
#[rtype(result = "()")]
struct Ship(usize);

#[derive(Message)]
#[rtype(result = "()")]
struct Subscribe(pub Recipient<OrderShipped>);

struct OrderEvents {
    subscribers: Vec<Recipient<OrderShipped>>,
}

impl OrderEvents {
    fn new() -> Self {
        OrderEvents {
            subscribers: vec![],
        }
    }
}

impl Actor for OrderEvents {
    type Context = Context<Self>;
}

impl OrderEvents {
    fn notify(&mut self, order_id: usize) {
        for subscr in &self.subscribers {
            subscr.do_send(OrderShipped(order_id)).unwrap();
        }
    }
}

impl Handler<Subscribe> for OrderEvents {
    type Result = ();

    fn handle(
        &mut self,
        msg: Subscribe,
        _: &mut <Self as actix::Actor>::Context,
    ) -> <Self as actix::Handler<Subscribe>>::Result {
        self.subscribers.push(msg.0);
    }
}

impl Handler<Ship> for OrderEvents {
    type Result = ();

    fn handle(
        &mut self,
        msg: Ship,
        _: &mut <Self as actix::Actor>::Context,
    ) -> <Self as actix::Handler<Ship>>::Result {
        self.notify(msg.0);
        System::current().stop();
    }
}

struct EmailSubscriber;

impl Actor for EmailSubscriber {
    type Context = Context<Self>;
}

impl Handler<OrderShipped> for EmailSubscriber {
    type Result = ();

    fn handle(
        &mut self,
        msg: OrderShipped,
        _: &mut <Self as actix::Actor>::Context,
    ) -> <Self as actix::Handler<OrderShipped>>::Result {
        println!("Email sent for order {}", msg.0)
    }
}

struct SmsSubscriber;

impl Actor for SmsSubscriber {
    type Context = Context<Self>;
}

impl Handler<OrderShipped> for SmsSubscriber {
    type Result = ();

    fn handle(
        &mut self,
        msg: OrderShipped,
        _: &mut <Self as actix::Actor>::Context,
    ) -> <Self as actix::Handler<OrderShipped>>::Result {
        println!("SMS sent for order {}", msg.0)
    }
}

fn main() {
    // 複数のサブスクライバに送信する
    let system = System::new("events");

    // サブスクライバの登録
    //   -- (Subscribe) -> OrderEvents
    let email_subscriber = Subscribe(EmailSubscriber {}.start().recipient());
    let sms_subscriber = Subscribe(SmsSubscriber {}.start().recipient());
    let order_event = OrderEvents::new().start();

    // サブスクライバへの送信
    //   -- (Ship) -> OrderEvents -- (OrderShipped) --> EmailSubscriber, SmsSubscriber
    order_event.do_send(email_subscriber);
    order_event.do_send(sms_subscriber);
    order_event.do_send(Ship(1));

    system.run().unwrap();
}
