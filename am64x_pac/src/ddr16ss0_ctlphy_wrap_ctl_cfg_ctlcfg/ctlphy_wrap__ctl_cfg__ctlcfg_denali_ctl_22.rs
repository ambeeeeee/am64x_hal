#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_22` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl22Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_22` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl22Spec>;
#[doc = "Field `DFIBUS_FREQ_F2` reader - 4:0\\]
Defines the DFI bus frequency. FC=2"]
pub type DfibusFreqF2R = crate::FieldReader;
#[doc = "Field `DFIBUS_FREQ_F2` writer - 4:0\\]
Defines the DFI bus frequency. FC=2"]
pub type DfibusFreqF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FREQ_CHANGE_TYPE_F0` reader - 9:8\\]
Defines the encoded frequency driven out on the cntrl_freq_change_req_type signal during a frequency change operation. FC=0"]
pub type FreqChangeTypeF0R = crate::FieldReader;
#[doc = "Field `FREQ_CHANGE_TYPE_F0` writer - 9:8\\]
Defines the encoded frequency driven out on the cntrl_freq_change_req_type signal during a frequency change operation. FC=0"]
pub type FreqChangeTypeF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FREQ_CHANGE_TYPE_F1` reader - 17:16\\]
Defines the encoded frequency driven out on the cntrl_freq_change_req_type signal during a frequency change operation. FC=1"]
pub type FreqChangeTypeF1R = crate::FieldReader;
#[doc = "Field `FREQ_CHANGE_TYPE_F1` writer - 17:16\\]
Defines the encoded frequency driven out on the cntrl_freq_change_req_type signal during a frequency change operation. FC=1"]
pub type FreqChangeTypeF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FREQ_CHANGE_TYPE_F2` reader - 25:24\\]
Defines the encoded frequency driven out on the cntrl_freq_change_req_type signal during a frequency change operation. FC=2"]
pub type FreqChangeTypeF2R = crate::FieldReader;
#[doc = "Field `FREQ_CHANGE_TYPE_F2` writer - 25:24\\]
Defines the encoded frequency driven out on the cntrl_freq_change_req_type signal during a frequency change operation. FC=2"]
pub type FreqChangeTypeF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Defines the DFI bus frequency. FC=2"]
    #[inline(always)]
    pub fn dfibus_freq_f2(&self) -> DfibusFreqF2R {
        DfibusFreqF2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines the encoded frequency driven out on the cntrl_freq_change_req_type signal during a frequency change operation. FC=0"]
    #[inline(always)]
    pub fn freq_change_type_f0(&self) -> FreqChangeTypeF0R {
        FreqChangeTypeF0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines the encoded frequency driven out on the cntrl_freq_change_req_type signal during a frequency change operation. FC=1"]
    #[inline(always)]
    pub fn freq_change_type_f1(&self) -> FreqChangeTypeF1R {
        FreqChangeTypeF1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Defines the encoded frequency driven out on the cntrl_freq_change_req_type signal during a frequency change operation. FC=2"]
    #[inline(always)]
    pub fn freq_change_type_f2(&self) -> FreqChangeTypeF2R {
        FreqChangeTypeF2R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Defines the DFI bus frequency. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn dfibus_freq_f2(&mut self) -> DfibusFreqF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl22Spec> {
        DfibusFreqF2W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines the encoded frequency driven out on the cntrl_freq_change_req_type signal during a frequency change operation. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn freq_change_type_f0(
        &mut self,
    ) -> FreqChangeTypeF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl22Spec> {
        FreqChangeTypeF0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines the encoded frequency driven out on the cntrl_freq_change_req_type signal during a frequency change operation. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn freq_change_type_f1(
        &mut self,
    ) -> FreqChangeTypeF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl22Spec> {
        FreqChangeTypeF1W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Defines the encoded frequency driven out on the cntrl_freq_change_req_type signal during a frequency change operation. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn freq_change_type_f2(
        &mut self,
    ) -> FreqChangeTypeF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl22Spec> {
        FreqChangeTypeF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl22Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_22::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl22Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_22::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_22 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl22Spec {
    const RESET_VALUE: u32 = 0;
}
