use std::path::PathBuf;
use LitePhoton::input::Input;
use LitePhoton::read_util;
use LitePhoton::read_util::Mode;

fn main() {
    let input = Input::File(
        PathBuf::from("./test_run/test.txt")
    );

    read_util::read_input(Mode::Chunk, input, true, "test");
}
