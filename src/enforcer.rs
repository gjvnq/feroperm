pub use crate::model::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct Enforcer<T> {
    context: Option<Arc<T>>,
    policies_by_id: HashMap<u64, PolicyHead>,
    policies_by_eid: HashMap<String, u64>,
    policies_by_verb: HashMap<String, HashSet<u64>>,
    policies_by_res: HashMap<String, HashSet<u64>>,

    rule_by_id: HashMap<u64, PolicyRule>,
    rule_by_eid: HashMap<String, u64>,
    rule_by_policy_id: HashMap<u64, HashSet<u64>>,
}

