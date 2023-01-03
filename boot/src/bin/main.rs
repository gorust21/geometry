use async_std::task;
use async_graphql::{Result};

use boot::run;

fn main() -> Result<()> {
    task::block_on(run())
}