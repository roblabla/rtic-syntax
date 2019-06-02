#![no_main]

#[mock::app(cores = 2, parse_cores)]
const APP: () = {
    extern "C" {
        static A: u32;
        static B: u32;
    }

    #[init(core = 0, late = [A])]
    fn init(_: init::Context) -> init::LateResources {}

    #[init(core = 1, late = [A])]
    fn init(_: init::Context) -> init::LateResources {}
};
