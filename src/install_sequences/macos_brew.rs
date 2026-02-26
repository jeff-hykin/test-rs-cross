use crate::checks;
use super::InstallSequence;

pub fn sequence() -> InstallSequence {
    InstallSequence {
        name: "macos_brew",
        label: "macOS — Homebrew",
        preamble: vec![],
        checks: vec![
            checks::brew::check(),
            checks::curl::brew::check(),
            checks::git::brew::check(),
            checks::git_lfs::brew::check(),
            checks::gxx::brew::check(),
            checks::portaudio::brew::check(),
            checks::libturbojpeg::brew::check(),
            checks::python_dev::brew::check(),
            checks::pre_commit_tool::brew::check(),
            checks::uv::check(),
        ],
    }
}
