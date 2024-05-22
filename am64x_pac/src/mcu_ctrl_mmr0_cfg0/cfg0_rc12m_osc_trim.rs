#[doc = "Register `CFG0_RC12M_OSC_TRIM` reader"]
pub type R = crate::R<Cfg0Rc12mOscTrimSpec>;
#[doc = "Register `CFG0_RC12M_OSC_TRIM` writer"]
pub type W = crate::W<Cfg0Rc12mOscTrimSpec>;
#[doc = "Field `RC12M_OSC_TRIM_TRIMOSC_FINE` reader - 2:0\\]
Fine adjustment. Decreases the frequency by 250 KHz per value."]
pub type Rc12mOscTrimTrimoscFineR = crate::FieldReader;
#[doc = "Field `RC12M_OSC_TRIM_TRIMOSC_FINE` writer - 2:0\\]
Fine adjustment. Decreases the frequency by 250 KHz per value."]
pub type Rc12mOscTrimTrimoscFineW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RC12M_OSC_TRIM_TRIMOSC_COARSE` reader - 5:3\\]
Coarse adjustment. Frequency is decreased or increased by 1.25 MHz per value based on the trimosc_coarse_dir value."]
pub type Rc12mOscTrimTrimoscCoarseR = crate::FieldReader;
#[doc = "Field `RC12M_OSC_TRIM_TRIMOSC_COARSE` writer - 5:3\\]
Coarse adjustment. Frequency is decreased or increased by 1.25 MHz per value based on the trimosc_coarse_dir value."]
pub type Rc12mOscTrimTrimoscCoarseW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RC12M_OSC_TRIM_TRIMOSC_COARSE_DIR` reader - 6:6\\]
Coarse adjustment direction. If output is greater than 12.5"]
pub type Rc12mOscTrimTrimoscCoarseDirR = crate::BitReader;
#[doc = "Field `RC12M_OSC_TRIM_TRIMOSC_COARSE_DIR` writer - 6:6\\]
Coarse adjustment direction. If output is greater than 12.5"]
pub type Rc12mOscTrimTrimoscCoarseDirW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Fine adjustment. Decreases the frequency by 250 KHz per value."]
    #[inline(always)]
    pub fn rc12m_osc_trim_trimosc_fine(&self) -> Rc12mOscTrimTrimoscFineR {
        Rc12mOscTrimTrimoscFineR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Coarse adjustment. Frequency is decreased or increased by 1.25 MHz per value based on the trimosc_coarse_dir value."]
    #[inline(always)]
    pub fn rc12m_osc_trim_trimosc_coarse(&self) -> Rc12mOscTrimTrimoscCoarseR {
        Rc12mOscTrimTrimoscCoarseR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Coarse adjustment direction. If output is greater than 12.5"]
    #[inline(always)]
    pub fn rc12m_osc_trim_trimosc_coarse_dir(&self) -> Rc12mOscTrimTrimoscCoarseDirR {
        Rc12mOscTrimTrimoscCoarseDirR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Fine adjustment. Decreases the frequency by 250 KHz per value."]
    #[inline(always)]
    #[must_use]
    pub fn rc12m_osc_trim_trimosc_fine(
        &mut self,
    ) -> Rc12mOscTrimTrimoscFineW<Cfg0Rc12mOscTrimSpec> {
        Rc12mOscTrimTrimoscFineW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Coarse adjustment. Frequency is decreased or increased by 1.25 MHz per value based on the trimosc_coarse_dir value."]
    #[inline(always)]
    #[must_use]
    pub fn rc12m_osc_trim_trimosc_coarse(
        &mut self,
    ) -> Rc12mOscTrimTrimoscCoarseW<Cfg0Rc12mOscTrimSpec> {
        Rc12mOscTrimTrimoscCoarseW::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
Coarse adjustment direction. If output is greater than 12.5"]
    #[inline(always)]
    #[must_use]
    pub fn rc12m_osc_trim_trimosc_coarse_dir(
        &mut self,
    ) -> Rc12mOscTrimTrimoscCoarseDirW<Cfg0Rc12mOscTrimSpec> {
        Rc12mOscTrimTrimoscCoarseDirW::new(self, 6)
    }
}
#[doc = "CFG0_RC12M_OSC_TRIM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rc12m_osc_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rc12m_osc_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Rc12mOscTrimSpec;
impl crate::RegisterSpec for Cfg0Rc12mOscTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_rc12m_osc_trim::R`](R) reader structure"]
impl crate::Readable for Cfg0Rc12mOscTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_rc12m_osc_trim::W`](W) writer structure"]
impl crate::Writable for Cfg0Rc12mOscTrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_RC12M_OSC_TRIM to value 0"]
impl crate::Resettable for Cfg0Rc12mOscTrimSpec {
    const RESET_VALUE: u32 = 0;
}
