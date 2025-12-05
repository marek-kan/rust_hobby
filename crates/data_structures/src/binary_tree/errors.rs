use thiserror::Error;

#[derive(Error, Debug)]
pub enum TreeCoreError {
    #[error("Root node does not exist!")]
    NoRootNode,
    #[error("Target node does not exist!")]
    NoTargetNode,
    #[error("Left tree already exists!")]
    LeftTreeExists,
    #[error("Right tree already exists!")]
    RightTreeExists,
}

#[derive(Error, Debug)]
pub enum SplitError {
    #[error("Failed to split the node!")]
    FailedToSplitNode,
    #[error("Failed to split the text!")]
    FailedToSplitText,
}

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("Index is not inclusive")]
    NotInclusiveError,
    #[error("Index is not exclusive")]
    NotExclusiveError,
}

#[derive(Error, Debug)]
pub enum InsertError {
    #[error("Parent and inserted value are same!")]
    ParentHasSameValue,
    #[error("Parent not found!")]
    ParentNotFound,

    #[error(transparent)]
    Core(#[from] TreeCoreError),

    #[error(transparent)]
    Index(#[from] IndexError),

    #[error(transparent)]
    Split(#[from] SplitError),
}

#[derive(Error, Debug)]
pub enum DeleteError {
    #[error("Failed to delete the node!")]
    FailedToDeleteNode,

    #[error(transparent)]
    Core(#[from] TreeCoreError),

    #[error[transparent]]
    Index(#[from] IndexError),

    #[error(transparent)]
    Split(#[from] SplitError),
}
