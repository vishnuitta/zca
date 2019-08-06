#[macro_use]
extern crate log;

use env_logger::Builder;
use futures::{Future, Stream};
use grpc_router::Router2;
use log::LevelFilter;
use std::fs;
use std::io::Write;
use structopt::StructOpt;
use tokio::net::{TcpListener, UnixListener};
use tower_hyper::server::{Http, Server};
use zca::csi::server::{IdentityServer, NodeServer};
use zca::identity::Identity;
use zca::node::CsiNode;
#[derive(StructOpt, Debug)]
struct CliArgs {
    #[structopt(short, long)]
    socket: String,
}

fn main() {
    let opt = CliArgs::from_args();
    let mut builder = Builder::from_default_env();

    builder
        .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Info)
        .init();

    let csi_svc = Router2::new(
        "/csi.v1.Identity/",
        IdentityServer::new(Identity::new()),
        NodeServer::new(CsiNode::new()),
    );

    let mut csi_server = Server::new(csi_svc);
    let accept_csi: Box<dyn Future<Item = (), Error = std::io::Error> + Send> =
        if opt.socket.starts_with('/') {
            // bind would fail if we did not remove stale socket
            let _ = fs::remove_file(&opt.socket);
            let bind_csi = UnixListener::bind(&opt.socket).expect("bind");
            Box::new(bind_csi.incoming().for_each(move |sock| {
                debug!("New csi connection");
                let http = Http::new().http2_only(true).clone();
                let serve = csi_server.serve_with(sock, http.clone());
                tokio::spawn(serve.map_err(|e| error!("error on CSI connection: {}", e)));
                Ok(())
            }))
        } else {
            let endpoint_csi = opt.socket.parse().unwrap();

            let bind_csi = TcpListener::bind(&endpoint_csi).expect("bind");
            Box::new(bind_csi.incoming().for_each(move |sock| {
                debug!("New csi connection");
                let http = Http::new().http2_only(true).clone();
                let serve = csi_server.serve_with(sock, http.clone());
                tokio::spawn(serve.map_err(|e| error!("error on CSI connection: {}", e)));
                Ok(())
            }))
        };

    info!("CSI listening on {}", opt.socket);

    tokio::run(accept_csi.then(|res| {
        if let Err(err) = res {
            error!("accept error: {}", err);
        }
        Ok(())
    }));
}
