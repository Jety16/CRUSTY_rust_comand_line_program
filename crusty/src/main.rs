use std::env;


fn get_args(args: &[String]) -> (&str, &str, &str){
    (&args[1], &args[2], &args[3])
}

fn grep_from_file(){

}

fn grep_from_text(){

}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    match get_args(&args) {
        ("grep", "-f", _) => grep_from_file(),
        ("grep", "-v", _) => grep_from_text(),
        ("grep", _, _) => println!("Use -f to grep from file, -t to grep from text"),
        (_, _, _) => println!("Im sorry i only code greep")
    }
}
