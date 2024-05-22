#[doc = "Register `MMR__VBUSP__CFG1_CTRL` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg1CtrlSpec>;
#[doc = "Register `MMR__VBUSP__CFG1_CTRL` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg1CtrlSpec>;
#[doc = "Field `GT_TH1_EN` reader - 8:8\\]
Enable over-threshold1 event. Temp event/level control: 0 = Don't generate output level. 1 = Enable generation of event/level high when temperature reading is > value set for threshold point 1. Reset value is POR only."]
pub type GtTh1EnR = crate::BitReader;
#[doc = "Field `GT_TH1_EN` writer - 8:8\\]
Enable over-threshold1 event. Temp event/level control: 0 = Don't generate output level. 1 = Enable generation of event/level high when temperature reading is > value set for threshold point 1. Reset value is POR only."]
pub type GtTh1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GT_TH2_EN` reader - 9:9\\]
Enable over-threshold2 event. Temp event/level control: 0 = Don't generate output level. 1 = Enable generation of event/level high when temperature reading is > value set for threshold point 2. Reset value is POR only."]
pub type GtTh2EnR = crate::BitReader;
#[doc = "Field `GT_TH2_EN` writer - 9:9\\]
Enable over-threshold2 event. Temp event/level control: 0 = Don't generate output level. 1 = Enable generation of event/level high when temperature reading is > value set for threshold point 2. Reset value is POR only."]
pub type GtTh2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LT_TH0_EN` reader - 10:10\\]
Enable under-threshold0 event. Temp event/level control: 0 = Don't generate output level. 1 = Enable generation of event/level high when temperature reading is &lt; value set for threshold point 0. Reset value is POR only."]
pub type LtTh0EnR = crate::BitReader;
#[doc = "Field `LT_TH0_EN` writer - 10:10\\]
Enable under-threshold0 event. Temp event/level control: 0 = Don't generate output level. 1 = Enable generation of event/level high when temperature reading is &lt; value set for threshold point 0. Reset value is POR only."]
pub type LtTh0EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
Enable over-threshold1 event. Temp event/level control: 0 = Don't generate output level. 1 = Enable generation of event/level high when temperature reading is > value set for threshold point 1. Reset value is POR only."]
    #[inline(always)]
    pub fn gt_th1_en(&self) -> GtTh1EnR {
        GtTh1EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable over-threshold2 event. Temp event/level control: 0 = Don't generate output level. 1 = Enable generation of event/level high when temperature reading is > value set for threshold point 2. Reset value is POR only."]
    #[inline(always)]
    pub fn gt_th2_en(&self) -> GtTh2EnR {
        GtTh2EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable under-threshold0 event. Temp event/level control: 0 = Don't generate output level. 1 = Enable generation of event/level high when temperature reading is &lt; value set for threshold point 0. Reset value is POR only."]
    #[inline(always)]
    pub fn lt_th0_en(&self) -> LtTh0EnR {
        LtTh0EnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
Enable over-threshold1 event. Temp event/level control: 0 = Don't generate output level. 1 = Enable generation of event/level high when temperature reading is > value set for threshold point 1. Reset value is POR only."]
    #[inline(always)]
    #[must_use]
    pub fn gt_th1_en(&mut self) -> GtTh1EnW<Mmr_Vbusp_Cfg1CtrlSpec> {
        GtTh1EnW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable over-threshold2 event. Temp event/level control: 0 = Don't generate output level. 1 = Enable generation of event/level high when temperature reading is > value set for threshold point 2. Reset value is POR only."]
    #[inline(always)]
    #[must_use]
    pub fn gt_th2_en(&mut self) -> GtTh2EnW<Mmr_Vbusp_Cfg1CtrlSpec> {
        GtTh2EnW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable under-threshold0 event. Temp event/level control: 0 = Don't generate output level. 1 = Enable generation of event/level high when temperature reading is &lt; value set for threshold point 0. Reset value is POR only."]
    #[inline(always)]
    #[must_use]
    pub fn lt_th0_en(&mut self) -> LtTh0EnW<Mmr_Vbusp_Cfg1CtrlSpec> {
        LtTh0EnW::new(self, 10)
    }
}
#[doc = "Temperature Sensor Band-gap control register for sensor a.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg1CtrlSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg1_ctrl::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg1_ctrl::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg1CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG1_CTRL to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg1CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
