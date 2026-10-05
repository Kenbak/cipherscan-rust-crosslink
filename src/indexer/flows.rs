//! Flow analysis logic
//!
//! Analyzes transactions to detect and classify shielded flows.

use crate::models::{Transaction, ShieldedFlow, FlowType, Pool};

/// Flow analyzer
pub struct FlowAnalyzer;

impl FlowAnalyzer {
    /// Analyze a transaction and extract all flows
    pub fn analyze(tx: &Transaction) -> Vec<ShieldedFlow> {
        ShieldedFlow::from_transaction(tx)
    }

    /// Classify a transaction's primary flow type
    pub fn classify(tx: &Transaction) -> Option<FlowType> {
        if tx.is_coinbase() {
            return None;
        }

        // Check for pool migration first
        if tx.vin_count == 0 && tx.vout_count == 0 && tx.has_shielded() {
            let balances = [tx.sapling_value_balance, tx.orchard_value_balance, tx.ironwood_value_balance];
            if balances.iter().any(|b| *b > 0) && balances.iter().any(|b| *b < 0) {
                return Some(FlowType::PoolMigration);
            }
        }

        // Fully shielded
        if tx.is_fully_shielded() {
            return Some(FlowType::FullyShielded);
        }

        // Shielding
        if tx.is_shielding() {
            return Some(FlowType::Shield);
        }

        // Deshielding
        if tx.is_deshielding() {
            return Some(FlowType::Deshield);
        }

        None
    }

    /// Determine which pools are involved in a transaction
    pub fn involved_pools(tx: &Transaction) -> Vec<Pool> {
        let mut pools = Vec::new();

        if tx.joinsplit_count > 0 {
            pools.push(Pool::Sprout);
        }
        if tx.sapling_spends > 0 || tx.sapling_outputs > 0 || tx.sapling_value_balance != 0 {
            pools.push(Pool::Sapling);
        }
        if tx.orchard_actions > 0 || tx.orchard_value_balance != 0 {
            pools.push(Pool::Orchard);
        }

        if tx.ironwood_actions > 0 || tx.ironwood_value_balance != 0 {
            pools.push(Pool::Ironwood);
        }

        pools
    }

    /// Calculate the net flow amount for a pool
    pub fn net_flow_amount(tx: &Transaction, pool: Pool) -> i64 {
        match pool {
            Pool::Sprout => 0, // Would need JoinSplit parsing
            Pool::Sapling => tx.sapling_value_balance,
            Pool::Orchard => tx.orchard_value_balance,
            Pool::Ironwood => tx.ironwood_value_balance,
        }
    }

    /// Check if a transaction involves cross-pool activity
    pub fn is_cross_pool(tx: &Transaction) -> bool {
        let pools = Self::involved_pools(tx);
        pools.len() > 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tx() -> Transaction {
        Transaction {
            txid: "test".to_string(),
            block_height: 1000,
            block_hash: "blockhash".to_string(),
            version: 5,
            lock_time: 0,
            expiry_height: Some(1100),
            size: 500,
            vin_count: 1,
            vout_count: 0,
            transparent_value_in: 10000000,
            transparent_value_out: 0,
            joinsplit_count: 0,
            sapling_spends: 0,
            sapling_outputs: 1,
            orchard_actions: 0,
            ironwood_actions: 0,
            ironwood_value_balance: 0,
            staking_value_balance: 0,
            sapling_value_balance: -9999000,
            orchard_value_balance: 0,
            fee: Some(1000),
            vin: vec![],
            vout: vec![],
            staking_action_type: None,
            staking_bond_key: None,
            staking_delegatee: None,
            staking_amount_zats: None,
        }
    }

    #[test]
    fn ironwood_only_transaction_is_fully_shielded_and_identifies_its_pool() {
        let mut tx = create_test_tx();
        tx.vin_count = 0;
        tx.vout_count = 0;
        tx.sapling_outputs = 0;
        tx.sapling_value_balance = 0;
        tx.ironwood_actions = 2;
        tx.ironwood_value_balance = 10_000;
        assert_eq!(FlowAnalyzer::classify(&tx), Some(FlowType::FullyShielded));
        assert_eq!(FlowAnalyzer::involved_pools(&tx), vec![Pool::Ironwood]);
        assert_eq!(FlowAnalyzer::net_flow_amount(&tx, Pool::Ironwood), 10_000);
    }

    #[test]
    fn sapling_to_ironwood_is_a_pool_migration() {
        let mut tx = create_test_tx();
        tx.vin_count = 0;
        tx.vout_count = 0;
        tx.sapling_value_balance = 100_000;
        tx.ironwood_actions = 2;
        tx.ironwood_value_balance = -90_000;
        assert_eq!(FlowAnalyzer::classify(&tx), Some(FlowType::PoolMigration));
        let flows = FlowAnalyzer::analyze(&tx);
        assert_eq!(flows.len(), 1);
        assert_eq!(flows[0].pool, "mixed");
        assert_eq!(flows[0].amount, 10_000);
    }

    #[test]
    fn test_classify_shielding() {
        let tx = create_test_tx();
        assert_eq!(FlowAnalyzer::classify(&tx), Some(FlowType::Shield));
    }

    #[test]
    fn test_involved_pools() {
        let mut tx = create_test_tx();
        tx.orchard_actions = 2;

        let pools = FlowAnalyzer::involved_pools(&tx);
        assert!(pools.contains(&Pool::Sapling));
        assert!(pools.contains(&Pool::Orchard));
    }
}
