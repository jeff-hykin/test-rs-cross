use anyhow::Result;
use std::process::Command;

use crate::{checks, config::Config};
use super::{InstallSequence, Step};

pub fn sequence() -> InstallSequence {
    InstallSequence {
        name: "linux_apt",
        label: "Linux — apt",
        preamble: vec![Step {
            label: "sudo apt-get update",
            run: apt_update,
        }],
        checks: vec![
            checks::curl::apt::check(),
            checks::gxx::apt::check(),
            checks::portaudio::apt::check(),
            checks::git_lfs::apt::check(),
            checks::libturbojpeg::apt::check(),
            checks::python_dev::apt::check(),
            checks::pre_commit_tool::apt::check(),
            checks::uv::check(),
        ],
    }
}

fn apt_update(_cfg: &Config) -> Result<()> {
    let status = Command::new("sudo")
        .args(["apt-get", "update"])
        .status()?;
    if !status.success() {
        anyhow::bail!("apt-get update failed");
    }
    Ok(())
}
