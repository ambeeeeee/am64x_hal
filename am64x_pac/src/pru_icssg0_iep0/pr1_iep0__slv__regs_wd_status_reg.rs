#[doc = "Register `PR1_IEP0__SLV__REGS_wd_status_reg` reader"]
pub type R = crate::R<Pr1Iep0_Slv_RegsWdStatusRegSpec>;
#[doc = "Register `PR1_IEP0__SLV__REGS_wd_status_reg` writer"]
pub type W = crate::W<Pr1Iep0_Slv_RegsWdStatusRegSpec>;
#[doc = "Field `PD_WD_STAT` reader - "]
pub type PdWdStatR = crate::BitReader;
#[doc = "Field `PD_WD_STAT` writer - "]
pub type PdWdStatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDI_WD_STAT` reader - "]
pub type PdiWdStatR = crate::BitReader;
#[doc = "Field `PDI_WD_STAT` writer - "]
pub type PdiWdStatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pd_wd_stat(&self) -> PdWdStatR {
        PdWdStatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pdi_wd_stat(&self) -> PdiWdStatR {
        PdiWdStatR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pd_wd_stat(&mut self) -> PdWdStatW<Pr1Iep0_Slv_RegsWdStatusRegSpec> {
        PdWdStatW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pdi_wd_stat(&mut self) -> PdiWdStatW<Pr1Iep0_Slv_RegsWdStatusRegSpec> {
        PdiWdStatW::new(self, 16)
    }
}
#[doc = "PR1_IEP0__SLV__REGS_wd_status_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep0__slv__regs_wd_status_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep0__slv__regs_wd_status_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep0_Slv_RegsWdStatusRegSpec;
impl crate::RegisterSpec for Pr1Iep0_Slv_RegsWdStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep0__slv__regs_wd_status_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep0_Slv_RegsWdStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep0__slv__regs_wd_status_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep0_Slv_RegsWdStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP0__SLV__REGS_wd_status_reg to value 0x0001_0001"]
impl crate::Resettable for Pr1Iep0_Slv_RegsWdStatusRegSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
