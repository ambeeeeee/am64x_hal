#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_frms1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_frms1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1Spec>;
#[doc = "Field `RX_MIN_FRM1` reader - 15:0\\]
rx_min_frm1"]
pub type RxMinFrm1R = crate::FieldReader<u16>;
#[doc = "Field `RX_MIN_FRM1` writer - 15:0\\]
rx_min_frm1"]
pub type RxMinFrm1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RX_MAX_FRM1` reader - 31:16\\]
rx_max_frm1"]
pub type RxMaxFrm1R = crate::FieldReader<u16>;
#[doc = "Field `RX_MAX_FRM1` writer - 31:16\\]
rx_max_frm1"]
pub type RxMaxFrm1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
rx_min_frm1"]
    #[inline(always)]
    pub fn rx_min_frm1(&self) -> RxMinFrm1R {
        RxMinFrm1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
rx_max_frm1"]
    #[inline(always)]
    pub fn rx_max_frm1(&self) -> RxMaxFrm1R {
        RxMaxFrm1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
rx_min_frm1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_min_frm1(&mut self) -> RxMinFrm1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1Spec> {
        RxMinFrm1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
rx_max_frm1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_max_frm1(&mut self) -> RxMaxFrm1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1Spec> {
        RxMaxFrm1W::new(self, 16)
    }
}
#[doc = "MIIRXFRMS1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_frms1 to value 0x1521_0063"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1Spec {
    const RESET_VALUE: u32 = 0x1521_0063;
}
