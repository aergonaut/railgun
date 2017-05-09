extern crate dotenv;
extern crate railgun;

fn main() {
    dotenv::dotenv().ok();
    
    railgun::app().launch();
}
