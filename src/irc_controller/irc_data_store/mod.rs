mod channel;

use std::collections::BTreeMap;
use irc_controller::irc_data_store::channel::Channel;


pub struct IrcDataStore{
    channels: BTreeMap<String, Channel>
}
// Use an arc for connection
impl IrcDataStore{
    pub fn new() -> IrcDataStore{
        let datastore = {
            IrcDataStore{
                channels: BTreeMap::new()
            }
        };
        datastore
    }
    pub fn add_channel(&mut self, channel_name : String){
        let channel_name_tmp = channel_name.clone();
        self.channels.insert(channel_name, Channel::new(channel_name_tmp));
    }
}
