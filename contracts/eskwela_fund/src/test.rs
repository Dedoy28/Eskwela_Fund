#![cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, Address};
    use soroban_sdk::token::Client as TokenClient;

    // Mock environment setup helper
    fn setup_env<'a>(env: &Env) -> (Address, Address, Address, TokenClient<'a>, EskwelaFundClient<'a>) {
        let admin = Address::generate(env);
        let student = Address::generate(env);
        let token_admin = Address::generate(env);
        
        let token_contract_id = env.register_stellar_asset_contract(token_admin.clone());
        let token_client = TokenClient::new(env, &token_contract_id);
        
        token_client.mint(&token_admin, &admin, &1000);

        let contract_id = env.register_contract(None, EskwelaFund);
        let client = EskwelaFundClient::new(env, &contract_id);

        (admin, student, token_contract_id, token_client, client)
    }

    // Test 1 (Happy path): The MVP transaction executes successfully end-to-end
    #[test]
    fn test_successful_disbursement() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, student, token_id, token_client, client) = setup_env(&env);

        client.init(&admin, &token_id);
        client.disburse(&admin, &student, &500);

        assert_eq!(token_client.balance(&admin), 500);
        assert_eq!(token_client.balance(&student), 500);
    }

    // Test 2 (Edge case): Unauthorized caller attempts to disburse funds
    #[test]
    #[should_panic(expected = "Unauthorized admin")]
    fn test_unauthorized_caller() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, student, token_id, _token_client, client) = setup_env(&env);
        
        let fake_admin = Address::generate(&env);

        client.init(&admin, &token_id);
        client.disburse(&fake_admin, &student, &500);
    }

    // Test 3 (State verification): Assert contract storage reflects correct initialization state
    #[test]
    fn test_state_verification() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, _student, token_id, _token_client, client) = setup_env(&env);

        client.init(&admin, &token_id);
        
        let stored_admin = client.get_admin();
        assert_eq!(stored_admin, admin);
    }

    // Test 4 (Edge case): Admin attempts to disburse more than they hold
    #[test]
    #[should_panic]
    fn test_insufficient_balance() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, student, token_id, _token_client, client) = setup_env(&env);

        client.init(&admin, &token_id);
        client.disburse(&admin, &student, &5000);
    }

    // Test 5 (Edge case): Attempting to re-initialize the contract
    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_double_initialization() {
        let env = Env::default();
        env.mock_all_auths();
        let (admin, _student, token_id, _token_client, client) = setup_env(&env);

        client.init(&admin, &token_id);
        client.init(&admin, &token_id);
    }
}