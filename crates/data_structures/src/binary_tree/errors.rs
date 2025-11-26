use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlreadyExists {
    #[error("Left tree already exists!")]
    LeftTreeExists,
    #[error("Right tree already exists!")]
    RightTreeExists,
}

#[derive(Error, Debug)]
pub enum DoesntExist {
    #[error["Root node does not exist!"]]
    NoRootNode,
    #[error["Target node does not exist!"]]
    NoTargetNode,
}

#[derive(Error, Debug)]
pub enum ParentError {
    #[error("Failed to find parent node!")]
    ParentNodeNotFound,
}

#[derive(Error, Debug)]
pub enum InsertError {
    #[error("Left tree already exists!")]
    LeftAlreadyExists,
    #[error("Right tree already exists!")]
    RightAlreadyExists,
    #[error("Parent and inserted value are same!")]
    ParentHasSameValue,
    #[error("Failed to find parent node!")]
    ParentNotFound,
}

impl From<AlreadyExists> for InsertError {
    fn from(error: AlreadyExists) -> Self {
        match error {
            AlreadyExists::LeftTreeExists => InsertError::LeftAlreadyExists,
            AlreadyExists::RightTreeExists => InsertError::RightAlreadyExists,
        }
    }
}

#[derive(Error, Debug)]
pub enum DeleteError {
    #[error("Failed to detele the node!")]
    FailedToDeleteNode,
}
