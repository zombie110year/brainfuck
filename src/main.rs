mod bfvm;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        let script_file = &args[1];
        let code = std::fs::read_to_string(script_file).unwrap();
        let mut vm = bfvm::BFVM::new();
        vm.run(code.into());

        // debug 模式编译时 debug_assertions 被启用, release 时则没有
        #[cfg(debug_assertions)]
        println!("{:?}", vm);
    } else {
        println!("Usage: {} <script.bf>", args[0])
    }
}
