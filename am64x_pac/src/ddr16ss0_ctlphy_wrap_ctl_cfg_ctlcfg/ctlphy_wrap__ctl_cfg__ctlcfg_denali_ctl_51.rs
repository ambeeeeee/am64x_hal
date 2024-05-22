#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_51` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl51Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_51` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl51Spec>;
#[doc = "Field `TRRD_F2` reader - 7:0\\]
DRAM TRRD value in cycles. FC=2"]
pub type TrrdF2R = crate::FieldReader;
#[doc = "Field `TRRD_F2` writer - 7:0\\]
DRAM TRRD value in cycles. FC=2"]
pub type TrrdF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRRD_L_F2` reader - 15:8\\]
DRAM TRRD_L value in cycles. FC=2"]
pub type TrrdLF2R = crate::FieldReader;
#[doc = "Field `TRRD_L_F2` writer - 15:8\\]
DRAM TRRD_L value in cycles. FC=2"]
pub type TrrdLF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRC_F2` reader - 24:16\\]
DRAM TRC value in cycles. FC=2"]
pub type TrcF2R = crate::FieldReader<u16>;
#[doc = "Field `TRC_F2` writer - 24:16\\]
DRAM TRC value in cycles. FC=2"]
pub type TrcF2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRRD value in cycles. FC=2"]
    #[inline(always)]
    pub fn trrd_f2(&self) -> TrrdF2R {
        TrrdF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TRRD_L value in cycles. FC=2"]
    #[inline(always)]
    pub fn trrd_l_f2(&self) -> TrrdLF2R {
        TrrdLF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
DRAM TRC value in cycles. FC=2"]
    #[inline(always)]
    pub fn trc_f2(&self) -> TrcF2R {
        TrcF2R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRRD value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn trrd_f2(&mut self) -> TrrdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl51Spec> {
        TrrdF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TRRD_L value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn trrd_l_f2(&mut self) -> TrrdLF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl51Spec> {
        TrrdLF2W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
DRAM TRC value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn trc_f2(&mut self) -> TrcF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl51Spec> {
        TrcF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_51::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_51::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl51Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl51Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_51::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl51Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_51::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl51Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_51 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl51Spec {
    const RESET_VALUE: u32 = 0;
}
