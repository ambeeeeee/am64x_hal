#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_209` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi209Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_209` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi209Spec>;
#[doc = "Field `PI_CALVL_VREF_DELTA_F2` reader - 3:0\\]
The delta fro the current CA vref for non-initial CA training for frequency set 2."]
pub type PiCalvlVrefDeltaF2R = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_DELTA_F2` writer - 3:0\\]
The delta fro the current CA vref for non-initial CA training for frequency set 2."]
pub type PiCalvlVrefDeltaF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TDFI_CALVL_STROBE_F0` reader - 11:8\\]
Minimum number of DFI PHY clocks from dfi_calvl_data to dfi_calvl_strobe mode for frequency set 0."]
pub type PiTdfiCalvlStrobeF0R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CALVL_STROBE_F0` writer - 11:8\\]
Minimum number of DFI PHY clocks from dfi_calvl_data to dfi_calvl_strobe mode for frequency set 0."]
pub type PiTdfiCalvlStrobeF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TXP_F0` reader - 20:16\\]
CKE assert to next valid command delay for frequency set 0."]
pub type PiTxpF0R = crate::FieldReader;
#[doc = "Field `PI_TXP_F0` writer - 20:16\\]
CKE assert to next valid command delay for frequency set 0."]
pub type PiTxpF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TMRWCKEL_F0` reader - 31:24\\]
Valid Clock and CS Requirement before CKE deassert after MRW Command for frequency set 0."]
pub type PiTmrwckelF0R = crate::FieldReader;
#[doc = "Field `PI_TMRWCKEL_F0` writer - 31:24\\]
Valid Clock and CS Requirement before CKE deassert after MRW Command for frequency set 0."]
pub type PiTmrwckelF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
The delta fro the current CA vref for non-initial CA training for frequency set 2."]
    #[inline(always)]
    pub fn pi_calvl_vref_delta_f2(&self) -> PiCalvlVrefDeltaF2R {
        PiCalvlVrefDeltaF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Minimum number of DFI PHY clocks from dfi_calvl_data to dfi_calvl_strobe mode for frequency set 0."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_strobe_f0(&self) -> PiTdfiCalvlStrobeF0R {
        PiTdfiCalvlStrobeF0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
CKE assert to next valid command delay for frequency set 0."]
    #[inline(always)]
    pub fn pi_txp_f0(&self) -> PiTxpF0R {
        PiTxpF0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Valid Clock and CS Requirement before CKE deassert after MRW Command for frequency set 0."]
    #[inline(always)]
    pub fn pi_tmrwckel_f0(&self) -> PiTmrwckelF0R {
        PiTmrwckelF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
The delta fro the current CA vref for non-initial CA training for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_delta_f2(
        &mut self,
    ) -> PiCalvlVrefDeltaF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi209Spec> {
        PiCalvlVrefDeltaF2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Minimum number of DFI PHY clocks from dfi_calvl_data to dfi_calvl_strobe mode for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_strobe_f0(
        &mut self,
    ) -> PiTdfiCalvlStrobeF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi209Spec> {
        PiTdfiCalvlStrobeF0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
CKE assert to next valid command delay for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_txp_f0(&mut self) -> PiTxpF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi209Spec> {
        PiTxpF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Valid Clock and CS Requirement before CKE deassert after MRW Command for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrwckel_f0(&mut self) -> PiTmrwckelF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi209Spec> {
        PiTmrwckelF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_209\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_209::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_209::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi209Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi209Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_209::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi209Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_209::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi209Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_209 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi209Spec {
    const RESET_VALUE: u32 = 0;
}
