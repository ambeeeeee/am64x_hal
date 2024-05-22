#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_class_gates15_pru0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates15Pru0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_class_gates15_pru0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates15Pru0Spec>;
#[doc = "Field `RX_RATE_GATE_SEL15` reader - 2:0\\]
defines which rx_rate will gate rx_class"]
pub type RxRateGateSel15R = crate::FieldReader;
#[doc = "Field `RX_RATE_GATE_SEL15` writer - 2:0\\]
defines which rx_rate will gate rx_class"]
pub type RxRateGateSel15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_PHASE_MASK15` reader - 4:4\\]
time phase mask 0: unmask 1: mask"]
pub type RxPhaseMask15R = crate::BitReader;
#[doc = "Field `RX_PHASE_MASK15` writer - 4:4\\]
time phase mask 0: unmask 1: mask"]
pub type RxPhaseMask15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CLASS_RAW_MASK15` reader - 5:5\\]
class raw mask 0: unmask 1: mask"]
pub type RxClassRawMask15R = crate::BitReader;
#[doc = "Field `RX_CLASS_RAW_MASK15` writer - 5:5\\]
class raw mask 0: unmask 1: mask"]
pub type RxClassRawMask15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ALLOW_MASK15` reader - 6:6\\]
allow mask 0: unmask 1: mask"]
pub type RxAllowMask15R = crate::BitReader;
#[doc = "Field `RX_ALLOW_MASK15` writer - 6:6\\]
allow mask 0: unmask 1: mask"]
pub type RxAllowMask15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_RED_PHASE_EN15` reader - 8:8\\]
red phase neable 0: disable 1: enable"]
pub type RxRedPhaseEn15R = crate::BitReader;
#[doc = "Field `RX_RED_PHASE_EN15` writer - 8:8\\]
red phase neable 0: disable 1: enable"]
pub type RxRedPhaseEn15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
defines which rx_rate will gate rx_class"]
    #[inline(always)]
    pub fn rx_rate_gate_sel15(&self) -> RxRateGateSel15R {
        RxRateGateSel15R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
time phase mask 0: unmask 1: mask"]
    #[inline(always)]
    pub fn rx_phase_mask15(&self) -> RxPhaseMask15R {
        RxPhaseMask15R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
class raw mask 0: unmask 1: mask"]
    #[inline(always)]
    pub fn rx_class_raw_mask15(&self) -> RxClassRawMask15R {
        RxClassRawMask15R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
allow mask 0: unmask 1: mask"]
    #[inline(always)]
    pub fn rx_allow_mask15(&self) -> RxAllowMask15R {
        RxAllowMask15R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
red phase neable 0: disable 1: enable"]
    #[inline(always)]
    pub fn rx_red_phase_en15(&self) -> RxRedPhaseEn15R {
        RxRedPhaseEn15R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
defines which rx_rate will gate rx_class"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rate_gate_sel15(
        &mut self,
    ) -> RxRateGateSel15W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates15Pru0Spec> {
        RxRateGateSel15W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
time phase mask 0: unmask 1: mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_phase_mask15(
        &mut self,
    ) -> RxPhaseMask15W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates15Pru0Spec> {
        RxPhaseMask15W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
class raw mask 0: unmask 1: mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_class_raw_mask15(
        &mut self,
    ) -> RxClassRawMask15W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates15Pru0Spec> {
        RxClassRawMask15W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
allow mask 0: unmask 1: mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_allow_mask15(
        &mut self,
    ) -> RxAllowMask15W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates15Pru0Spec> {
        RxAllowMask15W::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
red phase neable 0: disable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_red_phase_en15(
        &mut self,
    ) -> RxRedPhaseEn15W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates15Pru0Spec> {
        RxRedPhaseEn15W::new(self, 8)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_class_gates15_pru0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_class_gates15_pru0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_class_gates15_pru0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates15Pru0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates15Pru0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_class_gates15_pru0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates15Pru0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_class_gates15_pru0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates15Pru0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_class_gates15_pru0 to value 0x70"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxClassGates15Pru0Spec {
    const RESET_VALUE: u32 = 0x70;
}
