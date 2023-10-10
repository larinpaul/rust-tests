// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// } 

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn exploramos() {
//         assert_eq!(2 + 2, 4);
//     }

//     #[test]
//     fn otro_test() {
//         panic!("Vamos a hacer que falle");
//     }

// }


//// 23:30

// #[derive(Debug)]

// struct Rectangulo {
//     ancho: u32,
//     alto: u32,
// }

// impl Rectangulo {
//     fn puede_contener(&self, otro: &Rectangulo) -> bool {
//         self.ancho > otro.ancho && self.alto > otro.alto
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn grande_puede_contener_pequeno() {
        
//         let grande = Rectangulo {
//             ancho: 8,
//             alto: 7,
//         };

//         let pequeno = Rectangulo {
//             ancho: 5,
//             alto: 1,
//         };

//         assert!(grande.puede_contener(&pequeno));

//     }

//     #[test]
//     fn pequeno_puede_contener_grande() {
//         let grande = Rectangulo {
//             ancho: 8,
//             alto: 7,
//         };
//         let pequeno = Rectangulo {
//             ancho: 5,
//             alto: 1,
//         };
//         assert!(!pequeno.puede_contener(&grande));
//     }

// }


//// 23:44 

// pub fn incrementar_dos(numero: i32) -> i32 {
//     numero + 2
// }

// #[cfg(test)]

// mod tests {
//     use super::*;

//     // #[test]
//     // fn es_incrementado_en_dos() {
//     //     assert_eq!(5, incrementar_dos(3));
//     // }

//     // #[test]
//     // fn es_incrementado_en_dos() {
//     //     assert_ne!(5, incrementar_dos(3));
//     // }
    
// }


// //// 23:51
// pub fn saludo(nombre: &str) -> String {
//     format!("¡Hola {}!", nombre);
// }

// pub fn saludo(nombre: &str) {
//     // format!("¡Hola {}!", nombre);
//     format!("¡Hola!");
// }


// #[cfg(test)]

// mod tests {
//     use super::*;

//     #[test]
//     fn saludo_contiene_nombre() {
//         let resultado = saludo("Luis");
//         // assert!(resultado.contains("Luis"));
//         assert!(resultado.contains("Luis"), "El saludo no contiene el nombre, cuyo valor era `{}`", resultado);
//     }
// }


//// 23:56 

pub struct Invitado {
    valor: i32,
}

// impl Invitado {
//     pub fn nuevo(valor: i32) -> Invitado {
//         if valor < 1 || valor > 100 {
//             panic!("Invitado debe estar entre 1 y 100, no sirve {}.", valor)
//         }
//         Invitado { valor }
//     }
// }


// impl Invitado {
//     pub fn nuevo(valor: i32) -> Invitado {
//         if valor < 1 {
//             panic!("Invitado debe ser mayor que 1, no sirve {}.", valor);
//         }
//         else if valor > 100 {
//             panic!("Invitado debe ser menor que 100, no sirve {}.", valor);
//         }
//         Invitado { valor }
//     }
// }


#[cfg(test)]

mod tests {
    // use super::*;

    // #[test]
    // #[should_panic]
    // fn mayor_que_100() {
    //     Invitado::nuevo(200);
    // }

    
    // use super::*;

    // #[test]
    // #[should_panic(expected="Invitado debe ser un valor menor o igual que 100")]
    // fn mayor_que_100() {
    //     Invitado::nuevo(200);
    // }


    use super::*;

    #[test]
    fn funciona() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("dos más dos no es igual a 4"))
        }
    }

}

