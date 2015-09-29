pub mod connection;
pub mod irc_data_store;
pub mod irc_configuration;

use std::sync::{Mutex, Arc};
use std::thread;
use loirc::Event;

use self::connection::*;
use self::irc_data_store::*;
use self::irc_configuration::*;

pub struct IrcController{
    configuration: IrcConfiguration,
    datastore : Mutex<IrcDataStore>,
    connection: Arc<Connection>,

}

impl IrcController{
    pub fn new(config: IrcConfiguration) -> IrcController{
        let tmp_config = config.clone();
        let controller = IrcController{
            configuration: config,
            connection: Arc::new(Connection::create(&format!("{}:{}",tmp_config.host_name,tmp_config.port), &tmp_config.nick_name, &tmp_config.user_name)),
            datastore: Mutex::new(IrcDataStore::new())
        };

        controller
    }
    pub fn start_handling(self) -> Arc<IrcController>  {
        let arc = Arc::new(self);
        let arc_clone = arc.clone();
        thread::spawn( move || {
            loop{
                arc_clone.handle_event(arc_clone.connection.get_message());
            }
        });
        arc
    }
    pub fn handle_event(&self, event: Event){
        println!("{:?}",event);
    }
}
