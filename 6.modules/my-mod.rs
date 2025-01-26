fn function() {
    println!("This is a function");
}

// mod keyword to define a module
mod outer_module {

    // add pub keyword to make it public
    pub fn public_function() {
        println!("This is a public function in outer_module");
        // self keyword: current module
        self::private_function();
    }

    pub fn indirect_access() {
        // self keyword: current module
        self::private_inner_module::call_private_inner_private_function();
    }

    // Items inside the module are private by default
    fn private_function() {
        println!("This is a private function in outer_module");
        self::function();
    }

    fn function() {
        println!("This is a function in outer_module");
    }

    // pub(crate) makes the function public within the crate
    pub(crate) fn another_public_function() {
        println!("This is a public function inside another_module.");
    }

    // nested module
    pub mod inner_module {
        pub fn inner_public_function() {
            println!("This is a public function inside the inner_module.");
            // super keyword: parent module
            // super::private_function(); // an inner module can access private function of the parent module
            // super::function();

            // here these 2 call will call the same function
            // super::super::function();
            // here we can use crate keyword to access the root of the crate
            // crate::function();
        }

        // pub(super) makes the function public within the parent module
        pub(super) fn call_private_function() {
            inner_private_function();
        }

        fn inner_private_function() {
            println!("This is a private function inside the inner_module.");
        }
    }

    mod private_inner_module {

        pub(crate) fn call_private_inner_private_function2() {
            private_inner_private_function();
        }

        // pub(super) makes the function public within the parent module
        pub(super) fn call_private_inner_private_function() {
            private_inner_private_function();
        }

        // this is of no use as the module is private
        pub fn private_inner_public_function() {
            println!("This is a public function inside the private_inner_module.");
        }

        fn private_inner_private_function() {
            println!("This is a private function inside the private_inner_module.");
        }
    }
}

mod another_module {
    #[derive(Debug)]
    pub struct PublicStruct {
        pub public_field: i32,
        private_field: i32,
    }

    impl PublicStruct {
        pub fn new() -> PublicStruct {
            PublicStruct {
                public_field: 0,
                private_field: 0,
            }
        }
    }

    #[derive(Debug)]
    pub enum PublicEnum {
        PublicVariant,
        PrivateVariant,
    }
}

fn main() {
    // call public function
    outer_module::public_function();
    // outer_module::private_function(); // Error: function is private

    // call public function inside the inner module
    outer_module::inner_module::inner_public_function();
    // outer_module::inner_module::inner_private_function(); // Error: function is private

    // call public function inside the private inner module
    // outer_module::private_inner_module::private_inner_public_function(); // can't access private inner module
    // outer_module::private_inner_module::call_private_inner_public_function(); // can't access public function of private inner module even if it uses pub(super) / pub(crate)
    outer_module::indirect_access();

    // create a public struct
    let public_struct = another_module::PublicStruct::new();
    // let public_struct_n = another_module::PublicStruct {public_field: 0}; // can't construct struct directly as private field is not accessible
    println!("public_struct.public_field: {}", public_struct.public_field);
    // println!("Private field: {}", public_struct.private_field); // Error: field is private

    // create a public enum variant
    let public_variant = another_module::PublicEnum::PublicVariant;
    println!("public_enum::public_variant: {:?}", public_variant);
    let private_variant = another_module::PublicEnum::PrivateVariant; // Error: variant is private
    println!("public_enum::private_variant: {:?}", private_variant);

    // 'use' keyword to bring items into scope
    use outer_module::inner_module::inner_public_function;
    // use outer_module::inner_module::inner_public_function as ipf; // alias
    inner_public_function();
}
