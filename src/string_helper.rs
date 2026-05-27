pub trait Title {
    fn title(&self) -> String;
}

impl Title for String {
    fn title(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }
}
