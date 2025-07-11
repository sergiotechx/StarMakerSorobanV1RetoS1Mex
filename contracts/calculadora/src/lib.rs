#![no_std]

use soroban_sdk::{contract, contractimpl, Env, symbol_short, Symbol};

// Definimos una constante para la clave de almacenamiento del resultado
const RESULTADO: Symbol = symbol_short!("RESULTADO");

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    
    pub fn sumar(env: Env, a: i128, b: i128) -> i128 {
        // Calculamos la suma de los dos números
        let resultado = a + b;
        
        // Guardamos el resultado en el almacenamiento del contrato
        env.storage().instance().set(&RESULTADO, &resultado);
        
        // Retornamos el resultado de la suma
        resultado
    }

    pub fn resultado_anterior(env: Env) -> i128 {
        // Recuperamos el resultado anterior del almacenamiento
        // Si no existe un resultado anterior, retornamos 0
        env.storage().instance().get(&RESULTADO).unwrap_or(0)
    }
}

mod test;
