mod frame;
mod tls;

use bytes::BytesMut;
pub use frame::*;
pub use tls::*;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::{CommandRequest, CommandResponse, HikvError, Service};

pub struct ProstServerStream<S> {
    inner: S,
    service: Service,
}

pub struct ProstClientStream<S> {
    inner: S,
}

impl<S> ProstServerStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    pub fn new(inner: S, service: Service) -> Self {
        Self { inner, service }
    }

    pub async fn process(mut self) -> Result<(), HikvError> {
        while let Ok(cmd) = self.recv().await {
            let ret = self.service.execute(cmd);
            self.send(ret).await?;
        }
        Ok(())
    }

    pub async fn recv(&mut self) -> Result<CommandRequest, HikvError> {
        let mut buf = BytesMut::new();
        read_frame(&mut self.inner, &mut buf).await?;
        CommandRequest::decode_frame(&mut buf)
    }

    pub async fn send(&mut self, cmd: CommandResponse) -> Result<(), HikvError> {
        let mut buf = BytesMut::new();
        cmd.encode_frame(&mut buf).unwrap();
        let encoded = buf.freeze();
        self.inner.write_all(&encoded[..]).await?;
        Ok(())
    }
}

impl<S> ProstClientStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    pub fn new(inner: S) -> Self {
        Self { inner }
    }

    pub async fn execute(&mut self, cmd: CommandRequest) -> Result<CommandResponse, HikvError> {
        self.send(cmd).await?;
        Ok(self.recv().await?)
    }

    async fn send(&mut self, cmd: CommandRequest) -> Result<(), HikvError> {
        let mut buf = BytesMut::new();
        cmd.encode_frame(&mut buf)?;
        let encoded = buf.freeze();
        self.inner.write_all(&encoded[..]).await?;
        Ok(())
    }

    async fn recv(&mut self) -> Result<CommandResponse, HikvError> {
        let mut buf = BytesMut::new();
        read_frame(&mut self.inner, &mut buf).await?;
        CommandResponse::decode_frame(&mut buf)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use bytes::Bytes;
    use std::net::SocketAddr;
    use tokio::net::{TcpListener, TcpStream};

    use crate::{assert_ok, MemTable, ServiceInner, Value};

    use super::*;

    #[tokio::test]
    async fn client_server_basic_communication_should_work() -> anyhow::Result<()> {
        let addr = start_server().await?;

        let stream = TcpStream::connect(addr).await?;
        let mut client = ProstClientStream::new(stream);

        // 发送 HSET，等待回应

        let cmd = CommandRequest::new_set("k1", "v1".into());
        let res = client.execute(cmd).await.unwrap();

        // 第一次 HSET 服务器应该返回 None
        assert_ok(res, &[Value::default()]);

        // 再发一个 HSET
        let cmd = CommandRequest::new_get("k1");
        let res = client.execute(cmd).await?;

        // 服务器应该返回上一次的结果
        assert_ok(res, &["v1".into()]);

        Ok(())
    }

    #[tokio::test]
    async fn client_server_compression_should_work() -> anyhow::Result<()> {
        let addr = start_server().await?;

        let stream = TcpStream::connect(addr).await?;
        let mut client = ProstClientStream::new(stream);

        let v: Value = Bytes::from(vec![0u8; 16384]).into();
        let cmd = CommandRequest::new_set("k2", v.clone().into());
        let ret = client.execute(cmd).await?;

        assert_ok(ret, &[Value::default()]);

        let cmd = CommandRequest::new_get("k2");
        let ret = client.execute(cmd).await?;

        assert_ok(ret, &[v.into()]);

        Ok(())
    }

    async fn start_server() -> Result<SocketAddr> {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                let service: Service = ServiceInner::new(MemTable::new()).into();
                let server = ProstServerStream::new(stream, service);
                tokio::spawn(server.process());
            }
        });

        Ok(addr)
    }
}
