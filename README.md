# Events
A unconvential way to handle events with rust

Proposal API:
```rust
#[derive(Debug, Clone)]
pub struct EmittingData;

impl Send for EmittingData {}

// create an event channel.
let channel = EventChannel::new<EmittingData>("name", EmittingData::default());
let result = channel.recieve(|data: &mut EmittingData| -> ReceivingData {
    // return data to return to the emitter.
    RecievingData::new()
});

// on another thread
thread::spawn(move || {
    let result = channel.send(EmittingData {}).unwrap();
})
```