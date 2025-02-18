use std::io;
use std::process::Command;

use log::error;

use crate::domain::model::command::CommandList;

pub struct LocalConnector;

impl LocalConnector {
    pub fn execute(commands: Vec<CommandList>) -> Result<Vec<String>, io::Error> {
        let mut output_vec = Vec::new();
        for command in commands {
            let cmd = command.cmd;
            let args = command.args;
            let command_str = format!("{} {}", cmd, args.join(" "));
            println!("Executing command: {}", command_str);
            let output = Command::new(cmd).args(args).output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        let result = String::from_utf8_lossy(&output.stdout).into_owned();
                        output_vec.push(result.clone());
                        println!("{}", &result);
                        output.stderr;
                    } else {
                        let err = String::from_utf8_lossy(&output.stderr).into_owned();
                        error!("{}", err);

                        return Err(io::Error::new(io::ErrorKind::Other, err));
                    }
                }
                Err(e) => {
                    error!("{}", e.to_string());

                    return Err(io::Error::new(io::ErrorKind::Other, e));
                }
            }
        }

        Ok(output_vec)
    }
}
