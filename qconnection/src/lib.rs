use std::net::SocketAddr;

pub mod conn;
pub mod error;
pub mod path;
mod pipe;
pub mod router;
pub mod tls;
pub mod usc;

/// 发送报文的trait，但其实发送还有其他需要的形式，比如：
/// - 携带ttl设置发送
/// - sendmmsg/send_vectored，多个包一次系统调用发送，要求向同一个目标
///   - 配合GSO，携带segment size的形式 发送，内核发送优化，将是最高效的发送方法
pub trait Sendmsg {
    fn sendmsg(&mut self, msg: &[u8], dest: SocketAddr) -> std::io::Result<usize>;
}

#[cfg(test)]
mod tests {}
