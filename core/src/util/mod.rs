mod spsc;

#[cfg(feature = "async")]
pub mod async_runtime;

pub mod channel;
pub mod fx_hasher;
pub mod no_hasher;
pub mod spin;

pub mod pin_to_core {
    use alloc::vec::Vec;

    #[cfg(feature = "std")]
    pub type CoreId = core_affinity::CoreId;

    #[cfg(not(feature = "std"))]
    pub struct CoreId;

    pub fn get_core_ids() -> Vec<CoreId> {
        #[cfg(feature = "std")]
        {
            core_affinity::get_core_ids().unwrap()
        }

        #[cfg(not(feature = "std"))]
        {
            Vec::new()
        }
    }

    pub fn pin_to_core(id: Option<CoreId>) {
        if let Some(id) = id {
            #[cfg(feature = "std")]
            {
                core_affinity::set_for_current(id);
            }

            #[cfg(not(feature = "std"))]
            {
                drop(id);
            }
        }
    }
}
