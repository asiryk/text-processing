use regex::Regex;

pub fn handle_string(str: String) {
    let reg = Regex::new(r"(P|p)hilo[a-z]+").unwrap();

    reg.find_iter(&str).for_each(|mat| {
        println!("{} - ({},{})", mat.as_str(), mat.start(), mat.end());
    });
}

