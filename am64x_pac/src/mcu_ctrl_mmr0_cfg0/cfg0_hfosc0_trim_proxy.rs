#[doc = "Register `CFG0_HFOSC0_TRIM_PROXY` reader"]
pub type R = crate::R<Cfg0Hfosc0TrimProxySpec>;
#[doc = "Register `CFG0_HFOSC0_TRIM_PROXY` writer"]
pub type W = crate::W<Cfg0Hfosc0TrimProxySpec>;
#[doc = "Field `HFOSC0_TRIM_R_IBIAS_REF_PROXY` reader - 3:0\\]
Sets the base IBIAS reference"]
pub type Hfosc0TrimRIbiasRefProxyR = crate::FieldReader;
#[doc = "Field `HFOSC0_TRIM_R_IBIAS_REF_PROXY` writer - 3:0\\]
Sets the base IBIAS reference"]
pub type Hfosc0TrimRIbiasRefProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HFOSC0_TRIM_I_IBIAS_COMP_PROXY` reader - 7:4\\]
Sets the COMP bias current"]
pub type Hfosc0TrimIIbiasCompProxyR = crate::FieldReader;
#[doc = "Field `HFOSC0_TRIM_I_IBIAS_COMP_PROXY` writer - 7:4\\]
Sets the COMP bias current"]
pub type Hfosc0TrimIIbiasCompProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HFOSC0_TRIM_R_REF_PROXY` reader - 13:8\\]
Sets the AMP AGC bias current"]
pub type Hfosc0TrimRRefProxyR = crate::FieldReader;
#[doc = "Field `HFOSC0_TRIM_R_REF_PROXY` writer - 13:8\\]
Sets the AMP AGC bias current"]
pub type Hfosc0TrimRRefProxyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HFOSC0_TRIM_I_MULT_PROXY` reader - 18:16\\]
AGC AMP current multiplication gain"]
pub type Hfosc0TrimIMultProxyR = crate::FieldReader;
#[doc = "Field `HFOSC0_TRIM_I_MULT_PROXY` writer - 18:16\\]
AGC AMP current multiplication gain"]
pub type Hfosc0TrimIMultProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HFOSC0_TRIM_HYST_PROXY` reader - 21:20\\]
Sets comparator hysterisis"]
pub type Hfosc0TrimHystProxyR = crate::FieldReader;
#[doc = "Field `HFOSC0_TRIM_HYST_PROXY` writer - 21:20\\]
Sets comparator hysterisis"]
pub type Hfosc0TrimHystProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HFOSC0_TRIM_TRIM_EN_PROXY` reader - 31:31\\]
Apply MMR values to OSC trim inputs instead of tie-offs"]
pub type Hfosc0TrimTrimEnProxyR = crate::BitReader;
#[doc = "Field `HFOSC0_TRIM_TRIM_EN_PROXY` writer - 31:31\\]
Apply MMR values to OSC trim inputs instead of tie-offs"]
pub type Hfosc0TrimTrimEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Sets the base IBIAS reference"]
    #[inline(always)]
    pub fn hfosc0_trim_r_ibias_ref_proxy(&self) -> Hfosc0TrimRIbiasRefProxyR {
        Hfosc0TrimRIbiasRefProxyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Sets the COMP bias current"]
    #[inline(always)]
    pub fn hfosc0_trim_i_ibias_comp_proxy(&self) -> Hfosc0TrimIIbiasCompProxyR {
        Hfosc0TrimIIbiasCompProxyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Sets the AMP AGC bias current"]
    #[inline(always)]
    pub fn hfosc0_trim_r_ref_proxy(&self) -> Hfosc0TrimRRefProxyR {
        Hfosc0TrimRRefProxyR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
AGC AMP current multiplication gain"]
    #[inline(always)]
    pub fn hfosc0_trim_i_mult_proxy(&self) -> Hfosc0TrimIMultProxyR {
        Hfosc0TrimIMultProxyR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Sets comparator hysterisis"]
    #[inline(always)]
    pub fn hfosc0_trim_hyst_proxy(&self) -> Hfosc0TrimHystProxyR {
        Hfosc0TrimHystProxyR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Apply MMR values to OSC trim inputs instead of tie-offs"]
    #[inline(always)]
    pub fn hfosc0_trim_trim_en_proxy(&self) -> Hfosc0TrimTrimEnProxyR {
        Hfosc0TrimTrimEnProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Sets the base IBIAS reference"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_trim_r_ibias_ref_proxy(
        &mut self,
    ) -> Hfosc0TrimRIbiasRefProxyW<Cfg0Hfosc0TrimProxySpec> {
        Hfosc0TrimRIbiasRefProxyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Sets the COMP bias current"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_trim_i_ibias_comp_proxy(
        &mut self,
    ) -> Hfosc0TrimIIbiasCompProxyW<Cfg0Hfosc0TrimProxySpec> {
        Hfosc0TrimIIbiasCompProxyW::new(self, 4)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Sets the AMP AGC bias current"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_trim_r_ref_proxy(&mut self) -> Hfosc0TrimRRefProxyW<Cfg0Hfosc0TrimProxySpec> {
        Hfosc0TrimRRefProxyW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
AGC AMP current multiplication gain"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_trim_i_mult_proxy(&mut self) -> Hfosc0TrimIMultProxyW<Cfg0Hfosc0TrimProxySpec> {
        Hfosc0TrimIMultProxyW::new(self, 16)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Sets comparator hysterisis"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_trim_hyst_proxy(&mut self) -> Hfosc0TrimHystProxyW<Cfg0Hfosc0TrimProxySpec> {
        Hfosc0TrimHystProxyW::new(self, 20)
    }
    #[doc = "Bit 31 - 31:31\\]
Apply MMR values to OSC trim inputs instead of tie-offs"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_trim_trim_en_proxy(&mut self) -> Hfosc0TrimTrimEnProxyW<Cfg0Hfosc0TrimProxySpec> {
        Hfosc0TrimTrimEnProxyW::new(self, 31)
    }
}
#[doc = "CFG0_HFOSC0_TRIM_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_hfosc0_trim_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_hfosc0_trim_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Hfosc0TrimProxySpec;
impl crate::RegisterSpec for Cfg0Hfosc0TrimProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_hfosc0_trim_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Hfosc0TrimProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_hfosc0_trim_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Hfosc0TrimProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_HFOSC0_TRIM_PROXY to value 0x0013_1622"]
impl crate::Resettable for Cfg0Hfosc0TrimProxySpec {
    const RESET_VALUE: u32 = 0x0013_1622;
}
