#[doc = "Register `CFG0_RC12M_OSC_TRIM_PROXY` reader"]
pub type R = crate::R<Cfg0Rc12mOscTrimProxySpec>;
#[doc = "Register `CFG0_RC12M_OSC_TRIM_PROXY` writer"]
pub type W = crate::W<Cfg0Rc12mOscTrimProxySpec>;
#[doc = "Field `RC12M_OSC_TRIM_TRIMOSC_FINE_PROXY` reader - 2:0\\]
Fine adjustment. Decreases the frequency by 250 KHz per value."]
pub type Rc12mOscTrimTrimoscFineProxyR = crate::FieldReader;
#[doc = "Field `RC12M_OSC_TRIM_TRIMOSC_FINE_PROXY` writer - 2:0\\]
Fine adjustment. Decreases the frequency by 250 KHz per value."]
pub type Rc12mOscTrimTrimoscFineProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RC12M_OSC_TRIM_TRIMOSC_COARSE_PROXY` reader - 5:3\\]
Coarse adjustment. Frequency is decreased or increased by 1.25 MHz per value based on the trimosc_coarse_dir value."]
pub type Rc12mOscTrimTrimoscCoarseProxyR = crate::FieldReader;
#[doc = "Field `RC12M_OSC_TRIM_TRIMOSC_COARSE_PROXY` writer - 5:3\\]
Coarse adjustment. Frequency is decreased or increased by 1.25 MHz per value based on the trimosc_coarse_dir value."]
pub type Rc12mOscTrimTrimoscCoarseProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RC12M_OSC_TRIM_TRIMOSC_COARSE_DIR_PROXY` reader - 6:6\\]
Coarse adjustment direction. If output is greater than 12.5"]
pub type Rc12mOscTrimTrimoscCoarseDirProxyR = crate::BitReader;
#[doc = "Field `RC12M_OSC_TRIM_TRIMOSC_COARSE_DIR_PROXY` writer - 6:6\\]
Coarse adjustment direction. If output is greater than 12.5"]
pub type Rc12mOscTrimTrimoscCoarseDirProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Fine adjustment. Decreases the frequency by 250 KHz per value."]
    #[inline(always)]
    pub fn rc12m_osc_trim_trimosc_fine_proxy(&self) -> Rc12mOscTrimTrimoscFineProxyR {
        Rc12mOscTrimTrimoscFineProxyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Coarse adjustment. Frequency is decreased or increased by 1.25 MHz per value based on the trimosc_coarse_dir value."]
    #[inline(always)]
    pub fn rc12m_osc_trim_trimosc_coarse_proxy(&self) -> Rc12mOscTrimTrimoscCoarseProxyR {
        Rc12mOscTrimTrimoscCoarseProxyR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Coarse adjustment direction. If output is greater than 12.5"]
    #[inline(always)]
    pub fn rc12m_osc_trim_trimosc_coarse_dir_proxy(&self) -> Rc12mOscTrimTrimoscCoarseDirProxyR {
        Rc12mOscTrimTrimoscCoarseDirProxyR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Fine adjustment. Decreases the frequency by 250 KHz per value."]
    #[inline(always)]
    #[must_use]
    pub fn rc12m_osc_trim_trimosc_fine_proxy(
        &mut self,
    ) -> Rc12mOscTrimTrimoscFineProxyW<Cfg0Rc12mOscTrimProxySpec> {
        Rc12mOscTrimTrimoscFineProxyW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Coarse adjustment. Frequency is decreased or increased by 1.25 MHz per value based on the trimosc_coarse_dir value."]
    #[inline(always)]
    #[must_use]
    pub fn rc12m_osc_trim_trimosc_coarse_proxy(
        &mut self,
    ) -> Rc12mOscTrimTrimoscCoarseProxyW<Cfg0Rc12mOscTrimProxySpec> {
        Rc12mOscTrimTrimoscCoarseProxyW::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
Coarse adjustment direction. If output is greater than 12.5"]
    #[inline(always)]
    #[must_use]
    pub fn rc12m_osc_trim_trimosc_coarse_dir_proxy(
        &mut self,
    ) -> Rc12mOscTrimTrimoscCoarseDirProxyW<Cfg0Rc12mOscTrimProxySpec> {
        Rc12mOscTrimTrimoscCoarseDirProxyW::new(self, 6)
    }
}
#[doc = "CFG0_RC12M_OSC_TRIM_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rc12m_osc_trim_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rc12m_osc_trim_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Rc12mOscTrimProxySpec;
impl crate::RegisterSpec for Cfg0Rc12mOscTrimProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_rc12m_osc_trim_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Rc12mOscTrimProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_rc12m_osc_trim_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Rc12mOscTrimProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_RC12M_OSC_TRIM_PROXY to value 0"]
impl crate::Resettable for Cfg0Rc12mOscTrimProxySpec {
    const RESET_VALUE: u32 = 0;
}
