#[cfg(test)]
mod tests {
    use std::default;

    #[test]
    fn function_item() {
        fn foo() {}
        let mut fn_item = foo;
        let mut fn_item2 = foo;
        fn_item2 = fn_item;
    }

    #[test]
    fn generic_function_item() {
        fn bar<T>(_: u32) -> u32 {
            0
        }
        baz(bar::<u32>);
        baz(bar::<i32>);
        //coerce a function item to function pointer
        fn baz(f: fn(u32) -> u32) {
            println!("{}", std::mem::size_of_val(&f))
        }
    }
    #[test]
    fn function_trait() {
        fn bar<T>() {}
        fn quox<F>(f: F)
        where
            F: Fn(),
        {
        }
        //coerce function item to function trait
        quox(bar::<&str>);
    }
    #[test]
    fn convert_mutable_ref_to_shared() {
        let mut x = 2;
        let mutable_ref = &mut x;
        let shared_ref = &*mutable_ref;

        println!("{}", shared_ref);
    }
    #[test]
    fn function_trait2() {
        fn bar<T>() {}
        fn quox<F>(f: F)
        where
            F: Fn(),
        {
            f()
        }
        //coerce function item to function trait
        quox(bar::<&str>);
    }
    #[test]
    fn closure() {
        fn bar<T>() {}

        //coerce a function item to function pointer
        fn baz(f: fn()) {
            println!("{}", std::mem::size_of_val(&f))
        }
        fn quox<F>(f: F)
        where
            F: Fn(),
        {
            f()
        }
        let x = || ();
        baz(x);
        quox(x);
    }
    #[test]
    fn closure_move() {
        fn make_fn() -> impl Fn() {
            let x = String::new();
            //would not work without 'move'
            move || {
                println!("{}", x);
            }
        }
        let x = make_fn();
    }
    #[test]
    fn dyn_fn_test() {
        fn fn_trait(f: Box<dyn Fn()>) {
            f();
        }
        fn fn_mut_trait(mut f: Box<dyn FnMut()>) {
            f();
        }
        fn fn_once(mut f: Box<dyn FnMut()>) {
            f();
        }
    }
    #[test]
    fn dyn_fn_test2() {
        // fn bar<T>() {}

        // //coerce a function item to function pointer
        // fn baz(f: fn()) {
        //     println!("{}", std::mem::size_of_val(&f))
        // }
        fn quox<F>(f: F)
        where
            F: Fn(),
        {
            f()
        }
        let f = || ();
        let fn_dyn: &dyn Fn() = &f;
        quox(fn_dyn);
    }
    #[test]
    fn for_bounds() {
        fn quox<F>(f: F)
        where
            F: for<'a> Fn(&'a str) -> &'a str,
        {
        }
        quox(|x| x);
    }
}
