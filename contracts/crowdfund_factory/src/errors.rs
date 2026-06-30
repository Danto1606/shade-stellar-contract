use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum FactoryError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    WasmHashNotSet = 3,
    CampaignNotFound = 4,
    // Governance has not been initialised via `init_governance` (#358).
    GovernanceNotInitialized = 5,
    // Caller is neither the governance admin nor a granted reviewer.
    NotReviewer = 6,
    InvalidGoal = 7,
    InvalidDeadline = 8,
    ProposalNotFound = 9,
    // Proposal is not in `Pending` status.
    ProposalNotPending = 10,
    // Proposal is not in `Approved` status.
    ProposalNotApproved = 11,
}
