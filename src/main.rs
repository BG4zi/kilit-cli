mod crypto;
use clap::{Arg, App};
use rustyline::{error::ReadlineError, Editor};
use rustyline::completion::{Completer, Pair};
use rustyline::hint::Hinter;
use rustyline::Helper;
use rustyline::highlight::Highlighter;
use rustyline::validate::Validator;
use rpassword;
use std::fs::OpenOptions;
use std::env;
use std::process::Command;

// Struct for parsing Arguments
struct RunArgs {
	 pub conf: String,
	 pub prompt: String,
}

// Struct for parsing each line of the file
struct Block {
	 pub product: String,
	 pub name: String,
 	 pub passwd: String,
}


// Main autocompleter
struct MainCompleter;
impl Completer for MainCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> rustyline::Result<(usize, Vec<Pair>)> {
        let commands = vec!["create", "go", "conf", "exit"];
        let completions: Vec<Pair> = commands.iter()
            .filter(|&&cmd| cmd.starts_with(line))
            .map(|&cmd| Pair {
                display: cmd.to_string(),
                replacement: cmd.to_string(),
            })
            .collect();
        Ok((0, completions))
    }
}

// Autocompleter
struct GoCompleter;
impl Completer for GoCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> rustyline::Result<(usize, Vec<Pair>)> {
        let commands = vec!["list", "add", "change", "get", "del", "exit"];
        let completions: Vec<Pair> = commands.iter()
            .filter(|&&cmd| cmd.starts_with(line))
            .map(|&cmd| Pair {
                display: cmd.to_string(),
                replacement: cmd.to_string(),
            })
            .collect();
        Ok((0, completions))
    }
}


// Main helper struct that includes all required traits
struct MainHelper;
impl Helper for MainHelper {}
impl Completer for MainHelper {
    type Candidate = Pair;
    fn complete(&self, line: &str, pos: usize, ctx: &rustyline::Context) -> rustyline::Result<(usize, Vec<Pair>)> {
        MainCompleter.complete(line, pos, ctx)
    }
}

// ...
impl Hinter for MainHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context) -> Option<String> {
        None
    }
}

impl Highlighter for MainHelper {}
impl Validator for MainHelper {}

// helper struct that includes all required traits
struct GoHelper;
impl Helper for GoHelper {}
impl Completer for GoHelper {
    type Candidate = Pair;
    fn complete(&self, line: &str, pos: usize, ctx: &rustyline::Context) -> rustyline::Result<(usize, Vec<Pair>)> {
        GoCompleter.complete(line, pos, ctx)
    }
}

// ...
impl Hinter for GoHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context) -> Option<String> {
        None
    }
}

impl Highlighter for GoHelper {}
impl Validator for GoHelper {}


// Function that parses args as the struct
fn get_args() -> RunArgs {

	 let home_dir = env::var("HOME").expect("HOME environment variable not set");
	 let default_conf = home_dir + "/.kilit";
	 
	 let matches = App::new("kilit-cli")
		  .version("1.0")
		  .author("Burak Gazi Chetin")
		  .about("A simple CLI based password manager")
		  .arg(Arg::new("conf")
				 .short('c')
				 .long("conf")
				 .takes_value(true)
				 .required(false)
				 .default_missing_value("$HOME/.kilit")
				 .default_value(&default_conf)
				 .help("Set the configuration file path \n\t(Example usage: \"./kilit -c \"$HOME/.mypassfile\"\") and don't use ~ for home dir it crashes the tool"))
		  .arg(Arg::new("prompt")
				 .short('p')
				 .long("prompt")
				 .takes_value(true)
				 .required(false)
				 .default_missing_value("")
				 .default_value("")				 
			 .help("Instead of opening a new shell the commands will be written using this argument \n\t(Example usage kilit -c \"~/.adana\" -p \"go passwd  list name bgc\" \n                  kilit -c \"~/.adana\" -p \"create passwd\")"))
		  .get_matches();
     	 	 
	 let conf = matches
		  .value_of("conf")
		  .unwrap()
		  .to_owned();

	 let prompt = matches
		  .value_of("prompt")
		  .unwrap()
		  .to_owned();
	 
	 RunArgs {conf, prompt}
}


