use crate::answer::Answer;
use crate::header::Header;
use crate::query::Question;
use bincode::{Decode, Encode};

/// DNS 协议通信消息
#[derive(Encode, Decode)]
pub struct Message {
    header: Header,
    question: Question,
    answer: Answer,
}
