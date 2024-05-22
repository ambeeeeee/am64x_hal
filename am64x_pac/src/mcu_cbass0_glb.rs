#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    glb_regs_pid: GlbRegsPid,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn glb_regs_pid(&self) -> &GlbRegsPid {
        &self.glb_regs_pid
    }
}
#[doc = "GLB_REGS_pid (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_regs_pid`]
module"]
#[doc(alias = "GLB_REGS_pid")]
pub type GlbRegsPid = crate::Reg<glb_regs_pid::GlbRegsPidSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod glb_regs_pid;
