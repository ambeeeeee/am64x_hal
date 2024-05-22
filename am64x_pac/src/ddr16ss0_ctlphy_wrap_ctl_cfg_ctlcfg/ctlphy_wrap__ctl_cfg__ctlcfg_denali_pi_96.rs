#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_96` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi96Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_96` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi96Spec>;
#[doc = "Field `PI_TDFI_PARIN_LAT` reader - 2:0\\]
Defines the DFI tPARIN_LAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a DFI command and a dfi_parity_in signal assertion."]
pub type PiTdfiParinLatR = crate::FieldReader;
#[doc = "Field `PI_TDFI_PARIN_LAT` writer - 2:0\\]
Defines the DFI tPARIN_LAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a DFI command and a dfi_parity_in signal assertion."]
pub type PiTdfiParinLatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PI_BIST_GO` reader - 8:8\\]
Initiate a BIST operation. Set to 1 to trigger."]
pub type PiBistGoR = crate::BitReader;
#[doc = "Field `PI_BIST_GO` writer - 8:8\\]
Initiate a BIST operation. Set to 1 to trigger."]
pub type PiBistGoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_BIST_RESULT` reader - 17:16\\]
BIST operation status \\[pass/fail\\]. Bit \\[0\\]
indicates data check status and bit \\[1\\]
indicates address check status. Value of 1 is a passing result. READ-ONLY"]
pub type PiBistResultR = crate::FieldReader;
#[doc = "Field `PI_BIST_RESULT` writer - 17:16\\]
BIST operation status \\[pass/fail\\]. Bit \\[0\\]
indicates data check status and bit \\[1\\]
indicates address check status. Value of 1 is a passing result. READ-ONLY"]
pub type PiBistResultW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_BIST_LFSR_PATTERN_DONE` reader - 24:24\\]
BIST operation lfsr pattern, data pattern 1'b0 means the data is useful,1'b1 means next pattern sequence can ingore. READ-ONLY"]
pub type PiBistLfsrPatternDoneR = crate::BitReader;
#[doc = "Field `PI_BIST_LFSR_PATTERN_DONE` writer - 24:24\\]
BIST operation lfsr pattern, data pattern 1'b0 means the data is useful,1'b1 means next pattern sequence can ingore. READ-ONLY"]
pub type PiBistLfsrPatternDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Defines the DFI tPARIN_LAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a DFI command and a dfi_parity_in signal assertion."]
    #[inline(always)]
    pub fn pi_tdfi_parin_lat(&self) -> PiTdfiParinLatR {
        PiTdfiParinLatR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Initiate a BIST operation. Set to 1 to trigger."]
    #[inline(always)]
    pub fn pi_bist_go(&self) -> PiBistGoR {
        PiBistGoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
BIST operation status \\[pass/fail\\]. Bit \\[0\\]
indicates data check status and bit \\[1\\]
indicates address check status. Value of 1 is a passing result. READ-ONLY"]
    #[inline(always)]
    pub fn pi_bist_result(&self) -> PiBistResultR {
        PiBistResultR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
BIST operation lfsr pattern, data pattern 1'b0 means the data is useful,1'b1 means next pattern sequence can ingore. READ-ONLY"]
    #[inline(always)]
    pub fn pi_bist_lfsr_pattern_done(&self) -> PiBistLfsrPatternDoneR {
        PiBistLfsrPatternDoneR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Defines the DFI tPARIN_LAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a DFI command and a dfi_parity_in signal assertion."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_parin_lat(&mut self) -> PiTdfiParinLatW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi96Spec> {
        PiTdfiParinLatW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Initiate a BIST operation. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_go(&mut self) -> PiBistGoW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi96Spec> {
        PiBistGoW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
BIST operation status \\[pass/fail\\]. Bit \\[0\\]
indicates data check status and bit \\[1\\]
indicates address check status. Value of 1 is a passing result. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_result(&mut self) -> PiBistResultW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi96Spec> {
        PiBistResultW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
BIST operation lfsr pattern, data pattern 1'b0 means the data is useful,1'b1 means next pattern sequence can ingore. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_lfsr_pattern_done(
        &mut self,
    ) -> PiBistLfsrPatternDoneW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi96Spec> {
        PiBistLfsrPatternDoneW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_96::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_96::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi96Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi96Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_96::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi96Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_96::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi96Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_96 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi96Spec {
    const RESET_VALUE: u32 = 0;
}
