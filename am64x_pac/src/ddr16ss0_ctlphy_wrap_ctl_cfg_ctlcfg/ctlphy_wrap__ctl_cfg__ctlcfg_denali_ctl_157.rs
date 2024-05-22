#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_157` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl157Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_157` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl157Spec>;
#[doc = "Field `CKSRE_F1` reader - 7:0\\]
Clock hold delay on self-refresh entry. FC=1"]
pub type CksreF1R = crate::FieldReader;
#[doc = "Field `CKSRE_F1` writer - 7:0\\]
Clock hold delay on self-refresh entry. FC=1"]
pub type CksreF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKSRX_F1` reader - 15:8\\]
Clock stable delay on self-refresh exit. FC=1"]
pub type CksrxF1R = crate::FieldReader;
#[doc = "Field `CKSRX_F1` writer - 15:8\\]
Clock stable delay on self-refresh exit. FC=1"]
pub type CksrxF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKSRE_F2` reader - 23:16\\]
Clock hold delay on self-refresh entry. FC=2"]
pub type CksreF2R = crate::FieldReader;
#[doc = "Field `CKSRE_F2` writer - 23:16\\]
Clock hold delay on self-refresh entry. FC=2"]
pub type CksreF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKSRX_F2` reader - 31:24\\]
Clock stable delay on self-refresh exit. FC=2"]
pub type CksrxF2R = crate::FieldReader;
#[doc = "Field `CKSRX_F2` writer - 31:24\\]
Clock stable delay on self-refresh exit. FC=2"]
pub type CksrxF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Clock hold delay on self-refresh entry. FC=1"]
    #[inline(always)]
    pub fn cksre_f1(&self) -> CksreF1R {
        CksreF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Clock stable delay on self-refresh exit. FC=1"]
    #[inline(always)]
    pub fn cksrx_f1(&self) -> CksrxF1R {
        CksrxF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Clock hold delay on self-refresh entry. FC=2"]
    #[inline(always)]
    pub fn cksre_f2(&self) -> CksreF2R {
        CksreF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Clock stable delay on self-refresh exit. FC=2"]
    #[inline(always)]
    pub fn cksrx_f2(&self) -> CksrxF2R {
        CksrxF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Clock hold delay on self-refresh entry. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn cksre_f1(&mut self) -> CksreF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl157Spec> {
        CksreF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Clock stable delay on self-refresh exit. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn cksrx_f1(&mut self) -> CksrxF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl157Spec> {
        CksrxF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Clock hold delay on self-refresh entry. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn cksre_f2(&mut self) -> CksreF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl157Spec> {
        CksreF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Clock stable delay on self-refresh exit. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn cksrx_f2(&mut self) -> CksrxF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl157Spec> {
        CksrxF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_157\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_157::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_157::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl157Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl157Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_157::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl157Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_157::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl157Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_157 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl157Spec {
    const RESET_VALUE: u32 = 0;
}