// Edits a specified line
fn edit(file_path: &String, _args: &Vec<String>, password: &String) {
	 let lines = crypto::load_encrypted_from_file(file_path).unwrap();
	 let mut blocks: Vec<Block> = Vec::new();	 
	 for line in &lines {
		  let data =  crypto::decrypt_data(line, password).unwrap();
		  if data == "OK" {
				continue;
		  }
		  let parsed: Vec<String> = data.split(" : ").map(|s| s.to_string()).collect();
		  
		  blocks.push(Block { product: parsed[0].clone(),
									 name: parsed[1].clone(),
									 passwd: parsed[2].clone()
		  });
	 }
	 for (i, line) in blocks.iter().enumerate() {
		  println!("------------------------------");
		  println!("ID: {i}");
		  println!("Product: {}", line.product);
		  println!("Name: {}", line.name);
	 }
	 let mut rl = Editor::new().unwrap();
	 rl.set_helper(Some(GoHelper));
	 let id: usize = rl.readline("Choose (ID): ").unwrap().to_string().parse().unwrap();
	 let new_name = rl.readline("New Name (If you enter nothing will be the same): ").unwrap();	 
	 let new_passwd = rpassword::prompt_password("Password (If you enter nothing will be the same): ").unwrap();
	 if new_name != "" {
	 	  blocks[id].name = new_name;
	 }
	 if new_passwd != "" {
		  blocks[id].passwd = new_passwd;
	 }

	 let data = blocks[id].product.clone() + " : " + &blocks[id].name + " : " + &blocks[id].passwd;
	 let _text =  crypto::new_data(&data, &password, &file_path);
	 let cmd = format!("sed -i '{}d' {}", id+2, &file_path);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("Command Couldn't run");

    if output.status.success() {
		  println!("Editted")		 
    } else {
        eprintln!("Hata: {:?}", String::from_utf8_lossy(&output.stderr));
    }
}

// Lists the lines according to an SQL-esque prompt
fn list(file_path: &String, args: &Vec<String>, password: &String) -> Result<(), ()> {

	 let lines = crypto::load_encrypted_from_file(file_path).unwrap();	 
	 if lines.len() < 2 {
		  println!("Nothing to list!");
		  return Ok(());
	 }	 
	 let mut blocks: Vec<Block> = Vec::new();	 
	 for line in &lines {
		  let data =  crypto::decrypt_data(line, password).unwrap();
		  if data == "OK" {
				continue;
		  }
		  let parsed: Vec<String> = data.split(" : ").map(|s| s.to_string()).collect();
		  
		  blocks.push(Block { product: parsed[0].clone(),
									 name: parsed[1].clone(),
									 passwd: parsed[2].clone()
		  });
	 }
	 if args.len() == 0 {
		  for line in blocks {
				println!("------------------------------");
				println!("Product: {}", line.product);
				println!("Name: {}", line.name);
				println!("Password: {}", line.passwd);
		  }	  
	 } else {
		  if args.len() < 2 {
				println!("Unfamiliar syntax");
				args.clone().push("   ".to_string());
				println!("{args:?}");
		  } else {
				if args[0] == "product" {
					 for line in blocks {
						  if line.product == args[1] {
								println!("------------------------------");
								println!("Product: {}", line.product);
								println!("Name: {}", line.name);
								println!("Password: {}", line.passwd);								
						  }
					 }
				} else if args[0] == "name" {
					 for line in blocks {
						  if line.name == args[1] {
								println!("------------------------------");
								println!("Product: {}", line.product);
								println!("Name: {}", line.name);
								println!("Password: {}", line.passwd);																
						  }
					 }
				} else {
					 if args.len() < 2 {
						  println!("Unfamiliar syntax");
					 }
				}
		  }
	 }
	 
	 Ok(())
}

// Adds a new line
fn add(data: &String, password: &String, file_path: &String) {
	 let _text = crypto::new_data(data, &password, &file_path);
}

// Create a new config file with a verification line
fn create(password: String, file_path: &String) {
	 let mut _file = OpenOptions::new()
        .read(true)       // Allow reading
        .write(true)      // Allow writing
        .append(true)
        .create(true)     // Create if it doesn't exist
        .open(file_path)
        .expect("Failed to open or create the file");

    crypto::new_data("OK", &password, &file_path);
	 //	 crypto::save_encrypted_to_file(encrypted_data: &str, filename: &str) 
	 
	 println!("Created Successfully!");
}


