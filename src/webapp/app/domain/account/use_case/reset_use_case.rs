use super::AccountRepository;

pub struct ResetUseCase<'a, T> {
    repository: &'a T,
}

impl<'a, T> ResetUseCase<'a, T> {
    pub fn new(repository: &'a T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self)
    where
        T: AccountRepository,
    {
        self.repository.reset().await
    }
}
