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
