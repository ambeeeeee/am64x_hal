#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_fifo_level0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_fifo_level0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel0Spec>;
#[doc = "Field `RX_FIFO_LEVEL0` reader - 7:0\\]
rx_fifo_level0"]
pub type RxFifoLevel0R = crate::FieldReader;
#[doc = "Field `RX_FIFO_LEVEL0` writer - 7:0\\]
rx_fifo_level0"]
pub type RxFifoLevel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
rx_fifo_level0"]
    #[inline(always)]
    pub fn rx_fifo_level0(&self) -> RxFifoLevel0R {
        RxFifoLevel0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
rx_fifo_level0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_level0(&mut self) -> RxFifoLevel0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel0Spec> {
        RxFifoLevel0W::new(self, 0)
    }
}
#[doc = "MIIRXFIFOLEVEL0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_fifo_level0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel0Spec {
    const RESET_VALUE: u32 = 0;
}
