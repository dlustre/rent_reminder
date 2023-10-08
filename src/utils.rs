use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

pub fn get_var(key: &str) -> String {
    dotenv().ok();

    let env_var = env::var(key).expect(&format!("{} is not set", key));

    env_var
}

pub fn get_gateway(carrier: &str) -> Option<String> {
    let mut carriers: HashMap<&str, (&str, &str)> = HashMap::new();

    carriers.insert("AT&T", ("txt.att.net", "mms.att.net"));
    carriers.insert("T-Mobile", ("tmomail.net", "tmomail.net"));
    carriers.insert("Verizon", ("vtext.com", "vzwpix.com"));
    carriers.insert("Sprint", ("messaging.sprintpcs.com", "pm.sprint.com"));
    carriers.insert("XFinity Mobile", ("vtext.com", "mypixmessages.com"));
    carriers.insert("Virgin Mobile", ("vmobl.com", "vmpix.com"));
    carriers.insert("Tracfone", ("", "mmst5.tracfone.com"));
    carriers.insert("Metro PCS", ("mymetropcs.com", "mymetropcs.com"));
    carriers.insert(
        "Boost Mobile",
        ("sms.myboostmobile.com", "myboostmobile.com"),
    );
    carriers.insert(
        "Cricket",
        ("sms.cricketwireless.net", "mms.cricketwireless.net"),
    );
    carriers.insert("Republic Wireless", ("text.republicwireless.com", ""));
    carriers.insert("Google Fi", ("msg.fi.google.com", "msg.fi.google.com"));
    carriers.insert("U.S. Cellular", ("email.uscc.net", "mms.uscc.net"));
    carriers.insert("Ting", ("message.ting.com", ""));
    carriers.insert("Consumer Cellular", ("mailmymobile.net", ""));
    carriers.insert("C-Spire", ("cspire1.com", ""));
    carriers.insert("Page Plus", ("vtext.com", ""));

    if let Some(&(sms, mms)) = carriers.get(carrier) {
        if !mms.is_empty() {
            return Some(mms.to_string());
        } else if !sms.is_empty() {
            return Some(sms.to_string());
        }
    }

    None
}
