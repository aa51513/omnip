use clap::Parser;
use log::error;
use rsproxy::*;

extern crate pretty_env_logger;

fn main() {
    let args = RsproxyArgs::parse();

    rs_utilities::LogHelper::init_logger("rsp", &args.loglevel);

    let addr = parse_sock_addr(&args.addr);
    if addr.is_none() {
        error!("invalid address: {}", &args.addr);
        return;
    }

    let config = Config {
        addr: addr.unwrap(),
        downstream_addr: parse_sock_addr(args.downstream.as_str()),
        proxy_rules_file: args.proxy_rules_file,
        threads: args.threads,
        dot_server: args.dot_server,
        name_servers: args.name_servers,
        watch_proxy_rules_change: args.watch_proxy_rules_change,
    };

    let mut server = Server::new(config);
    // server.set_enable_on_info_report(true);
    // server.set_on_info_listener(|data: &str| {
    //     log::info!("Server Info: {}", data);
    // });
    server.start_and_block().ok();
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct RsproxyArgs {
    /// Address ([ip:]port pair) to listen on
    #[clap(short = 'l', long, required = true, display_order = 1)]
    addr: String,

    /// [ip:]port, downstream which the proxy server will relay traffic to based on proxy rules
    #[clap(short = 'd', long, default_value = "", display_order = 2)]
    downstream: String,

    /// Path to the proxy rules file
    #[clap(short = 'r', long, default_value = "", display_order = 3)]
    proxy_rules_file: String,

    /// Threads to run async tasks, default to number of cpu cores
    #[clap(short = 't', long, default_value = "0", display_order = 4)]
    threads: usize,

    /// DoT (DNS-over-TLS) server, e.g. dns.google
    #[clap(long, default_value = "", display_order = 5)]
    dot_server: String,

    /// comma saprated domain servers (E.g. 1.1.1.1,8.8.8.8), which will be used if no dot_server is specified, or system default if empty
    #[clap(long, default_value = "", display_order = 6)]
    name_servers: String,

    /// reload proxy rules if updated
    #[clap(short = 'w', long, action, display_order = 7)]
    watch_proxy_rules_change: bool,

    #[clap(short = 'L', long, possible_values = &["T", "D", "I", "W", "E"], default_value = "I", display_order = 8)]
    loglevel: String,
}
