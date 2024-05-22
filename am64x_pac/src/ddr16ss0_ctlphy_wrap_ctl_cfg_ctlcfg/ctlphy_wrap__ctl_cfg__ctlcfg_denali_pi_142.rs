#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_142` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi142Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_142` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi142Spec>;
#[doc = "Field `PI_READ_MODEREG` reader - 16:0\\]
Read the specified memory mode register from specified chip when start bit set. Bits \\[7:0\\]
define the memory mode register and bits \\[15:8\\]
define the chip select. Set bit \\[16\\]
to 1 to trigger."]
pub type PiReadModeregR = crate::FieldReader<u32>;
#[doc = "Field `PI_READ_MODEREG` writer - 16:0\\]
Read the specified memory mode register from specified chip when start bit set. Bits \\[7:0\\]
define the memory mode register and bits \\[15:8\\]
define the chip select. Set bit \\[16\\]
to 1 to trigger."]
pub type PiReadModeregW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Read the specified memory mode register from specified chip when start bit set. Bits \\[7:0\\]
define the memory mode register and bits \\[15:8\\]
define the chip select. Set bit \\[16\\]
to 1 to trigger."]
    #[inline(always)]
    pub fn pi_read_modereg(&self) -> PiReadModeregR {
        PiReadModeregR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Read the specified memory mode register from specified chip when start bit set. Bits \\[7:0\\]
define the memory mode register and bits \\[15:8\\]
define the chip select. Set bit \\[16\\]
to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn pi_read_modereg(&mut self) -> PiReadModeregW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi142Spec> {
        PiReadModeregW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_142\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_142::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_142::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi142Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi142Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_142::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi142Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_142::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi142Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_142 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi142Spec {
    const RESET_VALUE: u32 = 0;
}
