mod bfvm;

const CODE: &'static str = include_str!("../bf_scripts/helloworld.bf");

fn main() {
    let mut vm = bfvm::BFVM::new();
    vm.run(CODE.into());

    #[cfg(debug_assertions)]
    println!("{:?}", vm);
}
