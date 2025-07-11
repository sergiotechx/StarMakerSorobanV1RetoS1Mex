#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test_suma_basica() {
    // Configuración del entorno y contrato
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Prueba de suma básica
    let resultado = client.sumar(&10, &20);
    assert_eq!(resultado, 30, "La suma de 10 + 20 debe ser 30");
    
    // Verificar que el resultado anterior se almacenó correctamente
    assert_eq!(client.resultado_anterior(), 30, "El resultado anterior debe ser 30");
}

#[test]
fn test_multiples_sumas() {
    // Configuración del entorno y contrato
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Primera suma
    let resultado1 = client.sumar(&5, &7);
    assert_eq!(resultado1, 12, "La suma de 5 + 7 debe ser 12");
    assert_eq!(client.resultado_anterior(), 12, "El resultado anterior debe ser 12");
    
    // Segunda suma
    let resultado2 = client.sumar(&100, &50);
    assert_eq!(resultado2, 150, "La suma de 100 + 50 debe ser 150");
    assert_eq!(client.resultado_anterior(), 150, "El resultado anterior debe ser 150");
    
    // Tercera suma con números negativos
    let resultado3 = client.sumar(&-30, &10);
    assert_eq!(resultado3, -20, "La suma de -30 + 10 debe ser -20");
    assert_eq!(client.resultado_anterior(), -20, "El resultado anterior debe ser -20");
}

#[test]
fn test_resultado_anterior_inicial() {
    // Configuración del entorno y contrato
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Verificar que el resultado anterior es 0 cuando no se ha realizado ninguna suma
    assert_eq!(client.resultado_anterior(), 0, "El resultado anterior inicial debe ser 0");
    
    // Realizar una suma y verificar que el resultado anterior se actualiza
    let resultado = client.sumar(&42, &58);
    assert_eq!(resultado, 100, "La suma de 42 + 58 debe ser 100");
    assert_eq!(client.resultado_anterior(), 100, "El resultado anterior debe ser 100");
}
