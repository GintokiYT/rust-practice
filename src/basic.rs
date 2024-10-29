#[derive(Debug)]
struct Person {
	name: String,
	age: u32,
}
#[derive(Debug)]
struct Animal {
	id: u8,
	name: String
}

fn main() {
	let name = "Renato";
	let last_name: &str = "Navarro";
	println!("Hola {} {}", name, last_name);

	let num = 3;

	if num % 2 == 0 {
		println!("Es par");
	} else {
		println!("Es impar");
	}

	let mut first_name = "Chris";
	println!("first_name: {}", first_name);
	first_name = "Jhon";
	println!("mut first_name: {}", first_name);

	let currency = "PEN";

	match currency {
		"PEN" => println!("Nuevo soles"),
		"USD" => println!("Dolar americano"),
		_ => println!("Otras monedas")
	}

	if currency == "PEN" || currency == "Soles" {
		println!("Moneda de Perú");
	}

	let numbers = [1,2,3,4,5,6,7,8,9,0];

	println!("List de numeros: {:?}", numbers);
	println!("El primer valor es: {}", numbers[0]);
	println!("El tamaño del arreglo de numeros es: {}", numbers.len());

	let numbers_two = [1; 5];
	println!("List de numeros segundo: {:?}", numbers_two);

	let matriz = [[1,2,3], [4,5,6], [7,8,9]];
	println!("Lista de la matriz: {:?}", matriz);

	let person_one = Person {
		name: String::from("Renato"),
		age: 25
	};

	println!("person_one name: {}", person_one.name );
	println!("person_one age: {}", person_one.age );

	let _person_two = Person {
		name: String::from("Christian"),
		age: 26
	};

	let person_three = Person {
		name: String::from("Jhon"),
		age: 26
	};

	println!("el _ se usa para decir que la variable se crea pero no se va usar");

	let list_persons = [ person_one, person_three ];

	println!("Arreglo de personas: {:?}", list_persons);
	println!("Arreglo de personas formateada: {:#?}", list_persons);

	let garfield = Animal {
		id: 1,
		name: String::from("Garfield")
	};

	println!("Objeto Garfield: {:?}", garfield);
	println!("Garfield id: {}", garfield.id);
	println!("Gardield name: {}", garfield.name);

	say_hello("Jeyson");
	say_hello("Frank");

	let frutas = [ "Manzana", "Pera", "Naranja", "Fresa" ];
	println!("Lista de toda las frutas: {:?}", frutas);

	// Iterar sobre un arreglo de manera inmutable
	for fruta in frutas.iter() {
		println!("Fruta: {}", fruta);
	}

	let mut number_two = [1, 2, 3 ,4 ,5];

	for number in number_two.iter_mut() {
		*number += 1;
		println!("El nuevo valor es: {}", number);
	}

	println!("Lista de number_two actualizada: {:?}", number_two);

	for i in 0..frutas.len() {
		println!("{}. {}", i + 1, frutas[i]);
	}

	for (index, fruta) in frutas.iter().enumerate() {
		println!("{} en la posicion {}", fruta, index);
	}

}

fn say_hello(name: &str) {
	println!("Hola {}", name);
}