extern crate boinc_app_api_rs as api;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "action", content = "data")]
enum Action {
    #[serde(rename = "peek")]
    Peek(api::MsgChannel),
    #[serde(rename = "receive")]
    Receive(api::MsgChannel),
    #[serde(rename = "clear")]
    Clear(api::MsgChannel),
    #[serde(rename = "push")]
    Push(api::Message),
    #[serde(rename = "force")]
    Force(api::Message),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Request {
    pub action: Action,
    pub id: String,
}

fn main() {
    let mut app_channel = api::AppChannel::new("./boinc_mmap_file").unwrap();

    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();

        let action = serde_json::from_str::<Action>(&s).unwrap();

        println!("{:?}", action);

        let s = match action {
            Action::Peek(chan) => {
                format!("{:?}", app_channel.peek(&chan))
            }
            Action::Receive(chan) => {
                format!("{:?}", app_channel.receive(&chan))
            }
            Action::Clear(chan) => {
                format!("{:?}", app_channel.clear(&chan))
            }
            Action::Push(msg) => {
                format!("{:?}", app_channel.push(&msg))
            }
            Action::Force(msg) => {
                format!("{:?}", app_channel.force(&msg))
            }
        };

        println!("{}", s);
    }
}
