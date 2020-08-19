#![allow(dead_code, unused_variables)]

use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct BarData;

pub trait Ac {
    fn ac(&self) {
        println!("ac");
    }
}

pub struct BoxedAc {
    pub ac: Box<dyn Ac>,
    sender: Option<futures::channel::oneshot::Sender<()>>,
    receiver: Option<futures::channel::oneshot::Receiver<()>>,
}

impl BoxedAc {
    pub async fn run_forever(mut self) -> (Addr<BoxedAc>, futures::channel::oneshot::Receiver<()>) {
        let (tx, rx) = futures::channel::oneshot::channel::<()>();
        self.sender = Some(tx);
        println!("??");
        let addr = self.start();
        (addr, rx)
    }
    pub fn new(x: Box<dyn Ac>) -> BoxedAc {
        BoxedAc {
            ac: x,
            sender: None,
            receiver: None,
        }
    }
}

impl Actor for BoxedAc {
    type Context = Context<Self>;

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("we are stopped");
        let _ = self.sender.take().unwrap().send(());
    }
}

impl Handler<BarData> for BoxedAc {
    type Result = ();

    fn handle(&mut self, _: BarData, _: &mut Context<Self>) -> Self::Result {
        self.ac.ac();
    }
}

struct Test;

impl Ac for Test {}


#[actix_rt::main]
async fn main() {
    let ax = Test {};
    let mut x = BoxedAc::new(Box::new(ax));
    let (addr, x) = x.run_forever().await;
    x.await;
}