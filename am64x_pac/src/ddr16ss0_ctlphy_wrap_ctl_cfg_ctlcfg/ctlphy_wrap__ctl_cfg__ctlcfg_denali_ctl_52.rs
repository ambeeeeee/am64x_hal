#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_52` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl52Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_52` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl52Spec>;
#[doc = "Field `TRAS_MIN_F2` reader - 8:0\\]
DRAM TRAS_MIN value in cycles. FC=2"]
pub type TrasMinF2R = crate::FieldReader<u16>;
#[doc = "Field `TRAS_MIN_F2` writer - 8:0\\]
DRAM TRAS_MIN value in cycles. FC=2"]
pub type TrasMinF2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TWTR_F2` reader - 21:16\\]
DRAM TWTR value in cycles. FC=2"]
pub type TwtrF2R = crate::FieldReader;
#[doc = "Field `TWTR_F2` writer - 21:16\\]
DRAM TWTR value in cycles. FC=2"]
pub type TwtrF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TWTR_L_F2` reader - 29:24\\]
DRAM TWTR_L value in cycles. FC=2"]
pub type TwtrLF2R = crate::FieldReader;
#[doc = "Field `TWTR_L_F2` writer - 29:24\\]
DRAM TWTR_L value in cycles. FC=2"]
pub type TwtrLF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
DRAM TRAS_MIN value in cycles. FC=2"]
    #[inline(always)]
    pub fn tras_min_f2(&self) -> TrasMinF2R {
        TrasMinF2R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM TWTR value in cycles. FC=2"]
    #[inline(always)]
    pub fn twtr_f2(&self) -> TwtrF2R {
        TwtrF2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
DRAM TWTR_L value in cycles. FC=2"]
    #[inline(always)]
    pub fn twtr_l_f2(&self) -> TwtrLF2R {
        TwtrLF2R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
DRAM TRAS_MIN value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tras_min_f2(&mut self) -> TrasMinF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl52Spec> {
        TrasMinF2W::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM TWTR value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn twtr_f2(&mut self) -> TwtrF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl52Spec> {
        TwtrF2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
DRAM TWTR_L value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn twtr_l_f2(&mut self) -> TwtrLF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl52Spec> {
        TwtrLF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl52Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_52::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl52Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_52::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_52 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl52Spec {
    const RESET_VALUE: u32 = 0;
}
