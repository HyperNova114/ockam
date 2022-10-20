use crate::util::{extract_address_value, node_rpc, RpcBuilder};
use crate::{help, CommandGlobalOpts, Result};
use clap::{Args, Subcommand};
use ockam::{Context, TcpTransport};
use ockam_abac::{Action, Expr, Resource};
use ockam_api::nodes::models::policy::Policy;
use ockam_core::api::Request;

const HELP_DETAIL: &str = "";

#[derive(Clone, Debug, Args)]
#[command(hide = help::hide(), after_long_help = help::template(HELP_DETAIL))]
pub struct PolicyCommand {
    #[command(subcommand)]
    subcommand: PolicySubcommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum PolicySubcommand {
    Set {
        /// Node on which to start the tcp inlet.
        #[arg(long, display_order = 900, id = "NODE")]
        at: String,

        #[arg(short, long)]
        resource: Resource,

        #[arg(short, long)]
        action: Action,

        #[arg(short, long)]
        expression: Expr,
    },
    Get {
        /// Node on which to start the tcp inlet.
        #[arg(long, display_order = 900, id = "NODE")]
        at: String,

        #[arg(short, long)]
        resource: Resource,

        #[arg(short, long)]
        action: Action,
    },
}

impl PolicyCommand {
    pub fn run(self, opts: CommandGlobalOpts) {
        node_rpc(rpc, (opts, self))
    }
}

async fn rpc(ctx: Context, (opts, cmd): (CommandGlobalOpts, PolicyCommand)) -> Result<()> {
    let tcp = TcpTransport::create(&ctx).await?;

    match cmd.subcommand {
        #[rustfmt::skip]
        PolicySubcommand::Set { at, resource, action, expression } => {
            let node = extract_address_value(&at)?;
            let bdy = Policy::new(expression);
            let req = Request::post(policy_path(&resource, &action)).body(bdy);
            let mut rpc = RpcBuilder::new(&ctx, &opts, &node).tcp(&tcp)?.build();
            rpc.request(req).await?;
            rpc.is_ok()?
        }
        #[rustfmt::skip]
        PolicySubcommand::Get { at, resource, action } => {
            let node = extract_address_value(&at)?;
            let req = Request::get(policy_path(&resource, &action));
            let mut rpc = RpcBuilder::new(&ctx, &opts, &node).tcp(&tcp)?.build();
            rpc.request(req).await?;
            let pol: Policy = rpc.parse_response()?;
            println!("{}", pol.expression())
        }
    }

    Ok(())
}

fn policy_path(r: &Resource, a: &Action) -> String {
    format!("/policy/{r}/{a}")
}
