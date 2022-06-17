use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use hikv::{CommandRequest, CommandResponse, RocksDb, Service, ServiceInner};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let service: Service<RocksDb> = ServiceInner::new(RocksDb::new("/tmp/hikv"))
        .fn_before_reply(|ret| match ret.message.as_ref() {
            "" => ret.message = "altered. Original message is empty.".into(),
            s => ret.message = format!("altered: {}", s),
        })
        .into();

    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listenering on {addr}");

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connented", addr);
        let svc = service.clone();
        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();
            while let Some(Ok(cmd)) = stream.next().await {
                // 返回一个 404 response
                let resp = svc.execute(cmd);
                stream.send(resp).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
