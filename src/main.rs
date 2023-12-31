mod send_email;
mod send_sms_via_email;
mod utils;

fn main() {
    let master_amount = "2";
    let secondary_amount = "42";
    let tertiary_amount = "400";

    // send_email::send_email(
    //     "Dennis",
    //     &utils::get_var("DENNIS_GMAIL"),
    //     &format!("Hi {}! Rent is ${}.", "Dennis", secondary_amount),
    // );

    send_sms_via_email::send_sms_via_email(
        "Dennis",
        &utils::get_var("DENNIS_NUMBER"),
        &format!("Hi {}! Rent is ${}.", "Dennis", secondary_amount),
        &utils::get_gateway("Google Fi"),
    );

    send_sms_via_email::send_sms_via_email(
        "Ivan",
        &utils::get_var("IVAN_NUMBER"),
        &format!("Hi {}! Rent is ${} 💀", "Ivan", tertiary_amount),
        &utils::get_gateway(&utils::get_var("IVAN_CARRIER")),
    );

    // send_sms_via_email(
    //     "Ryan",
    //     &get_var("NGUYEN_NUMBER"),
    //     &format!("Hi {}! Rent is ${} 💀", "Ryan", master_amount),
    //     get_gateway(&utils::get_var("NGUYEN_CARRIER")),
    // );
    // send_sms_via_email(
    //     "Danny",
    //     &utils::get_var("DANNY_NUMBER"),
    //     &format!("Hi {}! Rent is ${} 💀", "Danny", secondary_amount),
    //     &utils::get_gateway(&utils::get_var("DANNY_CARRIER")),
    // );
}
