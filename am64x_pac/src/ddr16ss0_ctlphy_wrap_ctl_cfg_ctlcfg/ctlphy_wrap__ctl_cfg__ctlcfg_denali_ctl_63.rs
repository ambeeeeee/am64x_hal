#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_63` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl63Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_63` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl63Spec>;
#[doc = "Field `WRITEINTERP` reader - 8:8\\]
Allow controller to interrupt a write burst to the DRAMs with a read command. Set to 1 to allow interruption."]
pub type WriteinterpR = crate::BitReader;
#[doc = "Field `WRITEINTERP` writer - 8:8\\]
Allow controller to interrupt a write burst to the DRAMs with a read command. Set to 1 to allow interruption."]
pub type WriteinterpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCD_F0` reader - 23:16\\]
DRAM TRCD value in cycles. FC=0"]
pub type TrcdF0R = crate::FieldReader;
#[doc = "Field `TRCD_F0` writer - 23:16\\]
DRAM TRCD value in cycles. FC=0"]
pub type TrcdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TWR_F0` reader - 31:24\\]
DRAM TWR value in cycles. FC=0"]
pub type TwrF0R = crate::FieldReader;
#[doc = "Field `TWR_F0` writer - 31:24\\]
DRAM TWR value in cycles. FC=0"]
pub type TwrF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
Allow controller to interrupt a write burst to the DRAMs with a read command. Set to 1 to allow interruption."]
    #[inline(always)]
    pub fn writeinterp(&self) -> WriteinterpR {
        WriteinterpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TRCD value in cycles. FC=0"]
    #[inline(always)]
    pub fn trcd_f0(&self) -> TrcdF0R {
        TrcdF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TWR value in cycles. FC=0"]
    #[inline(always)]
    pub fn twr_f0(&self) -> TwrF0R {
        TwrF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
Allow controller to interrupt a write burst to the DRAMs with a read command. Set to 1 to allow interruption."]
    #[inline(always)]
    #[must_use]
    pub fn writeinterp(&mut self) -> WriteinterpW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl63Spec> {
        WriteinterpW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TRCD value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn trcd_f0(&mut self) -> TrcdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl63Spec> {
        TrcdF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TWR value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn twr_f0(&mut self) -> TwrF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl63Spec> {
        TwrF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_63::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_63::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl63Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl63Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_63::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl63Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_63::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl63Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_63 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl63Spec {
    const RESET_VALUE: u32 = 0;
}
