use cdn_dns::config::get_config;
use cdn_dns::errors::{failed_config_read, failed_socket_bind};

use cdn_dns::load_balancer::handle_query::handle_query;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let config = get_config().expect(failed_config_read());

    let socket_addr = format!("{}:{}", config.application.host, config.application.port);
    let socket = UdpSocket::bind(socket_addr)
        .await
        .expect(failed_socket_bind());
    handle_query(&socket, config.cdn).await.unwrap();
}
