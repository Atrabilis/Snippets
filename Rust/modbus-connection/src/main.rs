use modbus::{tcp, Client};

fn main() {
    let mut cfg = tcp::Config::default();
    cfg.modbus_uid = 20;
    cfg.tcp_port = 502;
    let mut client =
        tcp::Transport::new_with_cfg("192.168.100.8", cfg).expect("Creación del cliente Fallida");
    let register_reading = client
        .read_holding_registers(40162, 1)
        .expect("Lectura de dato invalida");
    let freq = register_reading[0] / 10;
    println!("{:#?}", freq);
}
