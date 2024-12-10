#[cfg(test)]
mod test {
    use rand::Rng;

    #[test]
    fn test_mutable_ref() {
        let x = 42;
        let mut y = &x;
        let z = &mut y;
        *z = &3;

        println!("");
    }
    #[test]
    fn test_move_behind_shared_ref() {
        fn replace(s: &mut Box<i32>) {
            let was = std::mem::take(s);
            println!("was:{}", &was);
            *s = was;
            *s = Box::new(3);
            std::mem::swap(s, &mut Box::new(5));
            println!("")
        }
        let x = &mut Box::new(1);
        replace(x);
        println!("{}", x);
        println!();
    }
    #[test]
    fn test_lifetime() {
        fn rand() -> i32 {
            let mut rng = rand::thread_rng();

            // Generate a random integer between 1 and 100
            let random_number: i32 = rng.gen_range(1..=100);
            random_number
        }
        let mut x = Box::new(42);
        let r = &x;
        if rand() > 50 {
            *x = 84;
        } else {
            println!("{}", r)
        }
        //println!("{}", &r);
    }

    #[test]
    fn test_lifetime_2() {
        struct MutStr<'a, 'b> {
            s: &'a mut &'b str,
        }
        let mut s = "evren";
        let mut x = MutStr { s: &mut "hello" };
        x.s = &mut "a";

        *MutStr { s: &mut s }.s = "world";

        println!("{}", s);
    }
}
