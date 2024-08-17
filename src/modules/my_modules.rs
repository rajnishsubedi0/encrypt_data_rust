pub struct do_io;
impl do_io{
pub fn read_file(path:String) -> String{
    match std::fs::
    read_to_string(path){
        Ok(s) => return s,
        Err(_) =>panic!("Unable to read file") ,
    };
}
pub fn create_file_and_open (dir:String) ->Option<std::fs::File>{
    match std::fs::File::create(dir) {
        Ok(file) => return Some(file),
        Err(_) => panic!("Unable to create or access file")
        
    }

}
pub fn encrypted_value(str:String) ->Vec<u8>{
    return do_io::read_file(str)
         .as_bytes()
         .to_vec()
         .into_iter()
         .map(|x| x+1).collect();
 }
}