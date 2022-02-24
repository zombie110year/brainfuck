mod bfvm;


fn main() {
    let mut vm = bfvm::BFVM::new();
    vm.run(",>,>,<<+.>+.>+.".into());
    println!("{:?}", vm);
}
