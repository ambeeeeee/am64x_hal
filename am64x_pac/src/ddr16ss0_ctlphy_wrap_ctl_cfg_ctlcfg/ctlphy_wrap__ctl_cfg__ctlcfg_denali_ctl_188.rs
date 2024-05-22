#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_188` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl188Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_188` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl188Spec>;
#[doc = "Field `WRITE_MODEREG` reader - 26:0\\]
Write memory mode register data to the DRAMs. Bits \\[7:0\\]
define the memory mode register number if bit \\[23\\]
is set, bits \\[15:8\\]
define the chip select if bit \\[24\\]
is clear, bits \\[23:16\\]
define which memory mode register/s to write, bit \\[24\\]
defines whether all chip selects will be written, and bit \\[25\\]
triggers the write."]
pub type WriteModeregR = crate::FieldReader<u32>;
#[doc = "Field `WRITE_MODEREG` writer - 26:0\\]
Write memory mode register data to the DRAMs. Bits \\[7:0\\]
define the memory mode register number if bit \\[23\\]
is set, bits \\[15:8\\]
define the chip select if bit \\[24\\]
is clear, bits \\[23:16\\]
define which memory mode register/s to write, bit \\[24\\]
defines whether all chip selects will be written, and bit \\[25\\]
triggers the write."]
pub type WriteModeregW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:26 - 26:0\\]
Write memory mode register data to the DRAMs. Bits \\[7:0\\]
define the memory mode register number if bit \\[23\\]
is set, bits \\[15:8\\]
define the chip select if bit \\[24\\]
is clear, bits \\[23:16\\]
define which memory mode register/s to write, bit \\[24\\]
defines whether all chip selects will be written, and bit \\[25\\]
triggers the write."]
    #[inline(always)]
    pub fn write_modereg(&self) -> WriteModeregR {
        WriteModeregR::new(self.bits & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:26 - 26:0\\]
Write memory mode register data to the DRAMs. Bits \\[7:0\\]
define the memory mode register number if bit \\[23\\]
is set, bits \\[15:8\\]
define the chip select if bit \\[24\\]
is clear, bits \\[23:16\\]
define which memory mode register/s to write, bit \\[24\\]
defines whether all chip selects will be written, and bit \\[25\\]
triggers the write."]
    #[inline(always)]
    #[must_use]
    pub fn write_modereg(&mut self) -> WriteModeregW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl188Spec> {
        WriteModeregW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_188\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_188::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_188::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl188Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl188Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_188::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl188Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_188::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl188Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_188 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl188Spec {
    const RESET_VALUE: u32 = 0;
}
