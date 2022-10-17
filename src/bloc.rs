pub fn convert_user_input_code(input_code: &String) -> String {
    if input_code.len() > 10 {
        "Ok".to_owned()
    } else {
        "Err".to_owned()
    }
}
