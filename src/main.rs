#[derive(Debug)]
enum OrderStatus {
    Paid,
    Cancelled,
    Pending,
}

#[derive(Debug)]
enum Event {
    Pay,
    Cancel,
}

struct SalesOrder<'a> {
    id: &'a str,
    status: OrderStatus,
}

impl<'a> SalesOrder<'a> {
    fn new(id: &'a str) -> Self {
        Self {
            id,
            status: OrderStatus::Paid,
        }
    }

    fn trigger(self, event: Event) -> Result<OrderStatus, String> {
        match (&self.status, &event) {
            (OrderStatus::Pending, Event::Pay) => Ok(OrderStatus::Paid),
            (OrderStatus::Pending, Event::Cancel) => Ok(OrderStatus::Cancelled),
            _ => Err(format!(
                "You can't {:?} when order's status is {:#?}",
                event, self.status
            )),
        }
    }
}

fn main() {
    let sales_order = SalesOrder::new("1234");
    println!("{:?}", sales_order.trigger(Event::Pay));
}
