#[derive(Debug, Default, Clone)]
pub struct PolicyRule {
    //! Internal numeric id. Don't touch it.
    id: u64,
    //! External numeric id. Used by applications.
    eid: String,
    //! Internal numeric id for the policy head. Don't touch it.
    head_id: u64,
    //! Optional field.
    desc: String,
    code_src: String
}

#[derive(Debug, Default, Clone)]
pub struct PolicyHead {
    //! Internal numeric id. Don't touch it.
    id: u64,
    //! External numeric id. Used by applications.
    eid: String,
    //! Optional field.
    name: String,
    //! Optional field.
    desc: String,
    //! Usually in uppercase. E.g. "ADD", "NEW", "DEL", "SET", etc.
    verb: String,
    //! List of names of the types of the resources that will be acted upon.
    res_types: Vec<String>,
    //! Default value
    allow_by_default: bool 
}
