use std::env;


pub fn get_port() -> u16 {
  let mut port: u16 = 8080;

  match env::var("PORT") {
    Ok(p) => {
      match p.parse::<u16>() {
        Ok(n) => {
          port = n;
        }
        Err(_e) => {}
      };
    }
    Err(_e) => {}
  };

  port
}
