/*
 * Copyright (c) 2023, Tobias Müller <git@tsmr.eu>
 *
 */

use anothertls::{TlsConfigBuilder, TlsListener};
use std::net::TcpListener;

fn main() {

    let config = TlsConfigBuilder::new()
        .set_client_cert_custom_verify_fn(|cert| {
            let name = match cert.tbs_certificate.subject.get("commonName") {
                Ok(e) => e,
                Err(_) => return false,
            };
            println!("User: {name}");
            name == "Michael Scott"
        })
        .add_client_cert_ca("./ecdsa_missing_checks/src/ca.cert".to_string())
        .add_cert_pem("./ecdsa_missing_checks/src/server.cert".to_string())
        .add_privkey_pem("./ecdsa_missing_checks/src/server.key".to_string())
        .build()
        .unwrap();

    println!("Listening on 127.0.0.1:4000");

    let tcp = TcpListener::bind("127.0.0.1:4000").expect("Error binding to tcp socket.");
    let listener = TlsListener::new(tcp, config);

    let (mut socket, _) = listener.accept().expect("Couldn't get client");

    socket.do_handshake_block().expect("Error while handshake.");

    println!("New secure connection");

    let mut buf: [u8; 4096] = [0; 4096];

    let n = socket.read(&mut buf).expect("Error reading from socket.");
    println!(
        "--- Request --- \n{}\n---------------",
        String::from_utf8(buf[..n - 4].to_vec()).unwrap()
    );
    let body = "Hello Michael Scott!\n Here is our Flag: flag_XXXXXXXXXXXXXXXXXXXXXX_";
    let data = format!(
        "\
HTTP/1.1 200\r\n\
Server: VulnTLS/1.0\r\n\
Content-Type: text/html; charset=utf-8\r\n\
Content-Length: {}\r\n\
\r\n\
{}",
        body.len(),
        body
    );

    socket
        .write_all(data.as_bytes())
        .expect("Error writing to socket.");
}
