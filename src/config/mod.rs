use libsignal_service::{
    models::Contact,
    prelude::{
        protocol::{IdentityKeyStore, PreKeyStore, SessionStoreExt, SignedPreKeyStore},
        Content, Uuid,
    },
};

use crate::{manager::Registered, Error};

#[cfg(feature = "sled-config-store")]
pub mod sled;

#[cfg(feature = "volatile-config-store")]
pub mod volatile;

#[cfg(feature = "secret-volatile-config-store")]
pub mod secret_volatile;

pub trait ConfigStore:
    PreKeyStore
    + SignedPreKeyStore
    + SessionStoreExt
    + IdentityKeyStore
    + StateStore<Registered>
    + ContactsStore
    + Sync
    + Clone
{
    fn pre_keys_offset_id(&self) -> Result<u32, Error>;
    fn set_pre_keys_offset_id(&mut self, id: u32) -> Result<(), Error>;

    fn next_signed_pre_key_id(&self) -> Result<u32, Error>;
    fn set_next_signed_pre_key_id(&mut self, id: u32) -> Result<(), Error>;
}

pub trait StateStore<S> {
    fn load_state(&self) -> Result<Registered, Error>;
    fn save_state(&mut self, state: &S) -> Result<(), Error>;
}

pub trait ContactsStore {
    fn save_contacts(&mut self, contacts: impl Iterator<Item = Contact>) -> Result<(), Error>;
    fn contacts(&self) -> Result<Vec<Contact>, Error>;
    fn contact_by_id(&self, id: Uuid) -> Result<Option<Contact>, Error>;
}

pub trait MessageStore {
    fn save_message(&mut self, sender: Uuid, timestamp: u64, message: Content)
        -> Result<(), Error>;
    fn messages(&self) -> Result<Vec<Content>, Error>;
    fn message_by_sender_timestamp(
        &self,
        sender: Uuid,
        timestamp: u64,
    ) -> Result<Option<Content>, Error>;
}
