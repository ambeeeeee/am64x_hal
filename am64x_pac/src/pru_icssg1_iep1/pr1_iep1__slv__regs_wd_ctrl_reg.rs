#[doc = "Register `PR1_IEP1__SLV__REGS_wd_ctrl_reg` reader"]
pub type R = crate::R<Pr1Iep1_Slv_RegsWdCtrlRegSpec>;
#[doc = "Register `PR1_IEP1__SLV__REGS_wd_ctrl_reg` writer"]
pub type W = crate::W<Pr1Iep1_Slv_RegsWdCtrlRegSpec>;
#[doc = "Field `PD_WD_EN` reader - "]
pub type PdWdEnR = crate::BitReader;
#[doc = "Field `PD_WD_EN` writer - "]
pub type PdWdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDI_WD_EN` reader - "]
pub type PdiWdEnR = crate::BitReader;
#[doc = "Field `PDI_WD_EN` writer - "]
pub type PdiWdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pd_wd_en(&self) -> PdWdEnR {
        PdWdEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pdi_wd_en(&self) -> PdiWdEnR {
        PdiWdEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pd_wd_en(&mut self) -> PdWdEnW<Pr1Iep1_Slv_RegsWdCtrlRegSpec> {
        PdWdEnW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pdi_wd_en(&mut self) -> PdiWdEnW<Pr1Iep1_Slv_RegsWdCtrlRegSpec> {
        PdiWdEnW::new(self, 16)
    }
}
#[doc = "PR1_IEP1__SLV__REGS_wd_ctrl_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_wd_ctrl_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_wd_ctrl_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep1_Slv_RegsWdCtrlRegSpec;
impl crate::RegisterSpec for Pr1Iep1_Slv_RegsWdCtrlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep1__slv__regs_wd_ctrl_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep1_Slv_RegsWdCtrlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep1__slv__regs_wd_ctrl_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep1_Slv_RegsWdCtrlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP1__SLV__REGS_wd_ctrl_reg to value 0"]
impl crate::Resettable for Pr1Iep1_Slv_RegsWdCtrlRegSpec {
    const RESET_VALUE: u32 = 0;
}
