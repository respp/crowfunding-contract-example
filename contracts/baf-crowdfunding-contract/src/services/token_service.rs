use soroban_sdk::{Address, Env, token::{self}};

use crate::storage::{token::get_token, types::error::Error};

/// TokenService - Maneja todas las operaciones relacionadas con tokens
/// Este es un patrón profesional que separa la lógica de negocio del storage
pub struct TokenService;

impl TokenService {
    /// Transfiere tokens entre direcciones
    /// Esta función encapsula toda la lógica de transferencia de tokens
    pub fn transfer(env: &Env, from: &Address, to: &Address, amount: &i128) -> Result<(), Error> {
        // Validaciones de seguridad
        if amount <= &0 {
            return Err(Error::AmountMustBePositive);
        }

        if from == to {
            return Err(Error::CannotTransferToSelf);
        }

        // Obtener el token configurado en el contrato
        let token_id = get_token(env);
        
        // Crear cliente del token
        let token = token::Client::new(env, &token_id);
        
        // Ejecutar la transferencia
        token.transfer(from, to, amount);
        
        Ok(())
    }

    /// Transfiere tokens desde el contrato a una dirección
    pub fn transfer_from_contract(env: &Env, to: &Address, amount: &i128) -> Result<(), Error> {
        Self::transfer(env, &env.current_contract_address(), to, amount)
    }

    /// Transfiere tokens desde una dirección al contrato
    pub fn transfer_to_contract(env: &Env, from: &Address, amount: &i128) -> Result<(), Error> {
        Self::transfer(env, from, &env.current_contract_address(), amount)
    }

    /// Verifica el balance de una dirección
    pub fn balance(env: &Env, address: &Address) -> Result<i128, Error> {
        let token_id = get_token(env);
        let token = token::Client::new(env, &token_id);
        Ok(token.balance(address))
    }
}
