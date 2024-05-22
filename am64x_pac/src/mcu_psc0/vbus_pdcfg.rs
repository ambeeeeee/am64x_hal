#[doc = "Register `VBUS_PDCFG` reader"]
pub type R = crate::R<VbusPdcfgSpec>;
#[doc = "Register `VBUS_PDCFG` writer"]
pub type W = crate::W<VbusPdcfgSpec>;
#[doc = "Field `ALWAYSON` reader - 0:0\\]
Always on power domain"]
pub type AlwaysonR = crate::BitReader;
#[doc = "Field `ALWAYSON` writer - 0:0\\]
Always on power domain"]
pub type AlwaysonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMSLPKWK` reader - 1:1\\]
Memory sleep-wake domain"]
pub type MemslpkwkR = crate::BitReader;
#[doc = "Field `MEMSLPKWK` writer - 1:1\\]
Memory sleep-wake domain"]
pub type MemslpkwkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK` reader - 3:3\\]
Icepick support"]
pub type IcepickR = crate::BitReader;
#[doc = "Field `ICEPICK` writer - 3:3\\]
Icepick support"]
pub type IcepickW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Always on power domain"]
    #[inline(always)]
    pub fn alwayson(&self) -> AlwaysonR {
        AlwaysonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Memory sleep-wake domain"]
    #[inline(always)]
    pub fn memslpkwk(&self) -> MemslpkwkR {
        MemslpkwkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Icepick support"]
    #[inline(always)]
    pub fn icepick(&self) -> IcepickR {
        IcepickR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Always on power domain"]
    #[inline(always)]
    #[must_use]
    pub fn alwayson(&mut self) -> AlwaysonW<VbusPdcfgSpec> {
        AlwaysonW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Memory sleep-wake domain"]
    #[inline(always)]
    #[must_use]
    pub fn memslpkwk(&mut self) -> MemslpkwkW<VbusPdcfgSpec> {
        MemslpkwkW::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
Icepick support"]
    #[inline(always)]
    #[must_use]
    pub fn icepick(&mut self) -> IcepickW<VbusPdcfgSpec> {
        IcepickW::new(self, 3)
    }
}
#[doc = "This is a status register. It shows PSC settings for easy debug.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_pdcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_pdcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusPdcfgSpec;
impl crate::RegisterSpec for VbusPdcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_pdcfg::R`](R) reader structure"]
impl crate::Readable for VbusPdcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_pdcfg::W`](W) writer structure"]
impl crate::Writable for VbusPdcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_PDCFG to value 0"]
impl crate::Resettable for VbusPdcfgSpec {
    const RESET_VALUE: u32 = 0;
}
