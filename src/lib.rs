use std::sync::RwLock;

#[derive(Clone)]
pub struct Channel<ST, RT> {
    pub(crate) receivers: Vec<fn(ST, Option<RT>) -> Option<RT>>,
    pub(crate) result: Option<RT>
}

impl<Sender, Recv> Channel<Sender, Recv>
where
    Sender: Send + Clone + Sized,
    Recv: Send + Clone + Sized
{
    pub fn new() -> Self {
        Channel {
            receivers: Vec::new(),
            result: None
        }
    }

    pub fn recieve(&mut self, receiver: fn(Sender, Option<Recv>) -> Option<Recv>) {
        self.receivers.push(receiver);
    }

    /// Creates a SendResult handle for the channel.
    /// This handle is safe to transport between threads, as it's data is a reference.
    pub fn send(channel: Box<Channel<Sender, Recv>>, data: Sender) -> SendResult<Sender, Recv> {
        // creates a send result handle.
        let result = SendResult {
            is_finished: RwLock::new(false),
            result: RwLock::new(None),
            channel,
            data: data
        };
        result
    }

    pub fn recieve_result(&mut self, sender: &mut SendResult<Sender, Recv>) {
        for receiver in (*self.receivers).iter_mut() {
            self.result = receiver(sender.data.clone(), match (self.result).clone() {
                Some(r) => Some(r),
                None => None
            }).clone();
        }

        sender.is_finished.write().unwrap().clone_from(&true);
        sender.result.write().unwrap().clone_from(&self.result);
    }
}

pub struct SendResult<ST, RT> {
    pub(crate) is_finished: RwLock<bool>,
    pub(crate) result: RwLock<Option<RT>>,
    pub(crate) channel: Box<Channel<ST, RT>>,
    pub(crate) data: ST
}

impl<Sender, Reciever> SendResult<Sender, Reciever>
where
    Sender: Send + Clone + Sized,
    Reciever: Send + Clone + Sized, {
    pub fn wait(&mut self) -> Option<Reciever> {
        let channel = self.channel.clone().as_mut().recieve_result(self);
        loop {
            if *self.is_finished.read().unwrap() == true {
                break;
            }
        }

        self.result.read().unwrap().clone()
    }
}