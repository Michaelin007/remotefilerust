use std::{error::Error, io::Read, net::TcpStream, path::Path};

use ssh2::Session;

//Connection details
const HOST: &str = "HOST_IP";
const USERNAME: &str = "USERNAME";
const PATH_TO_KEY: &str = "PATH_TO_KEY";
const PORT: i32 = 22;
fn main() -> Result<(), Box<dyn Error>> {

    let address = format!("{}:{}", HOST, PORT);
    let tcp = TcpStream::connect(&address)?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);

    //Disable host key checking
    session.handshake()?;

    session.userauth_pubkey_file(USERNAME, None, Path::new(PATH_TO_KEY), None)?;

    // Open a channel
    let mut channel = session.channel_session()?;

    // Execute command to read file
    channel.exec("cat /home/opc/file.txt")?;

    // Read the output
    let mut file_content = String::new();
    channel.read_to_string(&mut file_content)?;

    //Print the file content
    println!("{}",file_content);

    //Close the channel
    channel.wait_close()?;
    println!("Exit status: {}", channel.exit_status()?);
    

    Ok(())
}
