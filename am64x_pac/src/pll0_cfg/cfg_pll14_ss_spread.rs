#[doc = "Register `CFG_pll14_SS_SPREAD` reader"]
pub type R = crate::R<CfgPll14SsSpreadSpec>;
#[doc = "Register `CFG_pll14_SS_SPREAD` writer"]
pub type W = crate::W<CfgPll14SsSpreadSpec>;
#[doc = "Field `SPREAD` reader - 4:0\\]
Sets the spread modulation depth. The depth is spread*0.1% 5'b00000 - Reserved (don't use) 5'b00001 - 0.1% 5'b00010 - 0.2% : 5'b10000 - 1.6% : 5'b11111 - 3.1%"]
pub type SpreadR = crate::FieldReader;
#[doc = "Field `SPREAD` writer - 4:0\\]
Sets the spread modulation depth. The depth is spread*0.1% 5'b00000 - Reserved (don't use) 5'b00001 - 0.1% 5'b00010 - 0.2% : 5'b10000 - 1.6% : 5'b11111 - 3.1%"]
pub type SpreadW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MOD_DIV` reader - 19:16\\]
Input clock divider. This divider sets the modulation frequency. Supports divide values of 1-63"]
pub type ModDivR = crate::FieldReader;
#[doc = "Field `MOD_DIV` writer - 19:16\\]
Input clock divider. This divider sets the modulation frequency. Supports divide values of 1-63"]
pub type ModDivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Sets the spread modulation depth. The depth is spread*0.1% 5'b00000 - Reserved (don't use) 5'b00001 - 0.1% 5'b00010 - 0.2% : 5'b10000 - 1.6% : 5'b11111 - 3.1%"]
    #[inline(always)]
    pub fn spread(&self) -> SpreadR {
        SpreadR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Input clock divider. This divider sets the modulation frequency. Supports divide values of 1-63"]
    #[inline(always)]
    pub fn mod_div(&self) -> ModDivR {
        ModDivR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Sets the spread modulation depth. The depth is spread*0.1% 5'b00000 - Reserved (don't use) 5'b00001 - 0.1% 5'b00010 - 0.2% : 5'b10000 - 1.6% : 5'b11111 - 3.1%"]
    #[inline(always)]
    #[must_use]
    pub fn spread(&mut self) -> SpreadW<CfgPll14SsSpreadSpec> {
        SpreadW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Input clock divider. This divider sets the modulation frequency. Supports divide values of 1-63"]
    #[inline(always)]
    #[must_use]
    pub fn mod_div(&mut self) -> ModDivW<CfgPll14SsSpreadSpec> {
        ModDivW::new(self, 16)
    }
}
#[doc = "CFG_pll14_SS_SPREAD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_ss_spread::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_ss_spread::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll14SsSpreadSpec;
impl crate::RegisterSpec for CfgPll14SsSpreadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll14_ss_spread::R`](R) reader structure"]
impl crate::Readable for CfgPll14SsSpreadSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll14_ss_spread::W`](W) writer structure"]
impl crate::Writable for CfgPll14SsSpreadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll14_SS_SPREAD to value 0x0001_0001"]
impl crate::Resettable for CfgPll14SsSpreadSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
