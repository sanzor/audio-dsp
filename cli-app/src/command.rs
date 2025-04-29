pub enum Command{
    Load{name:String,filname:String},
    Save{name:String,filename:String},
    Delete{name:Option<String>},
    Ls,
    Info{name:Option<String>},
    Gain{name:Option<String>,gain:f32,mode:Option<RunMode>},
    LowPass{name:Option<String>,cutoff:f32},
    HighPass{name:Option<String>,cutoff:f32},
    Normalize{name:Option<String>,mode:Option<RunMode>},
    Exit
}

pub struct CommandResult{

}
pub enum RunMode{
    Simple,
    Parallel{parallelism:u8}
}