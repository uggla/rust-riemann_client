extern crate riemann_client;
extern crate simple_logger;

fn main() {
    simple_logger::init();

    let mut client = riemann_client::Client::connect(&("localhost", 5555)).unwrap();

    let mut event = riemann_client::proto::Event::new();
    event.set_service("rust".to_string());
    event.set_state("warn".to_string());
    event.set_host("centurion".to_string());

    client.send_event(event).unwrap();
}