use tauriless_macro::command;

#[command]
fn argsless_command() {}

#[command]
fn command_with_args(_a: i32, _b: i32) {}

fn main() {
    argsless_command();
    command_with_args(1, 2);
}
