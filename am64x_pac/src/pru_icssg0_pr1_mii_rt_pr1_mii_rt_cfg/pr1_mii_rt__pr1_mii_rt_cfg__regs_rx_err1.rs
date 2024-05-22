#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_err1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_err1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1Spec>;
#[doc = "Field `RX_MIN_PCNT_ERR1` reader - 0:0\\]
rx_min_pcnt_err1"]
pub type RxMinPcntErr1R = crate::BitReader;
#[doc = "Field `RX_MIN_PCNT_ERR1` writer - 0:0\\]
rx_min_pcnt_err1"]
pub type RxMinPcntErr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MAX_PCNT_ERR1` reader - 1:1\\]
rx_max_pcnt_err1"]
pub type RxMaxPcntErr1R = crate::BitReader;
#[doc = "Field `RX_MAX_PCNT_ERR1` writer - 1:1\\]
rx_max_pcnt_err1"]
pub type RxMaxPcntErr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MIN_FRM_ERR1` reader - 2:2\\]
rx_min_frm_err1"]
pub type RxMinFrmErr1R = crate::BitReader;
#[doc = "Field `RX_MIN_FRM_ERR1` writer - 2:2\\]
rx_min_frm_err1"]
pub type RxMinFrmErr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MAX_FRM_ERR1` reader - 3:3\\]
rx_max_frm_err1"]
pub type RxMaxFrmErr1R = crate::BitReader;
#[doc = "Field `RX_MAX_FRM_ERR1` writer - 3:3\\]
rx_max_frm_err1"]
pub type RxMaxFrmErr1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
rx_min_pcnt_err1"]
    #[inline(always)]
    pub fn rx_min_pcnt_err1(&self) -> RxMinPcntErr1R {
        RxMinPcntErr1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
rx_max_pcnt_err1"]
    #[inline(always)]
    pub fn rx_max_pcnt_err1(&self) -> RxMaxPcntErr1R {
        RxMaxPcntErr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
rx_min_frm_err1"]
    #[inline(always)]
    pub fn rx_min_frm_err1(&self) -> RxMinFrmErr1R {
        RxMinFrmErr1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
rx_max_frm_err1"]
    #[inline(always)]
    pub fn rx_max_frm_err1(&self) -> RxMaxFrmErr1R {
        RxMaxFrmErr1R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
rx_min_pcnt_err1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_min_pcnt_err1(&mut self) -> RxMinPcntErr1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1Spec> {
        RxMinPcntErr1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
rx_max_pcnt_err1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_max_pcnt_err1(&mut self) -> RxMaxPcntErr1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1Spec> {
        RxMaxPcntErr1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
rx_min_frm_err1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_min_frm_err1(&mut self) -> RxMinFrmErr1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1Spec> {
        RxMinFrmErr1W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
rx_max_frm_err1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_max_frm_err1(&mut self) -> RxMaxFrmErr1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1Spec> {
        RxMaxFrmErr1W::new(self, 3)
    }
}
#[doc = "MIIRXERR1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_err1 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1Spec {
    const RESET_VALUE: u32 = 0;
}
