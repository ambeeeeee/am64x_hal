#[doc = "Register `INTAGGR_MCAST_mcmap` reader"]
pub type R = crate::R<IntaggrMcastMcmapSpec>;
#[doc = "Register `INTAGGR_MCAST_mcmap` writer"]
pub type W = crate::W<IntaggrMcastMcmapSpec>;
#[doc = "Field `GEVIDX0` reader - 15:0\\]
Global event index 0. This field specifies the index of the outgoing global event on ETL 0. Set to 0xFFFF to disable."]
pub type Gevidx0R = crate::FieldReader<u16>;
#[doc = "Field `GEVIDX0` writer - 15:0\\]
Global event index 0. This field specifies the index of the outgoing global event on ETL 0. Set to 0xFFFF to disable."]
pub type Gevidx0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IRQMODE0` reader - 31:31\\]
IRQ Mode Flag 0. When set, this register act like a mapper with bitnum in 5:0 and regnum in 14:6."]
pub type Irqmode0R = crate::BitReader;
#[doc = "Field `IRQMODE0` writer - 31:31\\]
IRQ Mode Flag 0. When set, this register act like a mapper with bitnum in 5:0 and regnum in 14:6."]
pub type Irqmode0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEVIDX1` reader - 47:32\\]
Global event index 1. This field specifies the index of the outgoing global event on ETL 1. Set to 0xFFFF to disable."]
pub type Gevidx1R = crate::FieldReader<u16>;
#[doc = "Field `GEVIDX1` writer - 47:32\\]
Global event index 1. This field specifies the index of the outgoing global event on ETL 1. Set to 0xFFFF to disable."]
pub type Gevidx1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IRQMODE1` reader - 63:63\\]
IRQ Mode Flag 1. When set, this register act like a mapper with bitnum in 37:32 and regnum in 46:38."]
pub type Irqmode1R = crate::BitReader;
#[doc = "Field `IRQMODE1` writer - 63:63\\]
IRQ Mode Flag 1. When set, this register act like a mapper with bitnum in 37:32 and regnum in 46:38."]
pub type Irqmode1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Global event index 0. This field specifies the index of the outgoing global event on ETL 0. Set to 0xFFFF to disable."]
    #[inline(always)]
    pub fn gevidx0(&self) -> Gevidx0R {
        Gevidx0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
IRQ Mode Flag 0. When set, this register act like a mapper with bitnum in 5:0 and regnum in 14:6."]
    #[inline(always)]
    pub fn irqmode0(&self) -> Irqmode0R {
        Irqmode0R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 32:47 - 47:32\\]
Global event index 1. This field specifies the index of the outgoing global event on ETL 1. Set to 0xFFFF to disable."]
    #[inline(always)]
    pub fn gevidx1(&self) -> Gevidx1R {
        Gevidx1R::new(((self.bits >> 32) & 0xffff) as u16)
    }
    #[doc = "Bit 63 - 63:63\\]
IRQ Mode Flag 1. When set, this register act like a mapper with bitnum in 37:32 and regnum in 46:38."]
    #[inline(always)]
    pub fn irqmode1(&self) -> Irqmode1R {
        Irqmode1R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Global event index 0. This field specifies the index of the outgoing global event on ETL 0. Set to 0xFFFF to disable."]
    #[inline(always)]
    #[must_use]
    pub fn gevidx0(&mut self) -> Gevidx0W<IntaggrMcastMcmapSpec> {
        Gevidx0W::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
IRQ Mode Flag 0. When set, this register act like a mapper with bitnum in 5:0 and regnum in 14:6."]
    #[inline(always)]
    #[must_use]
    pub fn irqmode0(&mut self) -> Irqmode0W<IntaggrMcastMcmapSpec> {
        Irqmode0W::new(self, 31)
    }
    #[doc = "Bits 32:47 - 47:32\\]
Global event index 1. This field specifies the index of the outgoing global event on ETL 1. Set to 0xFFFF to disable."]
    #[inline(always)]
    #[must_use]
    pub fn gevidx1(&mut self) -> Gevidx1W<IntaggrMcastMcmapSpec> {
        Gevidx1W::new(self, 32)
    }
    #[doc = "Bit 63 - 63:63\\]
IRQ Mode Flag 1. When set, this register act like a mapper with bitnum in 37:32 and regnum in 46:38."]
    #[inline(always)]
    #[must_use]
    pub fn irqmode1(&mut self) -> Irqmode1W<IntaggrMcastMcmapSpec> {
        Irqmode1W::new(self, 63)
    }
}
#[doc = "This register determines how ingress global events from the ingress global event ETL are written out to the two egress global event ETL intefaces. The index of each of the two egress events is stored in this register, which is selected based in the ingress event index value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_mcast_mcmap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_mcast_mcmap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaggrMcastMcmapSpec;
impl crate::RegisterSpec for IntaggrMcastMcmapSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intaggr_mcast_mcmap::R`](R) reader structure"]
impl crate::Readable for IntaggrMcastMcmapSpec {}
#[doc = "`write(|w| ..)` method takes [`intaggr_mcast_mcmap::W`](W) writer structure"]
impl crate::Writable for IntaggrMcastMcmapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets INTAGGR_MCAST_mcmap to value 0x0006_5535_0000_0000"]
impl crate::Resettable for IntaggrMcastMcmapSpec {
    const RESET_VALUE: u64 = 0x0006_5535_0000_0000;
}
