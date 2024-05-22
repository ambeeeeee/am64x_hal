#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_err0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_err0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0Spec>;
#[doc = "Field `RX_MIN_PCNT_ERR0` reader - 0:0\\]
rx_min_pcnt_err0"]
pub type RxMinPcntErr0R = crate::BitReader;
#[doc = "Field `RX_MIN_PCNT_ERR0` writer - 0:0\\]
rx_min_pcnt_err0"]
pub type RxMinPcntErr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MAX_PCNT_ERR0` reader - 1:1\\]
rx_max_pcnt_err0"]
pub type RxMaxPcntErr0R = crate::BitReader;
#[doc = "Field `RX_MAX_PCNT_ERR0` writer - 1:1\\]
rx_max_pcnt_err0"]
pub type RxMaxPcntErr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MIN_FRM_ERR0` reader - 2:2\\]
rx_min_frm_err0"]
pub type RxMinFrmErr0R = crate::BitReader;
#[doc = "Field `RX_MIN_FRM_ERR0` writer - 2:2\\]
rx_min_frm_err0"]
pub type RxMinFrmErr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MAX_FRM_ERR0` reader - 3:3\\]
rx_max_frm_err0"]
pub type RxMaxFrmErr0R = crate::BitReader;
#[doc = "Field `RX_MAX_FRM_ERR0` writer - 3:3\\]
rx_max_frm_err0"]
pub type RxMaxFrmErr0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
rx_min_pcnt_err0"]
    #[inline(always)]
    pub fn rx_min_pcnt_err0(&self) -> RxMinPcntErr0R {
        RxMinPcntErr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
rx_max_pcnt_err0"]
    #[inline(always)]
    pub fn rx_max_pcnt_err0(&self) -> RxMaxPcntErr0R {
        RxMaxPcntErr0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
rx_min_frm_err0"]
    #[inline(always)]
    pub fn rx_min_frm_err0(&self) -> RxMinFrmErr0R {
        RxMinFrmErr0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
rx_max_frm_err0"]
    #[inline(always)]
    pub fn rx_max_frm_err0(&self) -> RxMaxFrmErr0R {
        RxMaxFrmErr0R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
rx_min_pcnt_err0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_min_pcnt_err0(&mut self) -> RxMinPcntErr0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0Spec> {
        RxMinPcntErr0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
rx_max_pcnt_err0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_max_pcnt_err0(&mut self) -> RxMaxPcntErr0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0Spec> {
        RxMaxPcntErr0W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
rx_min_frm_err0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_min_frm_err0(&mut self) -> RxMinFrmErr0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0Spec> {
        RxMinFrmErr0W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
rx_max_frm_err0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_max_frm_err0(&mut self) -> RxMaxFrmErr0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0Spec> {
        RxMaxFrmErr0W::new(self, 3)
    }
}
#[doc = "MIIRXERR0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_err0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0Spec {
    const RESET_VALUE: u32 = 0;
}
