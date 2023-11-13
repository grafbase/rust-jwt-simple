#[cfg(feature = "eddsa")]
mod eddsa;
#[cfg(feature = "es256")]
mod es256;
#[cfg(feature = "es256k")]
mod es256k;
#[cfg(feature = "es384")]
mod es384;
#[cfg(feature = "hmac")]
mod hmac;
#[cfg(feature = "rsa")]
mod rsa;

#[cfg(feature = "eddsa")]
pub use self::eddsa::*;
#[cfg(feature = "es256")]
pub use self::es256::*;
#[cfg(feature = "es256k")]
pub use self::es256k::*;
#[cfg(feature = "es384")]
pub use self::es384::*;
#[cfg(feature = "hmac")]
pub use self::hmac::*;
#[cfg(feature = "rsa")]
pub use self::rsa::*;
