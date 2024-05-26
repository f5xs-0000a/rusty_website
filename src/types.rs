use {
  crate::mycology::generate::{CatInfo, GenInfo, SpecInfo},
  std::{error, result},
  tokio::{io::BufReader, net::TcpStream},
};

pub type GenFold<'g> = Box<dyn FnMut(String, &SpecInfo) -> String + 'g>;

pub type SpecFold<'s> = Box<dyn FnMut(String, usize) -> String + 's>;

pub type Condition = Box<dyn Fn(&(usize, &String)) -> bool>;

pub type Buffer<'b> = BufReader<&'b mut TcpStream>;

pub type Categories = Vec<CatInfo>;

pub type Genera = Vec<GenInfo>;

pub type Species = Vec<SpecInfo>;

pub type YamlChunks = Vec<Vec<String>>;

pub type CxnLog<'l> = &'l mut String;

pub type IpAddr = [u8; 4];

pub type Content = Vec<u8>;

pub type Request = Vec<String>;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub mod tubes {
  use std::sync::{Arc, Mutex};
  use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
  pub type Tubes<T> = (Arc<Mutex<UnboundedSender<T>>>, UnboundedReceiver<T>);

  pub type RecvTube<T> = Arc<Mutex<UnboundedReceiver<T>>>;

  pub type SendTube<T> = Arc<Mutex<UnboundedSender<T>>>;
}
