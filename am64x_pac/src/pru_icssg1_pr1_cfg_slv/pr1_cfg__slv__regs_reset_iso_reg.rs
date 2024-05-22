#[doc = "Register `PR1_CFG__SLV__REGS_reset_iso_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsResetIsoRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_reset_iso_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsResetIsoRegSpec>;
#[doc = "Field `RESET_ISO_REQ` reader - "]
pub type ResetIsoReqR = crate::BitReader;
#[doc = "Field `RESET_ISO_REQ` writer - "]
pub type ResetIsoReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_ISO_ACK` reader - "]
pub type ResetIsoAckR = crate::BitReader;
#[doc = "Field `RESET_ISO_ACK` writer - "]
pub type ResetIsoAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_ISO_EDGE` reader - "]
pub type ResetIsoEdgeR = crate::BitReader;
#[doc = "Field `RESET_ISO_EDGE` writer - "]
pub type ResetIsoEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reset_iso_req(&self) -> ResetIsoReqR {
        ResetIsoReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reset_iso_ack(&self) -> ResetIsoAckR {
        ResetIsoAckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reset_iso_edge(&self) -> ResetIsoEdgeR {
        ResetIsoEdgeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reset_iso_req(&mut self) -> ResetIsoReqW<Pr1Cfg_Slv_RegsResetIsoRegSpec> {
        ResetIsoReqW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reset_iso_ack(&mut self) -> ResetIsoAckW<Pr1Cfg_Slv_RegsResetIsoRegSpec> {
        ResetIsoAckW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reset_iso_edge(&mut self) -> ResetIsoEdgeW<Pr1Cfg_Slv_RegsResetIsoRegSpec> {
        ResetIsoEdgeW::new(self, 2)
    }
}
#[doc = "PR1_CFG__SLV__REGS_reset_iso_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_reset_iso_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_reset_iso_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsResetIsoRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsResetIsoRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_reset_iso_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsResetIsoRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_reset_iso_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsResetIsoRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_reset_iso_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsResetIsoRegSpec {
    const RESET_VALUE: u32 = 0;
}
