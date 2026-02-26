use crate::checks;
use super::InstallSequence;

pub fn sequence() -> InstallSequence {
    InstallSequence {
        name: "linux_nix",
        label: "Linux — nix",
        preamble: vec![],
        checks: vec![
            checks::nix::check(),
            checks::nix_flakes::check(),
            checks::curl::nix::check(),
            checks::git::nix::check(),
            checks::git_lfs::nix::check(),
            checks::gxx::nix::check(),
            checks::portaudio::nix::check(),
            checks::libturbojpeg::nix::check(),
            checks::python_dev::nix::check(),
            checks::pre_commit_tool::nix::check(),
            checks::uv::check(),
        ],
    }
}
