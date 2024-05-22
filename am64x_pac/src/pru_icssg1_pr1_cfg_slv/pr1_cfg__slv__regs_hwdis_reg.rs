#[doc = "Register `PR1_CFG__SLV__REGS_hwdis_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsHwdisRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_hwdis_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsHwdisRegSpec>;
#[doc = "Field `HWDIS` reader - 7:0\\]
HW Disable Observation"]
pub type HwdisR = crate::FieldReader;
#[doc = "Field `HWDIS` writer - 7:0\\]
HW Disable Observation"]
pub type HwdisW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
HW Disable Observation"]
    #[inline(always)]
    pub fn hwdis(&self) -> HwdisR {
        HwdisR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
HW Disable Observation"]
    #[inline(always)]
    #[must_use]
    pub fn hwdis(&mut self) -> HwdisW<Pr1Cfg_Slv_RegsHwdisRegSpec> {
        HwdisW::new(self, 0)
    }
}
#[doc = "PR1_CFG__SLV__REGS_hwdis_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_hwdis_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_hwdis_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsHwdisRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsHwdisRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_hwdis_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsHwdisRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_hwdis_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsHwdisRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_hwdis_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsHwdisRegSpec {
    const RESET_VALUE: u32 = 0;
}
