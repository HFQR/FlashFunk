mod spsc;

pub mod channel;

pub(super) mod pin_to_core {
    use alloc::vec::Vec;

    #[cfg(feature = "std")]
    pub(crate) type CoreId = core_affinity::CoreId;

    #[cfg(not(feature = "std"))]
    pub(crate) struct CoreId;

    pub(crate) fn get_core_ids() -> Vec<CoreId> {
        #[cfg(feature = "std")]
        {
            core_affinity::get_core_ids().unwrap()
        }

        #[cfg(not(feature = "std"))]
        {
            Vec::new()
        }
    }

    pub(crate) fn pin_to_core(id: Option<CoreId>) {
        if let Some(id) = id {
            #[cfg(feature = "std")]
            {
                core_affinity::set_for_current(id)
            }

            #[cfg(not(feature = "std"))]
            {
                drop(id);
            }
        }
    }
}
