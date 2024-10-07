rust::rust! {
    extern crate rust;

    use std::collections::HashMap as Dico;

    trait KeyValue {
        fn write(&self, key: String, value: String);
        fn read(&self, key: String) -> Result<Option<&String>, String>;
    }

    static mut DBSHIT: Option<Dico<String, String>> = None;

    struct Concrete;

    impl KeyValue for Concrete {
        fn write(&self, key: String, value: String) {
            let dico = unsafe {
                DBSHIT.get_or_insert_with(Default::default)
            };
            dico.insert(key, value);
        }

        fn read(&self, key: String) -> Result<Option<&String>, String> {
            if let Some(dico) = unsafe { DBSHIT.as_ref() } {
                Ok(dico.get(&key))
            } else {
                Err("fetchez le dico".to_string())
            }
        }
    }

    pub fn maybe(i: u32) -> Option<Result<u32, String>> {
        if i % 2 == 1 {
            if i == 42 {
                Some(Err(String::from("merde")))
            } else {
                Some(Ok(33))
            }
        } else {
            None
        }
    }

    async fn example() {
    }

    async fn example2() {
        example().await;
    }

    fn important_criss() {
        let mut x = 31;

        match x {
            42 => {
                println!("omelette du fromage")
            }
            _ => println!("voila")
        }

        for i in 0..10 {
            let val = loop {
                break i;
            };

            while x < val {
                x += 1;
            }

            x = if let Some(result) = maybe(i) {
                result.unwrap_or(12)
            } else {
                12
            };
        }
    }

    #[allow(dead_code)]
    fn secondary() {
        println!("oh non");
        println!("tabernacle");
        println!("fetchez la vache");
    }
}
