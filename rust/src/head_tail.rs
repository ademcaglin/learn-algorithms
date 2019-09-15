  let vector = vec!["a", "b", "c"];
     
  let head: Option<&&str> = vector.first();
  let tail: &[&str] = &vector[1..];

  //println!("Hayyam Adem: {} {:?}", head.unwrap(), tail );