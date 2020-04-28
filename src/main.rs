use std::{
    env,
    fs::File,
    io::prelude::*,
    os::unix::process::CommandExt,
    path::PathBuf,
    process,
    process::Command,
    str,
};

fn main() {
    let mut arguments: Vec<String> = vec![];
    arguments.extend(env::args().skip(1).collect::<Vec<String>>().iter().cloned());
    let current_exe = env::current_exe().expect("Couldn't get the name of the binary");
    let script_name = current_exe
        .file_name()
        .expect("Couldn't get the name of the binary without the path");

    let mut brew_location = PathBuf::from(
        command_output("brew".to_string(), vec!["--prefix".to_string()])
            .expect("Couldn't get the path to the brew directory")
            .trim(),
    );

    let mut sops_key = dirs::home_dir().map(PathBuf::from).expect("No home?!");
    sops_key.push(".config");
    sops_key.push("gcloud");
    sops_key.push("application_sops_credentials.json");

    env::set_var("GOOGLE_APPLICATION_CREDENTIALS", sops_key);

    let mut home = dirs::home_dir().expect("No home?!");

    home.push(".config");
    home.push("git-duet");
    home.push("authors.yml");

    let decrypted_author_file = command_output(
        "sops".to_string(),
        vec![
            "-d".to_string(),
            home.to_str()
                .expect("Couldn't convert config home to string")
                .to_string(),
        ],
    )
    .expect("Couldn't decrypt the authors file");

    let mut dir = env::temp_dir();
    dir.push("authors.yml");

    let mut f = File::create(&dir).expect("Couldn't create temporary file");
    f.write_all(&decrypted_author_file.as_bytes())
        .expect("Couldn't write to temporary file");

    env::set_var("GIT_DUET_AUTHORS_FILE", dir);
    brew_location.push("bin/");
    brew_location.push(script_name);
    let cmd = brew_location;
    let err = process::Command::new(cmd).args(arguments).exec();
    panic!("panic!: {}", err)
}

fn command_output(command: String, args: Vec<String>) -> Result<String, String> {
    let output = Command::new(command).args(args).output();
    output
        .or_else(|err| Err(format!("Error: {}", err)))
        .and_then(|output| {
            str::from_utf8(&output.stdout)
                .or_else(|message| Err(message.to_string()))
                .and_then(|output| Ok(output.to_string()))
        })
}
