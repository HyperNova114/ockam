use crate::{
    util::{exitcode, get_final_element},
    CommandGlobalOpts,
};
use clap::Args;

#[derive(Clone, Debug, Args)]
pub struct SetDefaultNodeCommand {
    /// Name of the Node
    pub name: String,
}

impl SetDefaultNodeCommand {
    pub fn run(opts: CommandGlobalOpts, command: SetDefaultNodeCommand) {
        let name = get_final_element(&command.name);
        match opts.config.select_node(name) {
            Some(_) => {
                opts.config.set_default_node(&name.to_string());
            }
            None => {
                eprintln!("Node ({}) is not registered yet", command.name);
                std::process::exit(exitcode::CANTCREAT);
            }
        }
    }
}
