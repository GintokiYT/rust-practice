fn main() {
  match area_de_un_rectangulo(5, 3) {
    Ok(area) => println!("El area del rectangulo es: {}", area),
    Err(err) => println!("Error: {}", err)
  }
  match area_de_un_rectangulo(5, 0) {
    Ok(area) => println!("El area del rectangulo es: {}", area),
    Err(err) => println!("Error: {}", err)
  }
}

fn area_de_un_rectangulo(base: u128, altura: u128) -> Result<u128, &'static str> {
  if base <= 0 {
    return Err("La base debe ser mayor a 0");
  }
  if altura <= 0 {
    return Err("La altura debe ser mayor a 0");
  }
  Ok(base * altura)
}
