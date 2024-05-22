#[doc = "Register `APBADDR_DBG_CPU0_EDPIDR1` reader"]
pub type R = crate::R<ApbaddrDbgCpu0Edpidr1Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_EDPIDR1` writer"]
pub type W = crate::W<ApbaddrDbgCpu0Edpidr1Spec>;
#[doc = "Field `PART_1` reader - 3:0\\]
Part number, most significant nibble."]
pub type Part1R = crate::FieldReader;
#[doc = "Field `PART_1` writer - 3:0\\]
Part number, most significant nibble."]
pub type Part1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DES_0` reader - 7:4\\]
Designer, least significant nibble of JEP106 ID code. For ARM Limited, this field is 0b1011."]
pub type Des0R = crate::FieldReader;
#[doc = "Field `DES_0` writer - 7:4\\]
Designer, least significant nibble of JEP106 ID code. For ARM Limited, this field is 0b1011."]
pub type Des0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_EDPIDR1_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Edpidr1_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_EDPIDR1_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Edpidr1_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Part number, most significant nibble."]
    #[inline(always)]
    pub fn part_1(&self) -> Part1R {
        Part1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Designer, least significant nibble of JEP106 ID code. For ARM Limited, this field is 0b1011."]
    #[inline(always)]
    pub fn des_0(&self) -> Des0R {
        Des0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edpidr1_31_8(&self) -> Res0Edpidr1_31_8R {
        Res0Edpidr1_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Part number, most significant nibble."]
    #[inline(always)]
    #[must_use]
    pub fn part_1(&mut self) -> Part1W<ApbaddrDbgCpu0Edpidr1Spec> {
        Part1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Designer, least significant nibble of JEP106 ID code. For ARM Limited, this field is 0b1011."]
    #[inline(always)]
    #[must_use]
    pub fn des_0(&mut self) -> Des0W<ApbaddrDbgCpu0Edpidr1Spec> {
        Des0W::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edpidr1_31_8(&mut self) -> Res0Edpidr1_31_8W<ApbaddrDbgCpu0Edpidr1Spec> {
        Res0Edpidr1_31_8W::new(self, 8)
    }
}
#[doc = "External Debug Peripheral Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edpidr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edpidr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0Edpidr1Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0Edpidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_edpidr1::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0Edpidr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_edpidr1::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0Edpidr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_EDPIDR1 to value 0x0113"]
impl crate::Resettable for ApbaddrDbgCpu0Edpidr1Spec {
    const RESET_VALUE: u32 = 0x0113;
}
