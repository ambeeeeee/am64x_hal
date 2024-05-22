#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_349` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl349Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_349` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl349Spec>;
#[doc = "Field `INT_ACK_MISC` reader - 15:0\\]
Clear status of the INT_STATUS_MISC parameter. WRITE-ONLY"]
pub type IntAckMiscR = crate::FieldReader<u16>;
#[doc = "Field `INT_ACK_MISC` writer - 15:0\\]
Clear status of the INT_STATUS_MISC parameter. WRITE-ONLY"]
pub type IntAckMiscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INT_ACK_BIST` reader - 23:16\\]
Clear status of the INT_STATUS_BIST parameter. WRITE-ONLY"]
pub type IntAckBistR = crate::FieldReader;
#[doc = "Field `INT_ACK_BIST` writer - 23:16\\]
Clear status of the INT_STATUS_BIST parameter. WRITE-ONLY"]
pub type IntAckBistW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Clear status of the INT_STATUS_MISC parameter. WRITE-ONLY"]
    #[inline(always)]
    pub fn int_ack_misc(&self) -> IntAckMiscR {
        IntAckMiscR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Clear status of the INT_STATUS_BIST parameter. WRITE-ONLY"]
    #[inline(always)]
    pub fn int_ack_bist(&self) -> IntAckBistR {
        IntAckBistR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Clear status of the INT_STATUS_MISC parameter. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_ack_misc(&mut self) -> IntAckMiscW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl349Spec> {
        IntAckMiscW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Clear status of the INT_STATUS_BIST parameter. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_ack_bist(&mut self) -> IntAckBistW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl349Spec> {
        IntAckBistW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_349\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_349::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_349::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl349Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl349Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_349::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl349Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_349::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl349Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_349 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl349Spec {
    const RESET_VALUE: u32 = 0;
}
