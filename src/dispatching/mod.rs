pub trait Hi {
    fn hi(&self);
}

impl Hi for &str {
    fn hi(&self) {
        println!("Hi {}", self);
    }
}

impl Hi for String {
    fn hi(&self) {
        println!("Hi {}", self);
    }
}

pub fn print_hi_list(list: &[Box<dyn Hi>]) {
    for item in list {
        item.hi();
        // item.weird();
    }
}

#[cfg(test)]
mod test {
    use super::print_hi_list;

    #[test]
    fn test() {
        // let l1 = vec!["evren", "okcu"];
        // let l2 = vec![Box::new("evren"), Box::new(String::from("okcu"))];
        // print_hi_list(&l1);
    }
}
