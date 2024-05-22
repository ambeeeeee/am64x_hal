#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_pcnt1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_pcnt1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1Spec>;
#[doc = "Field `RX_MIN_PCNT1` reader - 3:0\\]
rx_min_pcnt1"]
pub type RxMinPcnt1R = crate::FieldReader;
#[doc = "Field `RX_MIN_PCNT1` writer - 3:0\\]
rx_min_pcnt1"]
pub type RxMinPcnt1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_MAX_PCNT1` reader - 8:4\\]
rx_max_pcnt1"]
pub type RxMaxPcnt1R = crate::FieldReader;
#[doc = "Field `RX_MAX_PCNT1` writer - 8:4\\]
rx_max_pcnt1"]
pub type RxMaxPcnt1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
rx_min_pcnt1"]
    #[inline(always)]
    pub fn rx_min_pcnt1(&self) -> RxMinPcnt1R {
        RxMinPcnt1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - 8:4\\]
rx_max_pcnt1"]
    #[inline(always)]
    pub fn rx_max_pcnt1(&self) -> RxMaxPcnt1R {
        RxMaxPcnt1R::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
rx_min_pcnt1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_min_pcnt1(&mut self) -> RxMinPcnt1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1Spec> {
        RxMinPcnt1W::new(self, 0)
    }
    #[doc = "Bits 4:8 - 8:4\\]
rx_max_pcnt1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_max_pcnt1(&mut self) -> RxMaxPcnt1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1Spec> {
        RxMaxPcnt1W::new(self, 4)
    }
}
#[doc = "MIIRXPCNT1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_pcnt1 to value 0x0141"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1Spec {
    const RESET_VALUE: u32 = 0x0141;
}
