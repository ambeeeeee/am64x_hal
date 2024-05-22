#[doc = "Register `INTAGGR_UNMAP_map` reader"]
pub type R = crate::R<IntaggrUnmapMapSpec>;
#[doc = "Register `INTAGGR_UNMAP_map` writer"]
pub type W = crate::W<IntaggrUnmapMapSpec>;
#[doc = "Field `UMAPIDX` reader - 15:0\\]
Global event index. This field specifies the index of the outgoing global event. Set to 0xFFFF to disable."]
pub type UmapidxR = crate::FieldReader<u16>;
#[doc = "Field `UMAPIDX` writer - 15:0\\]
Global event index. This field specifies the index of the outgoing global event. Set to 0xFFFF to disable."]
pub type UmapidxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IRQMODE` reader - 31:31\\]
IRQ Mode Flag. When set, this register act like a mapper with bitnum in 5:0 and regnum in 14:6."]
pub type IrqmodeR = crate::BitReader;
#[doc = "Field `IRQMODE` writer - 31:31\\]
IRQ Mode Flag. When set, this register act like a mapper with bitnum in 5:0 and regnum in 14:6."]
pub type IrqmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Global event index. This field specifies the index of the outgoing global event. Set to 0xFFFF to disable."]
    #[inline(always)]
    pub fn umapidx(&self) -> UmapidxR {
        UmapidxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
IRQ Mode Flag. When set, this register act like a mapper with bitnum in 5:0 and regnum in 14:6."]
    #[inline(always)]
    pub fn irqmode(&self) -> IrqmodeR {
        IrqmodeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Global event index. This field specifies the index of the outgoing global event. Set to 0xFFFF to disable."]
    #[inline(always)]
    #[must_use]
    pub fn umapidx(&mut self) -> UmapidxW<IntaggrUnmapMapSpec> {
        UmapidxW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
IRQ Mode Flag. When set, this register act like a mapper with bitnum in 5:0 and regnum in 14:6."]
    #[inline(always)]
    #[must_use]
    pub fn irqmode(&mut self) -> IrqmodeW<IntaggrUnmapMapSpec> {
        IrqmodeW::new(self, 31)
    }
}
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaggrUnmapMapSpec;
impl crate::RegisterSpec for IntaggrUnmapMapSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intaggr_unmap_map::R`](R) reader structure"]
impl crate::Readable for IntaggrUnmapMapSpec {}
#[doc = "`write(|w| ..)` method takes [`intaggr_unmap_map::W`](W) writer structure"]
impl crate::Writable for IntaggrUnmapMapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets INTAGGR_UNMAP_map to value 0"]
impl crate::Resettable for IntaggrUnmapMapSpec {
    const RESET_VALUE: u64 = 0;
}
