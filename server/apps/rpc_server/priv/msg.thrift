namespace * msg

struct Message {
  1:  i64 id,
  2:  string text
}

service MsgService {
  Message hello(1: Message m)
}