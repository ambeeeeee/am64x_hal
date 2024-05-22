#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0003_0040],
    gic_translater_regs_translater__1_gits_translater: GicTranslaterRegsTranslater_1GitsTranslater,
}
impl RegisterBlock {
    #[doc = "0x30040 - GITS_TRANSLATER"]
    #[inline(always)]
    pub const fn gic_translater_regs_translater__1_gits_translater(
        &self,
    ) -> &GicTranslaterRegsTranslater_1GitsTranslater {
        &self.gic_translater_regs_translater__1_gits_translater
    }
}
#[doc = "GIC_TRANSLATER_REGS_TRANSLATER__1_GITS_TRANSLATER (rw) register accessor: GITS_TRANSLATER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_translater_regs_translater__1_gits_translater::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_translater_regs_translater__1_gits_translater::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gic_translater_regs_translater__1_gits_translater`]
module"]
#[doc(alias = "GIC_TRANSLATER_REGS_TRANSLATER__1_GITS_TRANSLATER")]
pub type GicTranslaterRegsTranslater_1GitsTranslater = crate :: Reg < gic_translater_regs_translater__1_gits_translater :: GicTranslaterRegsTranslater_1GitsTranslaterSpec > ;
#[doc = "GITS_TRANSLATER"]
pub mod gic_translater_regs_translater__1_gits_translater;
