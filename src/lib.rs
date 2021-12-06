use std::sync::RwLock;

pub struct Channel<'chan, ST, RT> {
    pub(crate) receivers: RwLock<Vec<&'chan mut (dyn FnMut(ST, Option<RT>) -> Option<RT> + Send + Sync)>>,
    pub(crate) result: RwLock<Option<RT>>
}

impl<'chan, Sender, Recv> Channel<'chan, Sender, Recv>
where
    Sender: Send + Clone + Sized,
    Recv: Send + Clone + Sized
{
    pub fn new() -> Self {
        Channel {
            receivers: RwLock::new(Vec::new()),
            result: RwLock::new(None)
        }
    }

    pub fn receive(&self, receiver: &'chan mut (dyn FnMut(Sender, Option<Recv>) -> Option<Recv> + Send + Sync)) {
        let mut receivers = self.receivers.write().unwrap();
        receivers.push(receiver);
        drop(receivers);
    }

    pub fn send(&self, data: Sender) -> Option<Recv> {
        let mut receivers = self.receivers.write().unwrap();
        let mut result = self.result.write().unwrap();
        for receiver in (*receivers).iter_mut() {
            *result = receiver(data.clone(), match (*result).clone() {
                Some(r) => Some(r),
                None => None
            }).clone();
        }
        result.clone()
    }
}

pub struct ChannelEmitResult<RT> {
    pub(crate) is_finished: RwLock<bool>,
    pub(crate) data: RwLock<Option<RT>>,
}

impl<receiver> ChannelEmitResult<Receiver>
where
    Receiver: Send + Clone + Sized, {
    pub fn wait(&self) -> Option<Receiver> {
        loop {
            if *self.is_finished.read().unwrap() == true {
                break;
            }
        }

        self.data.read().unwrap().clone()
    }
}