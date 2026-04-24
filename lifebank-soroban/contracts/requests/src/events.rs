use crate::types::{BloodRequest, RequestCreatedEvent};
use soroban_sdk::{symbol_short, Address, Env, Symbol};

pub fn emit_initialized(env: &Env, admin: &Address, inventory_contract: &Address) {
    env.events().publish(
        (Symbol::new(env, "initialized"), symbol_short!("v1")),
        (admin.clone(), inventory_contract.clone()),
    );
}

pub fn emit_request_created(env: &Env, request: &BloodRequest) {
    env.events().publish(
        (
            Symbol::new(env, "request_created"),
            request.blood_type,
            symbol_short!("v1"),
        ),
        RequestCreatedEvent {
            request_id: request.id,
            hospital: request.hospital_id.clone(),
            blood_type: request.blood_type,
            quantity_ml: request.quantity_ml,
            urgency: request.urgency.priority(),
            timestamp: request.created_timestamp,
        },
    );
}
