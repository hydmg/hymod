pub enum ServerCommand {
    List,
    Add { name: String },
    Show { name: String },
    Test { name: String },
}

pub fn execute(cmd: ServerCommand) {
    let _ = cmd;
    todo!()
}
