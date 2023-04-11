use std::{collections::HashMap, hash::Hash};

pub trait Subscriber {
    fn on_event(&self, event: &Event);
}

pub struct Counter {
    count: u64,
}

impl Counter {
    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn print(&self) {
        println!("{}", self.count);
    }
}

pub enum Event {
    CurrentTemperature(u64),
    LightDensity(u64),
    SmokeDensity(u64),
}

#[derive(Eq, Hash, PartialEq)]
pub enum EventType {
    Temperature,
    Light,
    Smoke,
}

pub fn run_simulation() {
    println!("Start event system sim");

    //Vec<Box<dyn FnMut(&Event)>>
    let mut events = HashMap::<EventType, Vec<i64>>::new();
    events.insert(EventType::Smoke, vec![]);

    let curArr = events.get_mut(&EventType::Smoke).unwrap();
    curArr.push(10);
    println!("{:?}", events.get(&EventType::Smoke).unwrap());

    let mut counter = Counter { count: 0 };
    println!("Counter value: ");
    counter.print();

    {
        let tempReporter = |event: &Event| {
            counter.increment();
            counter.print();
        };

        let testFunc = |event: &Event| println!("Yo");

        let mut closure_events = HashMap::<EventType, Vec<Box<dyn FnMut(&Event)>>>::new();
        closure_events.insert(EventType::Smoke, vec![]);

        let arr = closure_events.get_mut(&EventType::Smoke).unwrap();
        arr.push(Box::new(testFunc));
        arr.push(Box::new(tempReporter));

        for i in arr {
            i(&Event::CurrentTemperature(20));
        }
    }
}
