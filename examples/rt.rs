use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct BarData;

pub trait Ac {
    fn ac(&self) {
        println!("ac");
    }
}

pub struct BoxedAc(pub Box<dyn Ac>);


impl Actor for BoxedAc {
    type Context = Context<Self>;
}

impl Handler<BarData> for BoxedAc {
    type Result = ();

    fn handle(&mut self, _: BarData, _: &mut Context<Self>) -> Self::Result {
        self.0.ac();
    }
}

fn main() {
    struct Test;
    impl Ac for Test {



    }

    let address = BoxedAc(Box::new(Test)).start();

    address.send(BarData).await.unwrap();
}