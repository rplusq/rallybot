mod generic;
mod traits;

pub use generic::Repository;
pub use traits::{
    InMemoryRepository, RegistrationError, SessionError, SessionRepository, UserRepository,
    VenueRepository,
};