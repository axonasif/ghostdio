use std::env;
use std::fs::File;
use std::io::BufReader;


fn main() {
    let args: Vec<String> = env::args().collect(); // Collect all args
    
    if args.len() == 2 {  //Check if a arg is supplied
    
        let filename = &args[1];
    
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap(); //Opens a audio device for outpu

        let val_file = File::open(filename);  //Opens the file
        
        match val_file {  //Check if file open was possible
            Ok(file) => {
                let val_sink = stream_handle.play_once(BufReader::new(file)); //Plays the media
                
                match val_sink { //Check if the media was playable
                    Ok(pipe) => while !pipe.empty() {}, //Loops while the pipe or sound channel not empty
                    Err(_er) => println!("Wrong File type")
                }
            },
            Err(_er) => println!("Cannot Open File")
        }
    
    }else{
        println!("Please supply a media as argument");
    }
}
