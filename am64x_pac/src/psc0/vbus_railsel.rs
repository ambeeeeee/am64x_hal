#[doc = "Register `VBUS_RAILSEL` reader"]
pub type R = crate::R<VbusRailselSpec>;
#[doc = "Register `VBUS_RAILSEL` writer"]
pub type W = crate::W<VbusRailselSpec>;
#[doc = "Field `P` reader - 31:0\\]
Rail Counter Select for Power Domain"]
pub type PR = crate::FieldReader<u32>;
#[doc = "Field `P` writer - 31:0\\]
Rail Counter Select for Power Domain"]
pub type PW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Rail Counter Select for Power Domain"]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Rail Counter Select for Power Domain"]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> PW<VbusRailselSpec> {
        PW::new(self, 0)
    }
}
#[doc = "User can use this register to select the counter value (RAILCTL) for each power domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_railsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_railsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusRailselSpec;
impl crate::RegisterSpec for VbusRailselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_railsel::R`](R) reader structure"]
impl crate::Readable for VbusRailselSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_railsel::W`](W) writer structure"]
impl crate::Writable for VbusRailselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_RAILSEL to value 0"]
impl crate::Resettable for VbusRailselSpec {
    const RESET_VALUE: u32 = 0;
}
