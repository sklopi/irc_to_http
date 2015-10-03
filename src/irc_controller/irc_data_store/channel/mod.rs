#[derive(Debug)]
pub struct Channel{
    channel_name : String
}

impl Channel{
    pub fn new(channel_name : String) -> Channel {
        Channel{
            channel_name: channel_name
        }
    }
    pub fn get_name(&self) -> String{
        self.channel_name.clone()
    }
}
