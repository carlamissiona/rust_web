use serde::{Deserialize, Serialize};
use serde_json::Result;



#[derive(Serialize, Deserialize)]
    struct Address {
        street: String,
        city: String,
    }
    
    
    
    pub fn check_url(s: i16) -> String{
       
      
            if s == 42 {
                return String::from("Ok")
            } else {
                return String::from("Error")
            }


    }
    pub fn json_serve() -> String{
     
         let address = Address {
            street: "10 Downing Street".to_owned(),
            city: "London".to_owned(),
        };
        // let string = json::to_string(&data).unwrap();
        let j = serde_json::to_string(&address).unwrap();
        return j 
    }
  

