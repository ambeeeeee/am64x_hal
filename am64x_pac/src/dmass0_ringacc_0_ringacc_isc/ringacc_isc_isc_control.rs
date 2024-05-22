#[doc = "Register `RINGACC_ISC_ISC_control` reader"]
pub type R = crate::R<RingaccIscIscControlSpec>;
#[doc = "Register `RINGACC_ISC_ISC_control` writer"]
pub type W = crate::W<RingaccIscIscControlSpec>;
#[doc = "Field `ENABLE` reader - 3:0\\]
Enable region. A value of 0xA enables, others disable."]
pub type EnableR = crate::FieldReader;
#[doc = "Field `ENABLE` writer - 3:0\\]
Enable region. A value of 0xA enables, others disable."]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOCK` reader - 4:4\\]
Lock region. Once set the region values cannot be modified."]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - 4:4\\]
Lock region. Once set the region values cannot be modified."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_ID` reader - 15:8\\]
Priv ID."]
pub type PrivIdR = crate::FieldReader;
#[doc = "Field `PRIV_ID` writer - 15:8\\]
Priv ID."]
pub type PrivIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEC` reader - 19:16\\]
Make outgoing secure. A value of 0xA enables, others disable."]
pub type SecR = crate::FieldReader;
#[doc = "Field `SEC` writer - 19:16\\]
Make outgoing secure. A value of 0xA enables, others disable."]
pub type SecW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NONSEC` reader - 20:20\\]
Make outgoing non-secure. Has precedence over secure enable bits."]
pub type NonsecR = crate::BitReader;
#[doc = "Field `NONSEC` writer - 20:20\\]
Make outgoing non-secure. Has precedence over secure enable bits."]
pub type NonsecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PASS` reader - 21:21\\]
No privID replacement, pass through value."]
pub type PassR = crate::BitReader;
#[doc = "Field `PASS` writer - 21:21\\]
No privID replacement, pass through value."]
pub type PassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV` reader - 25:24\\]
Set outgoing priv attribute. If each bit is set then the outgoing priv bit is set."]
pub type PrivR = crate::FieldReader;
#[doc = "Field `PRIV` writer - 25:24\\]
Set outgoing priv attribute. If each bit is set then the outgoing priv bit is set."]
pub type PrivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOPRIV` reader - 27:26\\]
Clear output priv attribute. If each bit is set then the outgoing priv bit is cleared. Has precedence over priv set bits."]
pub type NoprivR = crate::FieldReader;
#[doc = "Field `NOPRIV` writer - 27:26\\]
Clear output priv attribute. If each bit is set then the outgoing priv bit is cleared. Has precedence over priv set bits."]
pub type NoprivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Enable region. A value of 0xA enables, others disable."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Lock region. Once set the region values cannot be modified."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priv ID."]
    #[inline(always)]
    pub fn priv_id(&self) -> PrivIdR {
        PrivIdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Make outgoing secure. A value of 0xA enables, others disable."]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Make outgoing non-secure. Has precedence over secure enable bits."]
    #[inline(always)]
    pub fn nonsec(&self) -> NonsecR {
        NonsecR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
No privID replacement, pass through value."]
    #[inline(always)]
    pub fn pass(&self) -> PassR {
        PassR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Set outgoing priv attribute. If each bit is set then the outgoing priv bit is set."]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Clear output priv attribute. If each bit is set then the outgoing priv bit is cleared. Has precedence over priv set bits."]
    #[inline(always)]
    pub fn nopriv(&self) -> NoprivR {
        NoprivR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Enable region. A value of 0xA enables, others disable."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<RingaccIscIscControlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Lock region. Once set the region values cannot be modified."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<RingaccIscIscControlSpec> {
        LockW::new(self, 4)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priv ID."]
    #[inline(always)]
    #[must_use]
    pub fn priv_id(&mut self) -> PrivIdW<RingaccIscIscControlSpec> {
        PrivIdW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Make outgoing secure. A value of 0xA enables, others disable."]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SecW<RingaccIscIscControlSpec> {
        SecW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Make outgoing non-secure. Has precedence over secure enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn nonsec(&mut self) -> NonsecW<RingaccIscIscControlSpec> {
        NonsecW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
No privID replacement, pass through value."]
    #[inline(always)]
    #[must_use]
    pub fn pass(&mut self) -> PassW<RingaccIscIscControlSpec> {
        PassW::new(self, 21)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Set outgoing priv attribute. If each bit is set then the outgoing priv bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PrivW<RingaccIscIscControlSpec> {
        PrivW::new(self, 24)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Clear output priv attribute. If each bit is set then the outgoing priv bit is cleared. Has precedence over priv set bits."]
    #[inline(always)]
    #[must_use]
    pub fn nopriv(&mut self) -> NoprivW<RingaccIscIscControlSpec> {
        NoprivW::new(self, 26)
    }
}
#[doc = "The ISC a Region b Control Register defines the control fields for the ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_isc_isc_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_isc_isc_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccIscIscControlSpec;
impl crate::RegisterSpec for RingaccIscIscControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_isc_isc_control::R`](R) reader structure"]
impl crate::Readable for RingaccIscIscControlSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_isc_isc_control::W`](W) writer structure"]
impl crate::Writable for RingaccIscIscControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_ISC_ISC_control to value 0"]
impl crate::Resettable for RingaccIscIscControlSpec {
    const RESET_VALUE: u32 = 0;
}
