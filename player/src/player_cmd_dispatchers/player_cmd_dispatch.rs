pub trait PlayerCommadDispatch{
    pub fn dispatch(command:PlayerCommand)->Result<(),String>;
}