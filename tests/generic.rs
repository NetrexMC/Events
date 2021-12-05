use events::Channel;

#[test]
pub fn test_generic() {
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

    channel.recieve(|event, _current| {
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
    });

    // lets emit 12 times
    for _ in 0..12 {
        let result = channel.send(Event::HelloWorld);
        dbg!(result);
    }
}