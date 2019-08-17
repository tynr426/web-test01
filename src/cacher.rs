 use std::thread;
 use std::time::Duration;

 use std::collections::HashMap;
 use crate::config::Config;

 pub struct Cacher<T>
     where T: Fn(&str) -> Config
 {
     calculation: T,
     value: Option<HashMap<String, Config>>
}

 impl<T> Cacher<T>
     where T: Fn(&str) -> Config
 {
     fn new(calculation: T) -> Self {
         Cacher {
             calculation,
             value: None,
         }
     }

     fn value(&mut self, arg: &str) -> Config {
         let opt=match self.value{
             Some(v) => v,
             None=>HashMap::new()
         };
         match opt.get(arg) {
             Some(v1) => v1,
             None => {
                 let v1 = (self.calculation)(arg);
                 opt.insert(arg.to_string(),v1);
                 v1
             },
         }
     }
}