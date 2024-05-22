#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_211` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi211Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_211` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi211Spec>;
#[doc = "Field `PI_TCKELCK_F1` reader - 4:0\\]
Valid Clock Requirement after CKE deassert for frequency set 1."]
pub type PiTckelckF1R = crate::FieldReader;
#[doc = "Field `PI_TCKELCK_F1` writer - 4:0\\]
Valid Clock Requirement after CKE deassert for frequency set 1."]
pub type PiTckelckF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TDFI_CALVL_STROBE_F2` reader - 11:8\\]
Minimum number of DFI PHY clocks from dfi_calvl_data to dfi_calvl_strobe mode for frequency set 2."]
pub type PiTdfiCalvlStrobeF2R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CALVL_STROBE_F2` writer - 11:8\\]
Minimum number of DFI PHY clocks from dfi_calvl_data to dfi_calvl_strobe mode for frequency set 2."]
pub type PiTdfiCalvlStrobeF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TXP_F2` reader - 20:16\\]
CKE assert to next valid command delay for frequency set 2."]
pub type PiTxpF2R = crate::FieldReader;
#[doc = "Field `PI_TXP_F2` writer - 20:16\\]
CKE assert to next valid command delay for frequency set 2."]
pub type PiTxpF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TMRWCKEL_F2` reader - 31:24\\]
Valid Clock and CS Requirement before CKE deassert after MRW Command for frequency set 2."]
pub type PiTmrwckelF2R = crate::FieldReader;
#[doc = "Field `PI_TMRWCKEL_F2` writer - 31:24\\]
Valid Clock and CS Requirement before CKE deassert after MRW Command for frequency set 2."]
pub type PiTmrwckelF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Valid Clock Requirement after CKE deassert for frequency set 1."]
    #[inline(always)]
    pub fn pi_tckelck_f1(&self) -> PiTckelckF1R {
        PiTckelckF1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Minimum number of DFI PHY clocks from dfi_calvl_data to dfi_calvl_strobe mode for frequency set 2."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_strobe_f2(&self) -> PiTdfiCalvlStrobeF2R {
        PiTdfiCalvlStrobeF2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
CKE assert to next valid command delay for frequency set 2."]
    #[inline(always)]
    pub fn pi_txp_f2(&self) -> PiTxpF2R {
        PiTxpF2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Valid Clock and CS Requirement before CKE deassert after MRW Command for frequency set 2."]
    #[inline(always)]
    pub fn pi_tmrwckel_f2(&self) -> PiTmrwckelF2R {
        PiTmrwckelF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Valid Clock Requirement after CKE deassert for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tckelck_f1(&mut self) -> PiTckelckF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi211Spec> {
        PiTckelckF1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Minimum number of DFI PHY clocks from dfi_calvl_data to dfi_calvl_strobe mode for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_strobe_f2(
        &mut self,
    ) -> PiTdfiCalvlStrobeF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi211Spec> {
        PiTdfiCalvlStrobeF2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
CKE assert to next valid command delay for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_txp_f2(&mut self) -> PiTxpF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi211Spec> {
        PiTxpF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Valid Clock and CS Requirement before CKE deassert after MRW Command for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrwckel_f2(&mut self) -> PiTmrwckelF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi211Spec> {
        PiTmrwckelF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_211\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_211::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_211::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi211Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi211Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_211::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi211Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_211::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi211Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_211 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi211Spec {
    const RESET_VALUE: u32 = 0;
}
