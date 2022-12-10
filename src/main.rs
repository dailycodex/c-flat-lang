use cflat;
mod args;
fn main() {
    let settings = args::cargs();
    if settings.debug_graph {
        eprintln!("not implemented yet!");
        std::process::exit(1);
    }
    let Some(filename) = settings.filename else {
        eprintln!("No file given");
        return;
    };
    let Ok(src) = std::fs::read_to_string(&filename) else {
        eprintln!("failed to open '{filename}'");
        return;
    };

    let debug_token = cflat::TokenDebug::from(settings.debug_token);
    let debug_ast = cflat::ParseDebug::from(settings.debug_ast);
    let _ast = cflat::parse(&src, debug_token, debug_ast);
}
