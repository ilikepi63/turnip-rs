pub enum Constraint {
    BinaryConstraint(BinaryConstraint),
}

pub struct BinaryConstraint {
    pub left: String,
    pub right: String,
    pub operator: BinaryConstraintOperator,
}

pub enum BinaryConstraintOperator {
    Gt,
    Lt,
    GtEq,
    LtEq,
    Eq,
    NotEq,
    And,
    Or,
}
