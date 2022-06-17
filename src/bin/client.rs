use hikv::{CommandRequest, HikvError, ProstClientStream};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), HikvError> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    // 连接服务器
    let stream = TcpStream::connect(addr).await?;

    let mut client = ProstClientStream::new(stream);

    // 生成一个 HSET 命令
    let cmd = CommandRequest::new_set("hello", "world".into());

    // 发送 HSET 命令
    let data = client.execute(cmd).await?;
    info!("Got response {:?}", data);

    Ok(())
}
