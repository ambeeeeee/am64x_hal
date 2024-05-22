#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_class_gates2_pru0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates2Pru0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_class_gates2_pru0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates2Pru0Spec>;
#[doc = "Field `RX_RATE_GATE_SEL2` reader - 2:0\\]
defines which rx_rate will gate rx_class"]
pub type RxRateGateSel2R = crate::FieldReader;
#[doc = "Field `RX_RATE_GATE_SEL2` writer - 2:0\\]
defines which rx_rate will gate rx_class"]
pub type RxRateGateSel2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_PHASE_MASK2` reader - 4:4\\]
time phase mask 0: unmask 1: mask"]
pub type RxPhaseMask2R = crate::BitReader;
#[doc = "Field `RX_PHASE_MASK2` writer - 4:4\\]
time phase mask 0: unmask 1: mask"]
pub type RxPhaseMask2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CLASS_RAW_MASK2` reader - 5:5\\]
class raw mask 0: unmask 1: mask"]
pub type RxClassRawMask2R = crate::BitReader;
#[doc = "Field `RX_CLASS_RAW_MASK2` writer - 5:5\\]
class raw mask 0: unmask 1: mask"]
pub type RxClassRawMask2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ALLOW_MASK2` reader - 6:6\\]
allow mask 0: unmask 1: mask"]
pub type RxAllowMask2R = crate::BitReader;
#[doc = "Field `RX_ALLOW_MASK2` writer - 6:6\\]
allow mask 0: unmask 1: mask"]
pub type RxAllowMask2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_RED_PHASE_EN2` reader - 8:8\\]
red phase neable 0: disable 1: enable"]
pub type RxRedPhaseEn2R = crate::BitReader;
#[doc = "Field `RX_RED_PHASE_EN2` writer - 8:8\\]
red phase neable 0: disable 1: enable"]
pub type RxRedPhaseEn2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
defines which rx_rate will gate rx_class"]
    #[inline(always)]
    pub fn rx_rate_gate_sel2(&self) -> RxRateGateSel2R {
        RxRateGateSel2R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
time phase mask 0: unmask 1: mask"]
    #[inline(always)]
    pub fn rx_phase_mask2(&self) -> RxPhaseMask2R {
        RxPhaseMask2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
class raw mask 0: unmask 1: mask"]
    #[inline(always)]
    pub fn rx_class_raw_mask2(&self) -> RxClassRawMask2R {
        RxClassRawMask2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
allow mask 0: unmask 1: mask"]
    #[inline(always)]
    pub fn rx_allow_mask2(&self) -> RxAllowMask2R {
        RxAllowMask2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
red phase neable 0: disable 1: enable"]
    #[inline(always)]
    pub fn rx_red_phase_en2(&self) -> RxRedPhaseEn2R {
        RxRedPhaseEn2R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
defines which rx_rate will gate rx_class"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rate_gate_sel2(
        &mut self,
    ) -> RxRateGateSel2W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates2Pru0Spec> {
        RxRateGateSel2W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
time phase mask 0: unmask 1: mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_phase_mask2(
        &mut self,
    ) -> RxPhaseMask2W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates2Pru0Spec> {
        RxPhaseMask2W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
class raw mask 0: unmask 1: mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_class_raw_mask2(
        &mut self,
    ) -> RxClassRawMask2W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates2Pru0Spec> {
        RxClassRawMask2W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
allow mask 0: unmask 1: mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_allow_mask2(
        &mut self,
    ) -> RxAllowMask2W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates2Pru0Spec> {
        RxAllowMask2W::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
red phase neable 0: disable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_red_phase_en2(
        &mut self,
    ) -> RxRedPhaseEn2W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates2Pru0Spec> {
        RxRedPhaseEn2W::new(self, 8)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_class_gates2_pru0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_class_gates2_pru0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_class_gates2_pru0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates2Pru0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates2Pru0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_class_gates2_pru0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates2Pru0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_class_gates2_pru0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates2Pru0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_class_gates2_pru0 to value 0x70"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates2Pru0Spec {
    const RESET_VALUE: u32 = 0x70;
}
