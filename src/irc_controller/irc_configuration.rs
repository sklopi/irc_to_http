
#[derive(Clone)]
pub struct IrcConfiguration{
    pub host_name: String,
    pub port: u16,
    pub user_name: String,
    pub nick_name: String,
}
impl IrcConfiguration{
    pub fn new () -> IrcConfiguration {
            IrcConfiguration{
                host_name : String::from("127.0.0.1"),
                port: 6667,
                user_name : String::from("gonzales"),
                nick_name : String::from("gonzales")
            }
    }
    pub fn host_name(&mut self, host_name : String)-> &mut IrcConfiguration {
        self.host_name = host_name;
        self
    }
    pub fn port(&mut self, port : u16)-> &mut IrcConfiguration {
        self.port = port;
        self
    }
    pub fn user_name(&mut self, user_name : String)-> &mut IrcConfiguration {
        self.user_name = user_name;
        self
    }
    pub fn nick_name(&mut self, nick_name : String)-> &mut IrcConfiguration {
        self.nick_name = nick_name;
        self
    }
}
