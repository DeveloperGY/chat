pub enum Notif<'a> {
    Info(&'a str),
    Err(&'a str),
    Other(&'a str)
}

pub fn notify(notification: Notif) {
    match notification {
        Notif::Info(s) => {
            println!("[INFO]: {}", s);
        }
        
        Notif::Err(s) => {
            eprintln!("[ERR]: {}", s);
        }
        
        Notif::Other(s) => {
            println!("{}", s);
        }
    }
}