use std::io::{Read, Write};

use bytes::{Buf, BufMut, BytesMut};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use prost::Message;
use tokio::io::{AsyncRead, AsyncReadExt};
use tracing::debug;

use crate::{CommandRequest, CommandResponse, HikvError};

/// 长度占用4字节
pub const LEN_LEN: usize = 4;
/// 内容占 31 bit，最大frame size为 2GB
const MAX_FRAME: usize = 2 * 1024 * 1024;
/// 压缩阈值, 1500(MTU)-20(IP-Header)-20(TCP-Header)-20(TCP-Options)-4(Frame-Length)
const COMPRESSION_LIMIT: usize = 1436;
/// 压缩标志位(长度最高位)
const COMPRESSION_BIT: usize = 1 << 31;

/// message codec
pub trait FrameCodec
where
    Self: Message + Sized + Default,
{
    /// message encode to frame
    fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), HikvError> {
        let size = self.encoded_len();
        if size > MAX_FRAME {
            return Err(HikvError::FrameError);
        }

        // 写入长度，压缩后再重写
        buf.put_u32(size as _);
        if size <= COMPRESSION_LIMIT {
            // 不压缩，直接写回
            self.encode(buf)?;
            return Ok(());
        }

        let mut buf0 = Vec::with_capacity(size);
        self.encode(&mut buf0)?;

        // 分离出部分 BytesMut,之后再合并
        let payload = buf.split_off(LEN_LEN);
        buf.clear();

        // 处理压缩
        let mut encoder = GzEncoder::new(payload.writer(), Compression::default());
        encoder.write_all(&buf0)?;

        // 获取压缩后的内容
        let payload = encoder.finish()?.into_inner();
        debug!("Encode a frame: size {}({})", size, payload.len());

        // 重写len
        buf.put_u32((COMPRESSION_BIT | payload.len()) as _);

        // BytesMut 合并
        buf.unsplit(payload);

        Ok(())
    }

    /// frame decode to message
    fn decode_frame(buf: &mut BytesMut) -> Result<Self, HikvError> {
        // 解析 frame header
        let header = buf.get_u32() as usize;
        let (len, compressed) = decode_header(header);
        debug!("Got a frame: msg len {}, compressed {}", len, compressed);

        if compressed {
            // 解压缩
            let mut decoder = GzDecoder::new(&buf[..len]);
            let mut buf0 = Vec::with_capacity(len << 1);
            decoder.read_to_end(&mut buf0)?;
            buf.advance(len);

            Ok(Self::decode(&buf0[..])?)
        } else {
            let msg = Self::decode(&buf[..len])?;
            buf.advance(len);
            Ok(msg)
        }
    }
}

fn decode_header(header: usize) -> (usize, bool) {
    let len = header & !COMPRESSION_BIT;
    let compressed = header & COMPRESSION_BIT == COMPRESSION_BIT;
    (len, compressed)
}

impl FrameCodec for CommandRequest {}
impl FrameCodec for CommandResponse {}

/// 从 stream 中读取完整的 frame
pub async fn read_frame<S>(stream: &mut S, buf: &mut BytesMut) -> Result<(), HikvError>
where
    S: AsyncRead + Unpin + Send,
{
    let header = stream.read_u32().await? as usize;
    let (len, _) = decode_header(header);
    buf.reserve(LEN_LEN + len);
    buf.put_u32(header as _);
    unsafe {
        buf.advance_mut(len);
    }
    stream.read_exact(&mut buf[LEN_LEN..]).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::Value;

    use super::*;

    #[test]
    fn should_work_command_request_encode_decode() {
        let mut buf = BytesMut::new();

        let cmd = CommandRequest::new_set("hello", "world".into());
        cmd.encode_frame(&mut buf).unwrap();

        assert_eq!(is_compressed(&buf), false);

        let cmd0 = CommandRequest::decode_frame(&mut buf).unwrap();
        assert_eq!(cmd0, cmd);
    }

    #[test]
    fn should_work_command_response_encode_decode() {
        let mut buf = BytesMut::new();

        let values: Vec<Value> = vec![1.into(), "rust".into(), b"hello".into()];
        let ret: CommandResponse = values.into();
        ret.encode_frame(&mut buf).unwrap();

        assert_eq!(is_compressed(&buf), false);

        let ret0 = CommandResponse::decode_frame(&mut buf).unwrap();
        assert_eq!(ret0, ret);
    }

    #[test]
    fn should_work_command_response_compressed_encode_decode() {
        let mut buf = BytesMut::new();

        let value: Value = Bytes::from(vec![0u8; COMPRESSION_LIMIT + 1]).into();
        let ret: CommandResponse = value.into();
        ret.encode_frame(&mut buf).unwrap();

        assert_eq!(is_compressed(&buf), true);

        let ret0 = CommandResponse::decode_frame(&mut buf).unwrap();
        assert_eq!(ret0, ret);
    }

    fn is_compressed(data: &[u8]) -> bool {
        if let &[v] = &data[..1] {
            v >> 7 == 1
        } else {
            false
        }
    }

    struct DummyStream {
        buf: BytesMut,
    }
    impl AsyncRead for DummyStream {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            let len = buf.capacity();
            let data = self.get_mut().buf.split_to(len);
            buf.put_slice(&data);
            std::task::Poll::Ready(Ok(()))
        }
    }

    #[tokio::test]
    async fn should_work_read_frame() {
        let mut buf = BytesMut::new();
        let cmd = CommandRequest::new_set("hello", "world".into());
        cmd.encode_frame(&mut buf).unwrap();
        let mut stream = DummyStream { buf };

        let mut data = BytesMut::new();
        read_frame(&mut stream, &mut data).await.unwrap();

        let cmd0 = CommandRequest::decode_frame(&mut data).unwrap();
        assert_eq!(cmd0, cmd);
    }
}
