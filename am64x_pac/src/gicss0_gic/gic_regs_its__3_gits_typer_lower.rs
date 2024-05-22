#[doc = "Register `GIC_REGS_ITS__3_GITS_TYPER_lower` reader"]
pub type R = crate::R<GicRegsIts_3GitsTyperLowerSpec>;
#[doc = "Register `GIC_REGS_ITS__3_GITS_TYPER_lower` writer"]
pub type W = crate::W<GicRegsIts_3GitsTyperLowerSpec>;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__0_1` reader - 0:0\\]
PLPIS"]
pub type Its_3GitsTyperLower_0_1R = crate::BitReader;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__0_1` writer - 0:0\\]
PLPIS"]
pub type Its_3GitsTyperLower_0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__1_1` reader - 1:1\\]
VLPIS"]
pub type Its_3GitsTyperLower_1_1R = crate::BitReader;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__1_1` writer - 1:1\\]
VLPIS"]
pub type Its_3GitsTyperLower_1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__3_1` reader - 3:3\\]
Distributed"]
pub type Its_3GitsTyperLower_3_1R = crate::BitReader;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__3_1` writer - 3:3\\]
Distributed"]
pub type Its_3GitsTyperLower_3_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__4_4` reader - 7:4\\]
ITT Entry Size"]
pub type Its_3GitsTyperLower_4_4R = crate::FieldReader;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__4_4` writer - 7:4\\]
ITT Entry Size"]
pub type Its_3GitsTyperLower_4_4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__8_5` reader - 12:8\\]
IDbits"]
pub type Its_3GitsTyperLower_8_5R = crate::FieldReader;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__8_5` writer - 12:8\\]
IDbits"]
pub type Its_3GitsTyperLower_8_5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__13_5` reader - 17:13\\]
Devbits"]
pub type Its_3GitsTyperLower_13_5R = crate::FieldReader;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__13_5` writer - 17:13\\]
Devbits"]
pub type Its_3GitsTyperLower_13_5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__19_1` reader - 19:19\\]
PTA"]
pub type Its_3GitsTyperLower_19_1R = crate::BitReader;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__19_1` writer - 19:19\\]
PTA"]
pub type Its_3GitsTyperLower_19_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__24_8` reader - 31:24\\]
HCC"]
pub type Its_3GitsTyperLower_24_8R = crate::FieldReader;
#[doc = "Field `ITS__3_GITS_TYPER_LOWER__24_8` writer - 31:24\\]
HCC"]
pub type Its_3GitsTyperLower_24_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
PLPIS"]
    #[inline(always)]
    pub fn its__3_gits_typer_lower__0_1(&self) -> Its_3GitsTyperLower_0_1R {
        Its_3GitsTyperLower_0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
VLPIS"]
    #[inline(always)]
    pub fn its__3_gits_typer_lower__1_1(&self) -> Its_3GitsTyperLower_1_1R {
        Its_3GitsTyperLower_1_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Distributed"]
    #[inline(always)]
    pub fn its__3_gits_typer_lower__3_1(&self) -> Its_3GitsTyperLower_3_1R {
        Its_3GitsTyperLower_3_1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
ITT Entry Size"]
    #[inline(always)]
    pub fn its__3_gits_typer_lower__4_4(&self) -> Its_3GitsTyperLower_4_4R {
        Its_3GitsTyperLower_4_4R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
IDbits"]
    #[inline(always)]
    pub fn its__3_gits_typer_lower__8_5(&self) -> Its_3GitsTyperLower_8_5R {
        Its_3GitsTyperLower_8_5R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:17 - 17:13\\]
Devbits"]
    #[inline(always)]
    pub fn its__3_gits_typer_lower__13_5(&self) -> Its_3GitsTyperLower_13_5R {
        Its_3GitsTyperLower_13_5R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
PTA"]
    #[inline(always)]
    pub fn its__3_gits_typer_lower__19_1(&self) -> Its_3GitsTyperLower_19_1R {
        Its_3GitsTyperLower_19_1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
HCC"]
    #[inline(always)]
    pub fn its__3_gits_typer_lower__24_8(&self) -> Its_3GitsTyperLower_24_8R {
        Its_3GitsTyperLower_24_8R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
PLPIS"]
    #[inline(always)]
    #[must_use]
    pub fn its__3_gits_typer_lower__0_1(
        &mut self,
    ) -> Its_3GitsTyperLower_0_1W<GicRegsIts_3GitsTyperLowerSpec> {
        Its_3GitsTyperLower_0_1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
VLPIS"]
    #[inline(always)]
    #[must_use]
    pub fn its__3_gits_typer_lower__1_1(
        &mut self,
    ) -> Its_3GitsTyperLower_1_1W<GicRegsIts_3GitsTyperLowerSpec> {
        Its_3GitsTyperLower_1_1W::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
Distributed"]
    #[inline(always)]
    #[must_use]
    pub fn its__3_gits_typer_lower__3_1(
        &mut self,
    ) -> Its_3GitsTyperLower_3_1W<GicRegsIts_3GitsTyperLowerSpec> {
        Its_3GitsTyperLower_3_1W::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
ITT Entry Size"]
    #[inline(always)]
    #[must_use]
    pub fn its__3_gits_typer_lower__4_4(
        &mut self,
    ) -> Its_3GitsTyperLower_4_4W<GicRegsIts_3GitsTyperLowerSpec> {
        Its_3GitsTyperLower_4_4W::new(self, 4)
    }
    #[doc = "Bits 8:12 - 12:8\\]
IDbits"]
    #[inline(always)]
    #[must_use]
    pub fn its__3_gits_typer_lower__8_5(
        &mut self,
    ) -> Its_3GitsTyperLower_8_5W<GicRegsIts_3GitsTyperLowerSpec> {
        Its_3GitsTyperLower_8_5W::new(self, 8)
    }
    #[doc = "Bits 13:17 - 17:13\\]
Devbits"]
    #[inline(always)]
    #[must_use]
    pub fn its__3_gits_typer_lower__13_5(
        &mut self,
    ) -> Its_3GitsTyperLower_13_5W<GicRegsIts_3GitsTyperLowerSpec> {
        Its_3GitsTyperLower_13_5W::new(self, 13)
    }
    #[doc = "Bit 19 - 19:19\\]
PTA"]
    #[inline(always)]
    #[must_use]
    pub fn its__3_gits_typer_lower__19_1(
        &mut self,
    ) -> Its_3GitsTyperLower_19_1W<GicRegsIts_3GitsTyperLowerSpec> {
        Its_3GitsTyperLower_19_1W::new(self, 19)
    }
    #[doc = "Bits 24:31 - 31:24\\]
HCC"]
    #[inline(always)]
    #[must_use]
    pub fn its__3_gits_typer_lower__24_8(
        &mut self,
    ) -> Its_3GitsTyperLower_24_8W<GicRegsIts_3GitsTyperLowerSpec> {
        Its_3GitsTyperLower_24_8W::new(self, 24)
    }
}
#[doc = "GITS_TYPER_lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__3_gits_typer_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__3_gits_typer_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_3GitsTyperLowerSpec;
impl crate::RegisterSpec for GicRegsIts_3GitsTyperLowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__3_gits_typer_lower::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_3GitsTyperLowerSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__3_gits_typer_lower::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_3GitsTyperLowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__3_GITS_TYPER_lower to value 0x0304_1531"]
impl crate::Resettable for GicRegsIts_3GitsTyperLowerSpec {
    const RESET_VALUE: u32 = 0x0304_1531;
}
