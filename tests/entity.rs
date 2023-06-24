use std::fmt::Debug;

use anitomy_rust::elements::Category;

fn has_send_sync<T: Sized + Send + Sync + Unpin>() {}
fn has_needed<T: Clone + Debug>() {}
fn has_copy<T : Copy>(){}

/// Test if all Entity int this library has Send and Sync trait implemented
///
/// This test is successful if all entity have the Sync and Send trait.
/// If the library compile, then this test must pass (has Send and Sync
///  traits is checked at compile time).
#[test]
fn has_entity_sync_send() {
    has_send_sync::<Category>();
}

/// Test if all Entity int this library has Send and Sync trait implemented
///
/// This test is successful if all entity have Copy, Clone and Debug trait
/// If the library compile, then this test must pass
#[test]
fn has_entity_needed() {
    has_needed::<Category>();
    has_copy::<Category>();
}
