#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_28` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl28Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_28` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl28Spec>;
#[doc = "Field `MRR_LSB_REG` reader - 7:0\\]
Set LSB MRR register number for DQS Oscillator TEST mode."]
pub type MrrLsbRegR = crate::FieldReader;
#[doc = "Field `MRR_LSB_REG` writer - 7:0\\]
Set LSB MRR register number for DQS Oscillator TEST mode."]
pub type MrrLsbRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MRR_MSB_REG` reader - 15:8\\]
Set MSB MRR register number for DQS Oscillator TEST mode."]
pub type MrrMsbRegR = crate::FieldReader;
#[doc = "Field `MRR_MSB_REG` writer - 15:8\\]
Set MSB MRR register number for DQS Oscillator TEST mode."]
pub type MrrMsbRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DQS_OSC_ENABLE` reader - 16:16\\]
Enable DQS oscillator measurement function in DRAM. Set to 1 to enable."]
pub type DqsOscEnableR = crate::BitReader;
#[doc = "Field `DQS_OSC_ENABLE` writer - 16:16\\]
Enable DQS oscillator measurement function in DRAM. Set to 1 to enable."]
pub type DqsOscEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Set LSB MRR register number for DQS Oscillator TEST mode."]
    #[inline(always)]
    pub fn mrr_lsb_reg(&self) -> MrrLsbRegR {
        MrrLsbRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Set MSB MRR register number for DQS Oscillator TEST mode."]
    #[inline(always)]
    pub fn mrr_msb_reg(&self) -> MrrMsbRegR {
        MrrMsbRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable DQS oscillator measurement function in DRAM. Set to 1 to enable."]
    #[inline(always)]
    pub fn dqs_osc_enable(&self) -> DqsOscEnableR {
        DqsOscEnableR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Set LSB MRR register number for DQS Oscillator TEST mode."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_lsb_reg(&mut self) -> MrrLsbRegW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl28Spec> {
        MrrLsbRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Set MSB MRR register number for DQS Oscillator TEST mode."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_msb_reg(&mut self) -> MrrMsbRegW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl28Spec> {
        MrrMsbRegW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable DQS oscillator measurement function in DRAM. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_enable(&mut self) -> DqsOscEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl28Spec> {
        DqsOscEnableW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl28Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_28::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl28Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_28::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_28 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl28Spec {
    const RESET_VALUE: u32 = 0;
}
