// Autogenerated by Thrift Compiler (0.12.0)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments, type_complexity))]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate ordered_float;
extern crate thrift;
extern crate try_from;

use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::From;
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use try_from::TryFrom;

use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::server::TProcessor;

//
// Message
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Message {
  pub id: i64,
  pub text: String,
}

impl Message {
  pub fn new(id: i64, text: String) -> Message {
    Message {
      id: id,
      text: text,
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<Message> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<i64> = None;
    let mut f_2: Option<String> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i64()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("Message.id", &f_1)?;
    verify_required_field_exists("Message.text", &f_2)?;
    let ret = Message {
      id: f_1.expect("auto-generated code should have checked for presence of required fields"),
      text: f_2.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Message");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("id", TType::I64, 1))?;
    o_prot.write_i64(self.id)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("text", TType::String, 2))?;
    o_prot.write_string(&self.text)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// MsgService service client
//

pub trait TMsgServiceSyncClient {
  fn hello(&mut self, m: Message) -> thrift::Result<Message>;
}

pub trait TMsgServiceSyncClientMarker {}

pub struct MsgServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  _i_prot: IP,
  _o_prot: OP,
  _sequence_number: i32,
}

impl <IP, OP> MsgServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  pub fn new(input_protocol: IP, output_protocol: OP) -> MsgServiceSyncClient<IP, OP> {
    MsgServiceSyncClient { _i_prot: input_protocol, _o_prot: output_protocol, _sequence_number: 0 }
  }
}

impl <IP, OP> TThriftClient for MsgServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
  fn i_prot_mut(&mut self) -> &mut TInputProtocol { &mut self._i_prot }
  fn o_prot_mut(&mut self) -> &mut TOutputProtocol { &mut self._o_prot }
  fn sequence_number(&self) -> i32 { self._sequence_number }
  fn increment_sequence_number(&mut self) -> i32 { self._sequence_number += 1; self._sequence_number }
}

impl <IP, OP> TMsgServiceSyncClientMarker for MsgServiceSyncClient<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {}

impl <C: TThriftClient + TMsgServiceSyncClientMarker> TMsgServiceSyncClient for C {
  fn hello(&mut self, m: Message) -> thrift::Result<Message> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("hello", TMessageType::Call, self.sequence_number());
        let call_args = MsgServiceHelloArgs { m: m };
        self.o_prot_mut().write_message_begin(&message_ident)?;
        call_args.write_to_out_protocol(self.o_prot_mut())?;
        self.o_prot_mut().write_message_end()?;
        self.o_prot_mut().flush()
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin()?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("hello", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut())?;
        self.i_prot_mut().read_message_end()?;
        return Err(thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = MsgServiceHelloResult::read_from_in_protocol(self.i_prot_mut())?;
      self.i_prot_mut().read_message_end()?;
      result.ok_or()
    }
  }
}

//
// MsgService service processor
//

pub trait MsgServiceSyncHandler {
  fn handle_hello(&self, m: Message) -> thrift::Result<Message>;
}

pub struct MsgServiceSyncProcessor<H: MsgServiceSyncHandler> {
  handler: H,
}

impl <H: MsgServiceSyncHandler> MsgServiceSyncProcessor<H> {
  pub fn new(handler: H) -> MsgServiceSyncProcessor<H> {
    MsgServiceSyncProcessor {
      handler,
    }
  }
  fn process_hello(&self, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    TMsgServiceProcessFunctions::process_hello(&self.handler, incoming_sequence_number, i_prot, o_prot)
  }
}

pub struct TMsgServiceProcessFunctions;

impl TMsgServiceProcessFunctions {
  pub fn process_hello<H: MsgServiceSyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let args = MsgServiceHelloArgs::read_from_in_protocol(i_prot)?;
    match handler.handle_hello(args.m) {
      Ok(handler_return) => {
        let message_ident = TMessageIdentifier::new("hello", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident)?;
        let ret = MsgServiceHelloResult { result_value: Some(handler_return) };
        ret.write_to_out_protocol(o_prot)?;
        o_prot.write_message_end()?;
        o_prot.flush()
      },
      Err(e) => {
        match e {
          thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("hello", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.description()
              )
            };
            let message_ident = TMessageIdentifier::new("hello", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident)?;
            thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot)?;
            o_prot.write_message_end()?;
            o_prot.flush()
          },
        }
      },
    }
  }
}

impl <H: MsgServiceSyncHandler> TProcessor for MsgServiceSyncProcessor<H> {
  fn process(&self, i_prot: &mut TInputProtocol, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let message_ident = i_prot.read_message_begin()?;
    let res = match &*message_ident.name {
      "hello" => {
        self.process_hello(message_ident.sequence_number, i_prot, o_prot)
      },
      method => {
        Err(
          thrift::Error::Application(
            ApplicationError::new(
              ApplicationErrorKind::UnknownMethod,
              format!("unknown method {}", method)
            )
          )
        )
      },
    };
    thrift::server::handle_process_result(&message_ident, res, o_prot)
  }
}

//
// MsgServiceHelloArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct MsgServiceHelloArgs {
  m: Message,
}

impl MsgServiceHelloArgs {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<MsgServiceHelloArgs> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Message> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = Message::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("MsgServiceHelloArgs.m", &f_1)?;
    let ret = MsgServiceHelloArgs {
      m: f_1.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("hello_args");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("m", TType::Struct, 1))?;
    self.m.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// MsgServiceHelloResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct MsgServiceHelloResult {
  result_value: Option<Message>,
}

impl MsgServiceHelloResult {
  fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<MsgServiceHelloResult> {
    i_prot.read_struct_begin()?;
    let mut f_0: Option<Message> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        0 => {
          let val = Message::read_from_in_protocol(i_prot)?;
          f_0 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = MsgServiceHelloResult {
      result_value: f_0,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("MsgServiceHelloResult");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.result_value {
      o_prot.write_field_begin(&TFieldIdentifier::new("result_value", TType::Struct, 0))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
  fn ok_or(self) -> thrift::Result<Message> {
    if self.result_value.is_some() {
      Ok(self.result_value.unwrap())
    } else {
      Err(
        thrift::Error::Application(
          ApplicationError::new(
            ApplicationErrorKind::MissingResult,
            "no result received for MsgServiceHello"
          )
        )
      )
    }
  }
}

