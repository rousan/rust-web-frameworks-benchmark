use trillium::Conn;

async fn hello_world(conn: Conn) -> Conn {
    conn.ok("Hello, world!")
}

fn main() {
    trillium_smol::run(hello_world)
}