// Opens the file and verifies the password
fn go(password: String, file_path: &String) {
	 let  lines = crypto::load_encrypted_from_file(file_path).unwrap();
	 
	 let mut rl = Editor::new().unwrap();
    rl.set_helper(Some(GoHelper));

	 println!("{a}", a = &lines[0]);
	 
	 let verifier = crypto::decrypt_data(&lines[0], &password).unwrap();
	 
	 if verifier == "OK" {
		  println!("You logged in successfully");
		  
		  loop {
				match rl.readline("> ") {
					 Ok(line) => {
						  let  input: Vec<String> = line.trim().split(" ").map(|s| s.to_string()).collect();
						  let mut command = String::new();
						  let mut args: Vec<String> = Vec::new();
						  if input.len() > 0 {
								command = input[0].clone();
						  } 
						  if input.len() > 1 {
								args = input[1..].to_vec();
						  }
						  if command == "exit" {
								println!("Goodbye!");
								break;
						  } else if command == "list" {
								let _ = list(&file_path, &args, &password);
								//								println!("TODO")
								
						  } else if command == "edit" {
								edit(&file_path, &args, &password);
						  } else if command == "add" {
								let product = rl.readline("Product: ").unwrap();
								let name = rl.readline("Name: ").unwrap();
								let pass = rpassword::prompt_password("Password: ").unwrap();

								let data = product + " : " + &name + " : " + &pass;
								
								add(&data, &password, &file_path);

						  } else {
								println!("{command} is not an existant command");
						  }
					 }
					 Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
						  println!("Goodbye!");
						  break;
					 }
					 Err(err) => {
						  println!("Error: {:?}", err);
						  break;
					 }
				}
		  }		  
	 }
}

/* FOR TESTING smth
fn main() {
let password = "a";
let data = rpassword::prompt_password("Password: ").unwrap();

crypto::new_data(&data, password, "/home/ucenik/.kilit");
// Load previously saved encrypted data and decrypt it
let lines = crypto::load_encrypted_from_file("/home/ucenik/.kilit").unwrap();
for enc in lines {
println!("DEC: {datum}", datum = crypto::decrypt_data(&enc, password).unwrap());
    }

}
*/

// MAIN FUNC 
fn main() -> Result<(), ()> {

	 let args = get_args();

	 let file_path = args.conf;

	 // Example usages
	 // kilit -c "~/.adana" -p "go passwd list name bgc"
	 // kilit -c "~/.adana" -p "create passwd"
	 if args.prompt != "" {
		  let prompt: Vec<String> = args.prompt
				.split_ascii_whitespace()
				.map(|s| s.to_string()).collect();

		  if prompt[0] == "go" {
				let passwd = prompt[1].clone();
				if prompt[2] == "list" {
					 list(&file_path, &prompt[3..].to_vec(), &passwd).expect("Insufficient argument");
				} else if prompt[2] == "add" {
					 let product = prompt[3].clone();
					 let name = prompt[4].clone();
					 let pass = prompt[5].clone();

					 let data = product + " : " + &name + " : " + &pass;
					 
					 add(&data, &passwd, &file_path);
				}
				
		  }
		  if prompt[0] == "create" {
				let passwd = prompt[1].clone();
				create(passwd, &file_path);
		  }

	 }

	 else {

		  let mut rl = Editor::new().unwrap();
	 
		  rl.set_helper(Some(MainHelper));
		  
		  loop {
				match rl.readline("> ") {
					 Ok(line) => {
						  let input = line.trim();
						  if input == "exit" {
								println!("Goodbye!");
								break;
						  } else if input == "create" {
								let password = rpassword::prompt_password("Password: ").unwrap();
								create(password, &file_path);
						  } else if input == "go" {
								let password = rpassword::prompt_password("Password: ").unwrap();
								go(password, &file_path);
						  } else {
								println!("{input} is not an existant command");
						  }
					 }
					 Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
						  println!("Goodbye!");
						  break;
					 }
					 Err(err) => {
						  println!("Error: {:?}", err);
						  break;
					 }
				}
		  }
	 }
	 Ok(())
}
