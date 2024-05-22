#[doc = "Register `CFG_DWDCNTR` reader"]
pub type R = crate::R<CfgDwdcntrSpec>;
#[doc = "Register `CFG_DWDCNTR` writer"]
pub type W = crate::W<CfgDwdcntrSpec>;
#[doc = "Field `DWDCNTR` reader - 24:0\\]
The value of the DWDCNTR after a system reset is 0x002D_FFFF. When the DWD is enabled and the DWD counter starts counting down from this value with an RTICLK1 time base of 3MHz, a watchdog reset will be generated in 1 second. User and privilege mode (read): Reads return the current counter value. Privilege mode (write): Writes don't have an effect."]
pub type DwdcntrR = crate::FieldReader<u32>;
#[doc = "Field `DWDCNTR` writer - 24:0\\]
The value of the DWDCNTR after a system reset is 0x002D_FFFF. When the DWD is enabled and the DWD counter starts counting down from this value with an RTICLK1 time base of 3MHz, a watchdog reset will be generated in 1 second. User and privilege mode (read): Reads return the current counter value. Privilege mode (write): Writes don't have an effect."]
pub type DwdcntrW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - 24:0\\]
The value of the DWDCNTR after a system reset is 0x002D_FFFF. When the DWD is enabled and the DWD counter starts counting down from this value with an RTICLK1 time base of 3MHz, a watchdog reset will be generated in 1 second. User and privilege mode (read): Reads return the current counter value. Privilege mode (write): Writes don't have an effect."]
    #[inline(always)]
    pub fn dwdcntr(&self) -> DwdcntrR {
        DwdcntrR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - 24:0\\]
The value of the DWDCNTR after a system reset is 0x002D_FFFF. When the DWD is enabled and the DWD counter starts counting down from this value with an RTICLK1 time base of 3MHz, a watchdog reset will be generated in 1 second. User and privilege mode (read): Reads return the current counter value. Privilege mode (write): Writes don't have an effect."]
    #[inline(always)]
    #[must_use]
    pub fn dwdcntr(&mut self) -> DwdcntrW<CfgDwdcntrSpec> {
        DwdcntrW::new(self, 0)
    }
}
#[doc = "CFG_DWDCNTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dwdcntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dwdcntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDwdcntrSpec;
impl crate::RegisterSpec for CfgDwdcntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dwdcntr::R`](R) reader structure"]
impl crate::Readable for CfgDwdcntrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dwdcntr::W`](W) writer structure"]
impl crate::Writable for CfgDwdcntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DWDCNTR to value 0x3355_4431"]
impl crate::Resettable for CfgDwdcntrSpec {
    const RESET_VALUE: u32 = 0x3355_4431;
}
