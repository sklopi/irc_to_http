use loirc::{connect,  Event, Writer, Reader};
use rand;
use rand::Rng;
use std::sync::mpsc::{  TryRecvError};
use std::sync::{Arc, Mutex};



pub struct Connection{
    writer  : Writer,
    reader  : Arc<Mutex<Reader>>,
    connection_password : u32,
    nickname : String,
    username : String
}
impl Connection{
    pub fn create(host : &str, nickname : &str, username : &str) -> Connection {
        let (writer, reader) = connect(host, Default::default()).unwrap();
        let mut rnd = rand::thread_rng();
        let threading_writer = writer.clone();

        let connection = Connection{
            writer          : writer,
            reader          : Arc::new(Mutex::new(reader)),
            connection_password : rnd.gen::<u32>(),
            nickname : nickname.to_string(),
            username : username.to_string()
        };

        threading_writer.raw(format!("PASS {}\n ", connection.connection_password)).unwrap();
        threading_writer.raw(format!("USER {} * * :{}\n", connection.username, connection.username)).unwrap();
        threading_writer.raw(format!("NICK {} \n ", connection.nickname)).unwrap();

        connection
    }
    /// Get a loirc::Event from the connection
    /// This Method is the Blocking version which returns an Event or panics!
    pub fn get_message(&self) -> Event {
        let guard = self.reader.lock().unwrap();
        guard.recv().unwrap()
    }
    pub fn get_message_nonblocking(&self) -> Result<Event, TryRecvError> {
        let guard = self.reader.lock().unwrap();
        guard.try_recv()
    }
    pub fn get_writer(&self) -> Writer {
        self.writer.clone()
    }
    /// Tells the irc Server you want to join a channel
    /// # Arguments
    ///
    /// * `channel` - channelname, you may pass the name with or without '#'
    pub fn join_channel(&self, channel :&str) -> bool {
        let mut channel = channel.to_string();
        if !channel.starts_with("#") {
            channel = format!("#{}",channel);
        }
        match self.writer.raw(format!("JOIN {}\n",channel)){
            Ok(_) => true,
            Err(_) => false
        }
    }
    pub fn part_channel(&self, channel :&str) -> bool {
        let mut channel = channel.to_string();
        if !channel.starts_with("#") {
            channel = format!("#{}",channel);
        }
        match self.writer.raw(format!("PART {}\n",channel)){
            Ok(_) => true,
            Err(_) => false
        }
    }
}
