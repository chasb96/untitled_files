use std::io::ErrorKind;

use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;

pub async fn verify<T>(read: &mut T) -> Result<bool, std::io::Error> 
where
    T: AsyncRead + Unpin
{
    match read.read_exact(&mut [0u8; 80]).await {
        Ok(80) => {},
        Ok(_) => return Ok(false),
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(false),
        Err(e) => return Err(e),
    }

    let triangle_count = match read.read_u32_le().await {
        Ok(count) => count,
        Err(_) => return Ok(false),
    };

    let mut buf = [0u8; 50];
    for _ in 0..triangle_count {
        match read.read_exact(&mut buf).await {
            Ok(50) => {},
            Ok(_) => return Ok(false),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(false),
            Err(e) => return Err(e),
        }
    }

    match read.read(&mut buf).await {
        Ok(0) => Ok(true),
        Ok(_) => Ok(false),
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => Ok(true),
        Err(e) => Err(e),
    }
}