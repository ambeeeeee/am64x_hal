#[doc = "Register `APBADDR_ETM_CPU0_TRCEVENTCTL0R` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trceventctl0rSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCEVENTCTL0R` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trceventctl0rSpec>;
#[doc = "Field `SEL0` reader - 3:0\\]
Selects the resource number based on the value of TYPE0"]
pub type Sel0R = crate::FieldReader;
#[doc = "Field `SEL0` writer - 3:0\\]
Selects the resource number based on the value of TYPE0"]
pub type Sel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TYPE0` reader - 7:7\\]
Selects the resource type for trace event 0"]
pub type Type0R = crate::BitReader;
#[doc = "Field `TYPE0` writer - 7:7\\]
Selects the resource type for trace event 0"]
pub type Type0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL1` reader - 11:8\\]
Selects the resource number based on the value of TYPE1"]
pub type Sel1R = crate::FieldReader;
#[doc = "Field `SEL1` writer - 11:8\\]
Selects the resource number based on the value of TYPE1"]
pub type Sel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TYPE1` reader - 15:15\\]
Selects the resource type for trace event 1"]
pub type Type1R = crate::BitReader;
#[doc = "Field `TYPE1` writer - 15:15\\]
Selects the resource type for trace event 1"]
pub type Type1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL2` reader - 19:16\\]
Selects the resource number based on the value of TYPE2"]
pub type Sel2R = crate::FieldReader;
#[doc = "Field `SEL2` writer - 19:16\\]
Selects the resource number based on the value of TYPE2"]
pub type Sel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TYPE2` reader - 23:23\\]
Selects the resource type for trace event 2"]
pub type Type2R = crate::BitReader;
#[doc = "Field `TYPE2` writer - 23:23\\]
Selects the resource type for trace event 2"]
pub type Type2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL3` reader - 27:24\\]
Selects the resource number based on the value of TYPE3"]
pub type Sel3R = crate::FieldReader;
#[doc = "Field `SEL3` writer - 27:24\\]
Selects the resource number based on the value of TYPE3"]
pub type Sel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TYPE3` reader - 31:31\\]
Selects the resource type for trace event 3"]
pub type Type3R = crate::BitReader;
#[doc = "Field `TYPE3` writer - 31:31\\]
Selects the resource type for trace event 3"]
pub type Type3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Selects the resource number based on the value of TYPE0"]
    #[inline(always)]
    pub fn sel0(&self) -> Sel0R {
        Sel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Selects the resource type for trace event 0"]
    #[inline(always)]
    pub fn type0(&self) -> Type0R {
        Type0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Selects the resource number based on the value of TYPE1"]
    #[inline(always)]
    pub fn sel1(&self) -> Sel1R {
        Sel1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Selects the resource type for trace event 1"]
    #[inline(always)]
    pub fn type1(&self) -> Type1R {
        Type1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Selects the resource number based on the value of TYPE2"]
    #[inline(always)]
    pub fn sel2(&self) -> Sel2R {
        Sel2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Selects the resource type for trace event 2"]
    #[inline(always)]
    pub fn type2(&self) -> Type2R {
        Type2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Selects the resource number based on the value of TYPE3"]
    #[inline(always)]
    pub fn sel3(&self) -> Sel3R {
        Sel3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Selects the resource type for trace event 3"]
    #[inline(always)]
    pub fn type3(&self) -> Type3R {
        Type3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Selects the resource number based on the value of TYPE0"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> Sel0W<ApbaddrEtmCpu0Trceventctl0rSpec> {
        Sel0W::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Selects the resource type for trace event 0"]
    #[inline(always)]
    #[must_use]
    pub fn type0(&mut self) -> Type0W<ApbaddrEtmCpu0Trceventctl0rSpec> {
        Type0W::new(self, 7)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Selects the resource number based on the value of TYPE1"]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> Sel1W<ApbaddrEtmCpu0Trceventctl0rSpec> {
        Sel1W::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
Selects the resource type for trace event 1"]
    #[inline(always)]
    #[must_use]
    pub fn type1(&mut self) -> Type1W<ApbaddrEtmCpu0Trceventctl0rSpec> {
        Type1W::new(self, 15)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Selects the resource number based on the value of TYPE2"]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> Sel2W<ApbaddrEtmCpu0Trceventctl0rSpec> {
        Sel2W::new(self, 16)
    }
    #[doc = "Bit 23 - 23:23\\]
Selects the resource type for trace event 2"]
    #[inline(always)]
    #[must_use]
    pub fn type2(&mut self) -> Type2W<ApbaddrEtmCpu0Trceventctl0rSpec> {
        Type2W::new(self, 23)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Selects the resource number based on the value of TYPE3"]
    #[inline(always)]
    #[must_use]
    pub fn sel3(&mut self) -> Sel3W<ApbaddrEtmCpu0Trceventctl0rSpec> {
        Sel3W::new(self, 24)
    }
    #[doc = "Bit 31 - 31:31\\]
Selects the resource type for trace event 3"]
    #[inline(always)]
    #[must_use]
    pub fn type3(&mut self) -> Type3W<ApbaddrEtmCpu0Trceventctl0rSpec> {
        Type3W::new(self, 31)
    }
}
#[doc = "Event Control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trceventctl0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trceventctl0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trceventctl0rSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trceventctl0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trceventctl0r::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trceventctl0rSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trceventctl0r::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trceventctl0rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCEVENTCTL0R to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trceventctl0rSpec {
    const RESET_VALUE: u32 = 0;
}
