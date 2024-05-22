#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_156` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl156Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_156` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl156Spec>;
#[doc = "Field `PPR_STATUS` reader - 1:0\\]
Reports the status of the PPR operation. Bit \\[0\\]
set indicates that PPR operations are now allowed and bit \\[1\\]
set indicates if the last PPR command is complete. READ-ONLY"]
pub type PprStatusR = crate::FieldReader;
#[doc = "Field `PPR_STATUS` writer - 1:0\\]
Reports the status of the PPR operation. Bit \\[0\\]
set indicates that PPR operations are now allowed and bit \\[1\\]
set indicates if the last PPR command is complete. READ-ONLY"]
pub type PprStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FM_OVRIDE_CONTROL` reader - 8:8\\]
Enables the FM Override feature. Set to 1 to enable."]
pub type FmOvrideControlR = crate::BitReader;
#[doc = "Field `FM_OVRIDE_CONTROL` writer - 8:8\\]
Enables the FM Override feature. Set to 1 to enable."]
pub type FmOvrideControlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSRE_F0` reader - 23:16\\]
Clock hold delay on self-refresh entry. FC=0"]
pub type CksreF0R = crate::FieldReader;
#[doc = "Field `CKSRE_F0` writer - 23:16\\]
Clock hold delay on self-refresh entry. FC=0"]
pub type CksreF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKSRX_F0` reader - 31:24\\]
Clock stable delay on self-refresh exit. FC=0"]
pub type CksrxF0R = crate::FieldReader;
#[doc = "Field `CKSRX_F0` writer - 31:24\\]
Clock stable delay on self-refresh exit. FC=0"]
pub type CksrxF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reports the status of the PPR operation. Bit \\[0\\]
set indicates that PPR operations are now allowed and bit \\[1\\]
set indicates if the last PPR command is complete. READ-ONLY"]
    #[inline(always)]
    pub fn ppr_status(&self) -> PprStatusR {
        PprStatusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables the FM Override feature. Set to 1 to enable."]
    #[inline(always)]
    pub fn fm_ovride_control(&self) -> FmOvrideControlR {
        FmOvrideControlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Clock hold delay on self-refresh entry. FC=0"]
    #[inline(always)]
    pub fn cksre_f0(&self) -> CksreF0R {
        CksreF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Clock stable delay on self-refresh exit. FC=0"]
    #[inline(always)]
    pub fn cksrx_f0(&self) -> CksrxF0R {
        CksrxF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reports the status of the PPR operation. Bit \\[0\\]
set indicates that PPR operations are now allowed and bit \\[1\\]
set indicates if the last PPR command is complete. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn ppr_status(&mut self) -> PprStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl156Spec> {
        PprStatusW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables the FM Override feature. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn fm_ovride_control(
        &mut self,
    ) -> FmOvrideControlW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl156Spec> {
        FmOvrideControlW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Clock hold delay on self-refresh entry. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn cksre_f0(&mut self) -> CksreF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl156Spec> {
        CksreF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Clock stable delay on self-refresh exit. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn cksrx_f0(&mut self) -> CksrxF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl156Spec> {
        CksrxF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_156\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_156::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_156::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl156Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl156Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_156::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl156Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_156::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl156Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_156 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl156Spec {
    const RESET_VALUE: u32 = 0;
}
