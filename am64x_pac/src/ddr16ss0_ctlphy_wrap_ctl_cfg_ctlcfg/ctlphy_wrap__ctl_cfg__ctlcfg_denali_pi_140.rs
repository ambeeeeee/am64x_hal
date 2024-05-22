#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_140` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi140Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_140` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi140Spec>;
#[doc = "Field `PI_WRITE_MODEREG` reader - 25:0\\]
Write memory mode register data to the DRAMs. Bits \\[7:0\\]
define the memory mode register number if bit \\[23\\]
is set, bits \\[15:8\\]
define the chip select if bit \\[24\\]
is clear, bits \\[23:16\\]
define which memory mode register/s to write, bit \\[24\\]
defines whether all chip selects will be written, and bit \\[25\\]
triggers the write."]
pub type PiWriteModeregR = crate::FieldReader<u32>;
#[doc = "Field `PI_WRITE_MODEREG` writer - 25:0\\]
Write memory mode register data to the DRAMs. Bits \\[7:0\\]
define the memory mode register number if bit \\[23\\]
is set, bits \\[15:8\\]
define the chip select if bit \\[24\\]
is clear, bits \\[23:16\\]
define which memory mode register/s to write, bit \\[24\\]
defines whether all chip selects will be written, and bit \\[25\\]
triggers the write."]
pub type PiWriteModeregW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - 25:0\\]
Write memory mode register data to the DRAMs. Bits \\[7:0\\]
define the memory mode register number if bit \\[23\\]
is set, bits \\[15:8\\]
define the chip select if bit \\[24\\]
is clear, bits \\[23:16\\]
define which memory mode register/s to write, bit \\[24\\]
defines whether all chip selects will be written, and bit \\[25\\]
triggers the write."]
    #[inline(always)]
    pub fn pi_write_modereg(&self) -> PiWriteModeregR {
        PiWriteModeregR::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - 25:0\\]
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
    pub fn pi_write_modereg(&mut self) -> PiWriteModeregW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi140Spec> {
        PiWriteModeregW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_140::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_140::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi140Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_140::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi140Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_140::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi140Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_140 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi140Spec {
    const RESET_VALUE: u32 = 0;
}
