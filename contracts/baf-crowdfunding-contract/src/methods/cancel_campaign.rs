use soroban_sdk::{Address, Env};

use crate::{
    events,
    storage::{
        campaign::{has_campaign, get_campaign, remove_campaign},
        contribution::{get_all_contributors, get_contribution, remove_contribution},
        types::error::Error
    },
    methods::token::token_transfer
};

pub fn cancel_campaign(env: &Env, creator: Address) -> Result<(), Error> {
    // Verificar que el creador esté autenticado
    creator.require_auth();

    // Verificar si la campaña existe
    if !has_campaign(env, &creator) {
        return Err(Error::CampaignNotFound);
    }

    // Obtener campaña
    let campaign = get_campaign(env, &creator)?;
    
    // Verificar que la campaña no esté completada
    if campaign.total_raised >= campaign.goal {
        return Err(Error::CampaignAlreadyCompleted);
    }

    // Si hay fondos recaudados, devolverlos a todos los contribuyentes
    if campaign.total_raised > 0 {
        // Obtener lista de contribuyentes
        let contributors = get_all_contributors(env, &creator)?;
        
        // Devolver cada contribución
        for contributor in contributors.iter() {
            // Obtener el monto de la contribución
            let amount = get_contribution(env, &creator, &contributor)?;
            
            // Transferir tokens de vuelta al contribuyente
            token_transfer(
                &env,
                &env.current_contract_address(),
                &contributor,
                &amount
            )?;
            
            // Remover la contribución del storage
            remove_contribution(env, &creator, &contributor);
            
            // Emitir evento de reembolso
            events::refund::refund(&env, &contributor, &creator, &amount);
        }
    }

    // Remover la campaña
    remove_campaign(env, &creator);

    // Emitir evento
    events::campaign::campaign_cancelled(&env, &creator);

    Ok(())
}