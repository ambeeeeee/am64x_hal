#[doc = "Register `CFG0_HFOSC0_TRIM` reader"]
pub type R = crate::R<Cfg0Hfosc0TrimSpec>;
#[doc = "Register `CFG0_HFOSC0_TRIM` writer"]
pub type W = crate::W<Cfg0Hfosc0TrimSpec>;
#[doc = "Field `HFOSC0_TRIM_R_IBIAS_REF` reader - 3:0\\]
Sets the base IBIAS reference"]
pub type Hfosc0TrimRIbiasRefR = crate::FieldReader;
#[doc = "Field `HFOSC0_TRIM_R_IBIAS_REF` writer - 3:0\\]
Sets the base IBIAS reference"]
pub type Hfosc0TrimRIbiasRefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HFOSC0_TRIM_I_IBIAS_COMP` reader - 7:4\\]
Sets the COMP bias current"]
pub type Hfosc0TrimIIbiasCompR = crate::FieldReader;
#[doc = "Field `HFOSC0_TRIM_I_IBIAS_COMP` writer - 7:4\\]
Sets the COMP bias current"]
pub type Hfosc0TrimIIbiasCompW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HFOSC0_TRIM_R_REF` reader - 13:8\\]
Sets the AMP AGC bias current"]
pub type Hfosc0TrimRRefR = crate::FieldReader;
#[doc = "Field `HFOSC0_TRIM_R_REF` writer - 13:8\\]
Sets the AMP AGC bias current"]
pub type Hfosc0TrimRRefW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HFOSC0_TRIM_I_MULT` reader - 18:16\\]
AGC AMP current multiplication gain"]
pub type Hfosc0TrimIMultR = crate::FieldReader;
#[doc = "Field `HFOSC0_TRIM_I_MULT` writer - 18:16\\]
AGC AMP current multiplication gain"]
pub type Hfosc0TrimIMultW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HFOSC0_TRIM_HYST` reader - 21:20\\]
Sets comparator hysterisis"]
pub type Hfosc0TrimHystR = crate::FieldReader;
#[doc = "Field `HFOSC0_TRIM_HYST` writer - 21:20\\]
Sets comparator hysterisis"]
pub type Hfosc0TrimHystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HFOSC0_TRIM_TRIM_EN` reader - 31:31\\]
Apply MMR values to OSC trim inputs instead of tie-offs"]
pub type Hfosc0TrimTrimEnR = crate::BitReader;
#[doc = "Field `HFOSC0_TRIM_TRIM_EN` writer - 31:31\\]
Apply MMR values to OSC trim inputs instead of tie-offs"]
pub type Hfosc0TrimTrimEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Sets the base IBIAS reference"]
    #[inline(always)]
    pub fn hfosc0_trim_r_ibias_ref(&self) -> Hfosc0TrimRIbiasRefR {
        Hfosc0TrimRIbiasRefR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Sets the COMP bias current"]
    #[inline(always)]
    pub fn hfosc0_trim_i_ibias_comp(&self) -> Hfosc0TrimIIbiasCompR {
        Hfosc0TrimIIbiasCompR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Sets the AMP AGC bias current"]
    #[inline(always)]
    pub fn hfosc0_trim_r_ref(&self) -> Hfosc0TrimRRefR {
        Hfosc0TrimRRefR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
AGC AMP current multiplication gain"]
    #[inline(always)]
    pub fn hfosc0_trim_i_mult(&self) -> Hfosc0TrimIMultR {
        Hfosc0TrimIMultR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Sets comparator hysterisis"]
    #[inline(always)]
    pub fn hfosc0_trim_hyst(&self) -> Hfosc0TrimHystR {
        Hfosc0TrimHystR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Apply MMR values to OSC trim inputs instead of tie-offs"]
    #[inline(always)]
    pub fn hfosc0_trim_trim_en(&self) -> Hfosc0TrimTrimEnR {
        Hfosc0TrimTrimEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Sets the base IBIAS reference"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_trim_r_ibias_ref(&mut self) -> Hfosc0TrimRIbiasRefW<Cfg0Hfosc0TrimSpec> {
        Hfosc0TrimRIbiasRefW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Sets the COMP bias current"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_trim_i_ibias_comp(&mut self) -> Hfosc0TrimIIbiasCompW<Cfg0Hfosc0TrimSpec> {
        Hfosc0TrimIIbiasCompW::new(self, 4)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Sets the AMP AGC bias current"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_trim_r_ref(&mut self) -> Hfosc0TrimRRefW<Cfg0Hfosc0TrimSpec> {
        Hfosc0TrimRRefW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
AGC AMP current multiplication gain"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_trim_i_mult(&mut self) -> Hfosc0TrimIMultW<Cfg0Hfosc0TrimSpec> {
        Hfosc0TrimIMultW::new(self, 16)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Sets comparator hysterisis"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_trim_hyst(&mut self) -> Hfosc0TrimHystW<Cfg0Hfosc0TrimSpec> {
        Hfosc0TrimHystW::new(self, 20)
    }
    #[doc = "Bit 31 - 31:31\\]
Apply MMR values to OSC trim inputs instead of tie-offs"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_trim_trim_en(&mut self) -> Hfosc0TrimTrimEnW<Cfg0Hfosc0TrimSpec> {
        Hfosc0TrimTrimEnW::new(self, 31)
    }
}
#[doc = "CFG0_HFOSC0_TRIM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_hfosc0_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_hfosc0_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Hfosc0TrimSpec;
impl crate::RegisterSpec for Cfg0Hfosc0TrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_hfosc0_trim::R`](R) reader structure"]
impl crate::Readable for Cfg0Hfosc0TrimSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_hfosc0_trim::W`](W) writer structure"]
impl crate::Writable for Cfg0Hfosc0TrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_HFOSC0_TRIM to value 0x0013_1622"]
impl crate::Resettable for Cfg0Hfosc0TrimSpec {
    const RESET_VALUE: u32 = 0x0013_1622;
}
