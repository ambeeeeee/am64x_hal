#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_class_cfg2_pru0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassCfg2Pru0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_class_cfg2_pru0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassCfg2Pru0Spec>;
#[doc = "Field `RX_CLASS_AND_NV` reader - 15:0\\]
rx class and nv enable"]
pub type RxClassAndNvR = crate::FieldReader<u16>;
#[doc = "Field `RX_CLASS_AND_NV` writer - 15:0\\]
rx class and nv enable"]
pub type RxClassAndNvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RX_CLASS_OR_NV` reader - 31:16\\]
rx class or nv enable"]
pub type RxClassOrNvR = crate::FieldReader<u16>;
#[doc = "Field `RX_CLASS_OR_NV` writer - 31:16\\]
rx class or nv enable"]
pub type RxClassOrNvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
rx class and nv enable"]
    #[inline(always)]
    pub fn rx_class_and_nv(&self) -> RxClassAndNvR {
        RxClassAndNvR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
rx class or nv enable"]
    #[inline(always)]
    pub fn rx_class_or_nv(&self) -> RxClassOrNvR {
        RxClassOrNvR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
rx class and nv enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_class_and_nv(
        &mut self,
    ) -> RxClassAndNvW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassCfg2Pru0Spec> {
        RxClassAndNvW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
rx class or nv enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_class_or_nv(
        &mut self,
    ) -> RxClassOrNvW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassCfg2Pru0Spec> {
        RxClassOrNvW::new(self, 16)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_class_cfg2_pru0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_class_cfg2_pru0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_class_cfg2_pru0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassCfg2Pru0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassCfg2Pru0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_class_cfg2_pru0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassCfg2Pru0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_class_cfg2_pru0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassCfg2Pru0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_class_cfg2_pru0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassCfg2Pru0Spec {
    const RESET_VALUE: u32 = 0;
}
