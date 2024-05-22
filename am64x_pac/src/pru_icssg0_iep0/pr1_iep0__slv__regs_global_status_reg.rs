#[doc = "Register `PR1_IEP0__SLV__REGS_global_status_reg` reader"]
pub type R = crate::R<Pr1Iep0_Slv_RegsGlobalStatusRegSpec>;
#[doc = "Register `PR1_IEP0__SLV__REGS_global_status_reg` writer"]
pub type W = crate::W<Pr1Iep0_Slv_RegsGlobalStatusRegSpec>;
#[doc = "Field `CNT_OVF` reader - "]
pub type CntOvfR = crate::BitReader;
#[doc = "Field `CNT_OVF` writer - "]
pub type CntOvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnt_ovf(&self) -> CntOvfR {
        CntOvfR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_ovf(&mut self) -> CntOvfW<Pr1Iep0_Slv_RegsGlobalStatusRegSpec> {
        CntOvfW::new(self, 0)
    }
}
#[doc = "PR1_IEP0__SLV__REGS_global_status_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep0__slv__regs_global_status_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep0__slv__regs_global_status_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep0_Slv_RegsGlobalStatusRegSpec;
impl crate::RegisterSpec for Pr1Iep0_Slv_RegsGlobalStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep0__slv__regs_global_status_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep0_Slv_RegsGlobalStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep0__slv__regs_global_status_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep0_Slv_RegsGlobalStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP0__SLV__REGS_global_status_reg to value 0"]
impl crate::Resettable for Pr1Iep0_Slv_RegsGlobalStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
