
use crate::audio_file_content::AudioFileContent;

pub fn read_audio_file(filename:&str)->Result<AudioFileContent,String>{
    println!("Trying to read: {}", filename);
    let file=match std::fs::read(filename){
        Err(err)=> return Err(err.to_string()),
        Ok(file)=>file
    };
    let file_content=AudioFileContent{content:file};
    Ok(file_content)
}