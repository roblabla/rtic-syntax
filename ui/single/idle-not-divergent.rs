#![no_main]

#[mock::app]
const APP: () = {
    #[idle]
    fn idle(_: idle::Context) {}
};
