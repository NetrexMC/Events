use std::sync::Arc;

use netrex_events::Channel;

#[test]
pub fn test_threads() {
    #[derive(Clone)]
    enum Event {
        HelloWorld,
        GoodbyeWorld,
    }

    #[derive(Debug, Clone)]
    enum EventResult {
        Complete,
        Incomplete,
    }

    let channel = Channel::<Event, EventResult>::new();
    let mut some_value = 0;

    // Our listener
    let mut listener = |event, _current| {
        match event {
            Event::HelloWorld => {
                some_value += 1;
                if some_value == 3 {
                    return Some(EventResult::Incomplete);
                }
                Some(EventResult::Complete)
            }
            Event::GoodbyeWorld => Some(EventResult::Incomplete),
        }
    };

    channel.receive(&mut listener);

    let a = Arc::new(channel);

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            // lets emit 12 times
            for _ in 0..12 {
                let result = a.send(Event::HelloWorld);
                dbg!(result);
            }
        });
    }).unwrap();
}