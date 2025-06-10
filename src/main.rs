#![feature(proc_macro_hygiene, decl_macro)]

use nu_protocol::debugger::WithoutDebug;
use rocket::response::status::BadRequest;

mod context;
use context::create_sandboxed_context;

use nu_engine::eval_expression_with_input;
use nu_parser::parse;
use nu_protocol::ast::Block;
use nu_protocol::engine::{Call, EngineState, Stack, StateWorkingSet};
use nu_protocol::{Config, IntoSpanned, PipelineData, ShellError, Signals, Span, Value};
use std::error::Error;
use std::path::PathBuf;
use std::sync::{mpsc, Arc};
use std::{env, thread};

struct Handler;

#[derive(Debug)]
enum HandlerError {
    FormatError,
    ShellError(ShellError),
    TimeoutError,
}

fn parse_single_message(msg: &str) -> Result<&str, HandlerError> {
    let msg = msg.trim();

    if let Some(msg_content) = msg.strip_prefix("`").and_then(|msg| msg.strip_suffix("`")) {
        return Ok(msg_content);
    }

    Err(HandlerError::FormatError)
}

// fn parse_block_message<'a>(msg: &'a str) -> Result<&'a str, HandlerError> {
//     let msg = msg.trim();

//     if let Some(msg) = msg
//         .strip_prefix("nu!\n```")
//         .and_then(|msg| msg.strip_suffix("```"))
//     {
//         return Ok(msg);
//     }

//     return Err(HandlerError::FormatError);
// }

fn parse_message<'a>(msg: &'a str) -> Result<&'a str, HandlerError> {
    parse_single_message(msg)
}

fn parse_command(
    engine_state: &mut EngineState,
    stack: &mut Stack,
    source: &[u8],
) -> Result<Arc<Block>, HandlerError> {
    let mut working_set = StateWorkingSet::new(engine_state);

    
    let output = parse(
        &mut working_set,
        Some("entry #0"), // format!("entry #{}", entry_num)
        source,
        false,
    );
    
    let delta = working_set.render();

    engine_state
        .merge_delta(delta)
        .map_err(HandlerError::ShellError)?;

    engine_state
        .merge_env(stack)
        .map_err(HandlerError::ShellError)?;

    Ok(output)
}

fn eval_block(
    engine_state: &EngineState,
    stack: &mut Stack,
    block: &Block,
) -> Result<String, ShellError> {
    let mut input = PipelineData::with_span(PipelineData::empty(), Span { start: 0, end: 0 });
    let mut result = "".to_string();




    for pipeline in block.pipelines.iter() {
        for elem in pipeline.elements.iter() {
            input = eval_expression_with_input::<WithoutDebug>(engine_state, stack, &elem.expr, input)?
        }

        match input {
            PipelineData::Value(Value::Nothing { .. }, ..) => {},
            _ => {
                 let config = engine_state.get_config();

                // Drain the input to the screen via tabular output
                match engine_state.find_decl("table".as_bytes(), &[]) {
                    Some(decl_id) => {
                        let mut call = nu_protocol::ast::Call::new(Span::unknown());
                        call.add_named((String::from("expand").into_spanned(Span::unknown()), None, None));
                        let table = engine_state.get_decl(decl_id).run(
                            engine_state,
                            stack,
                            &Call::from(&call),
                            input,
                        )?;

                        for item in table {
                            if let Value::Error { error, internal_span: _ } = item {
                                return Err(*error);
                            }

                            result.push_str(&item.to_expanded_string("\n", config));
                            result.push('\n');
                        }
                    }
                    None => {
                        for item in input {
                            if let Value::Error { error, internal_span: _ } = item {
                                return Err(*error);
                            }

                            result.push_str(&item.to_expanded_string("\n", config));
                            result.push('\n');
                        }
                    }
                };
            }
        }

        input = PipelineData::empty()
    }

    Ok(result)
}

fn handle_message(content: String) -> Result<String, HandlerError> {
    let source = parse_message(&content)?.as_bytes();

    let mut sandbox = create_sandboxed_context();
    let mut stack = Stack::new();

    let path = nu_path::expand_tilde("~");
  
    stack.add_env_var(
        "PWD".into(),
        Value::string(path.to_string_lossy(), Span::unknown()),
    );

    let block = parse_command(&mut sandbox, &mut stack, source)?;
    

    eval_block(&sandbox, &mut stack, &block).map_err(HandlerError::ShellError)
}

fn try_handle_message(content: &str) -> Result<String, HandlerError> {
    let (sender, receiver) = mpsc::channel();

    let cloned_content = content.to_string();

    let message_handling_thread =
        thread::spawn(move || sender.send(handle_message(cloned_content)));

    match receiver.recv_timeout(std::time::Duration::new(1000, 0)) {
        Ok(result) => result,
        Err(mpsc::RecvTimeoutError::Timeout) => {
            drop(receiver);
            drop(message_handling_thread);
            // took more than 5 seconds
            Err(HandlerError::TimeoutError)
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let result = try_handle_message("`3 + 4`");

        match result {
            Ok(result) => assert_eq!(result, "7\n".to_owned()),
            Err(error) => panic!("{error:?}"),
        }
    }

    #[test]
    fn parse_add() {
        let result = parse_message("`3 + 4`");
        assert!(result.is_ok(), "{result:?}");
        assert_eq!(result.unwrap(), "3 + 4");
    }
}

fn message(msg: &str) -> String {
    let msg = msg.trim();
    match try_handle_message(msg) {
        Ok(res) => match res.is_empty() {
            true => "*Empty*".to_string(),
            false => res.to_string(),
        },

        Err(HandlerError::FormatError) => {
            "Improper formatting. Format as \"`[command]`\"".to_string()
        }
        Err(HandlerError::ShellError(shell_error)) => {
            format!("ShellError: {shell_error:#?}")
        }
        Err(HandlerError::TimeoutError) => "运行超时 (5s).".to_string(),
    }
}

#[macro_use]
extern crate rocket;


#[post("/", data = "<input>")]
fn hello(input: String) -> Result<String, BadRequest<String>> {
    if !input.is_empty() {
        return Ok(message(&input));
    }
    Err(BadRequest("Empty value".into()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
