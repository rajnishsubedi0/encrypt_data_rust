mod modules;
use modules::my_modules::do_io;

use std::io::Write;


fn main(){
    
    //File paths
    let enc_file_path="/home/rkant/NewActixProject/exercise_project/enc.txt".to_string();
    let data_file_path = "/home/rkant/NewActixProject/exercise_project/file.txt".to_string();
    
    //Reading and Encrypting file from data_file_path directory
    let encrypt=do_io::encrypted_value(data_file_path);
       
    
    let mut open_file_to_write=do_io
            ::create_file_and_open(enc_file_path.clone()).unwrap();


    // Writing encrypted data
    open_file_to_write.write_all(&encrypt.as_slice());    

    
}