#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bcdma_cred_cred: BcdmaCredCred,
}
impl RegisterBlock {
    #[doc = "0x00 - The Credentials Register provides credentials to be used when performing memory accesses using this flow."]
    #[inline(always)]
    pub const fn bcdma_cred_cred(&self) -> &BcdmaCredCred {
        &self.bcdma_cred_cred
    }
}
#[doc = "BCDMA_CRED_CRED (rw) register accessor: The Credentials Register provides credentials to be used when performing memory accesses using this flow.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_cred_cred::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_cred_cred::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_cred_cred`]
module"]
#[doc(alias = "BCDMA_CRED_CRED")]
pub type BcdmaCredCred = crate::Reg<bcdma_cred_cred::BcdmaCredCredSpec>;
#[doc = "The Credentials Register provides credentials to be used when performing memory accesses using this flow."]
pub mod bcdma_cred_cred;
