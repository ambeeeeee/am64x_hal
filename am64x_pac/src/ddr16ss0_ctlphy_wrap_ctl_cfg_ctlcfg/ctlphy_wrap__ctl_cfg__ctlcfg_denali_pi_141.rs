#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_141` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi141Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_141` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi141Spec>;
#[doc = "Field `PI_MRW_STATUS` reader - 7:0\\]
Write memory mode register status. Bit \\[0\\]
set indicates a WRITE_MODEREG parameter programming error. Bit \\[1\\]
set indicates a PASR error. Bit \\[2\\]
is Reserved. Bit \\[3\\]
set indicates a self refresh or deep power down error. Bit \\[4\\]
set indicates that a write to MR3 or MR11 was attempted \\[write_modereg bit \\[25\\]
was asserted with bit \\[17\\]
set, or bit \\[23\\]
was asserted with bits \\[7:0\\]
defining MR3 or MR11\\]
during tZQCAL after a ZQ calibration start command. READ-ONLY"]
pub type PiMrwStatusR = crate::FieldReader;
#[doc = "Field `PI_MRW_STATUS` writer - 7:0\\]
Write memory mode register status. Bit \\[0\\]
set indicates a WRITE_MODEREG parameter programming error. Bit \\[1\\]
set indicates a PASR error. Bit \\[2\\]
is Reserved. Bit \\[3\\]
set indicates a self refresh or deep power down error. Bit \\[4\\]
set indicates that a write to MR3 or MR11 was attempted \\[write_modereg bit \\[25\\]
was asserted with bit \\[17\\]
set, or bit \\[23\\]
was asserted with bits \\[7:0\\]
defining MR3 or MR11\\]
during tZQCAL after a ZQ calibration start command. READ-ONLY"]
pub type PiMrwStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Write memory mode register status. Bit \\[0\\]
set indicates a WRITE_MODEREG parameter programming error. Bit \\[1\\]
set indicates a PASR error. Bit \\[2\\]
is Reserved. Bit \\[3\\]
set indicates a self refresh or deep power down error. Bit \\[4\\]
set indicates that a write to MR3 or MR11 was attempted \\[write_modereg bit \\[25\\]
was asserted with bit \\[17\\]
set, or bit \\[23\\]
was asserted with bits \\[7:0\\]
defining MR3 or MR11\\]
during tZQCAL after a ZQ calibration start command. READ-ONLY"]
    #[inline(always)]
    pub fn pi_mrw_status(&self) -> PiMrwStatusR {
        PiMrwStatusR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Write memory mode register status. Bit \\[0\\]
set indicates a WRITE_MODEREG parameter programming error. Bit \\[1\\]
set indicates a PASR error. Bit \\[2\\]
is Reserved. Bit \\[3\\]
set indicates a self refresh or deep power down error. Bit \\[4\\]
set indicates that a write to MR3 or MR11 was attempted \\[write_modereg bit \\[25\\]
was asserted with bit \\[17\\]
set, or bit \\[23\\]
was asserted with bits \\[7:0\\]
defining MR3 or MR11\\]
during tZQCAL after a ZQ calibration start command. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_mrw_status(&mut self) -> PiMrwStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi141Spec> {
        PiMrwStatusW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_141\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_141::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_141::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi141Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi141Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_141::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi141Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_141::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi141Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_141 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi141Spec {
    const RESET_VALUE: u32 = 0;
}
