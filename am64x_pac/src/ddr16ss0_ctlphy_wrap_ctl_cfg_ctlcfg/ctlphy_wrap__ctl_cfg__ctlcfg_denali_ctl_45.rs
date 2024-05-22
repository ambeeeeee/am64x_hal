#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_45` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl45Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_45` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl45Spec>;
#[doc = "Field `TRRD_L_F0` reader - 7:0\\]
DRAM TRRD_L value in cycles. FC=0"]
pub type TrrdLF0R = crate::FieldReader;
#[doc = "Field `TRRD_L_F0` writer - 7:0\\]
DRAM TRRD_L value in cycles. FC=0"]
pub type TrrdLF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRC_F0` reader - 16:8\\]
DRAM TRC value in cycles. FC=0"]
pub type TrcF0R = crate::FieldReader<u16>;
#[doc = "Field `TRC_F0` writer - 16:8\\]
DRAM TRC value in cycles. FC=0"]
pub type TrcF0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRRD_L value in cycles. FC=0"]
    #[inline(always)]
    pub fn trrd_l_f0(&self) -> TrrdLF0R {
        TrrdLF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - 16:8\\]
DRAM TRC value in cycles. FC=0"]
    #[inline(always)]
    pub fn trc_f0(&self) -> TrcF0R {
        TrcF0R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRRD_L value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn trrd_l_f0(&mut self) -> TrrdLF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl45Spec> {
        TrrdLF0W::new(self, 0)
    }
    #[doc = "Bits 8:16 - 16:8\\]
DRAM TRC value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn trc_f0(&mut self) -> TrcF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl45Spec> {
        TrcF0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl45Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_45::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl45Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_45::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_45 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl45Spec {
    const RESET_VALUE: u32 = 0;
}
