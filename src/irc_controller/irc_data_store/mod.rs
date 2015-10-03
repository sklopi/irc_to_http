mod channel;

use std::collections::BTreeMap;
use std::collections::LinkedList;
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
    pub fn get_channel_list(&self) -> Vec<&Channel>{
        let mut vec : Vec<&Channel> = Vec::new();
        for (key,channel) in self.channels.iter() {
            vec.push(&channel);
        }
        vec
    }
}
