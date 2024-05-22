#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_307` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl307Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_307` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl307Spec>;
#[doc = "Field `TZQCAL_F0` reader - 11:0\\]
Holds the DRAM ZQCAL value in cycles. FC=0"]
pub type TzqcalF0R = crate::FieldReader<u16>;
#[doc = "Field `TZQCAL_F0` writer - 11:0\\]
Holds the DRAM ZQCAL value in cycles. FC=0"]
pub type TzqcalF0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TZQLAT_F0` reader - 22:16\\]
Holds the DRAM ZQLAT value in cycles. FC=0"]
pub type TzqlatF0R = crate::FieldReader;
#[doc = "Field `TZQLAT_F0` writer - 22:16\\]
Holds the DRAM ZQLAT value in cycles. FC=0"]
pub type TzqlatF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Holds the DRAM ZQCAL value in cycles. FC=0"]
    #[inline(always)]
    pub fn tzqcal_f0(&self) -> TzqcalF0R {
        TzqcalF0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Holds the DRAM ZQLAT value in cycles. FC=0"]
    #[inline(always)]
    pub fn tzqlat_f0(&self) -> TzqlatF0R {
        TzqlatF0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Holds the DRAM ZQCAL value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tzqcal_f0(&mut self) -> TzqcalF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl307Spec> {
        TzqcalF0W::new(self, 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Holds the DRAM ZQLAT value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tzqlat_f0(&mut self) -> TzqlatF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl307Spec> {
        TzqlatF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_307\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_307::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_307::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl307Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl307Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_307::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl307Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_307::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl307Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_307 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl307Spec {
    const RESET_VALUE: u32 = 0;
}
