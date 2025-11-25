#[derive(Debug)]
pub enum AlreadyExists {
    LeftTreeExists,
    RightTreeExists,
}

#[derive(Debug)]
pub enum DoesntExist {
    NoRootNode,
    NoTargetNode,
}

#[derive(Debug)]
pub enum ParentError {
    ParentNodeNotFound,
}

#[derive(Debug)]
pub enum InsertError {
    LeftAlreadyExists,
    RightAlreadyExists,
    ParentHasSameValue,
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

#[derive(Debug)]
pub enum DeleteError {
    FailedToDeleteNode,
}
