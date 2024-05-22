#[doc = "Register `CFG0_JTAGID` reader"]
pub type R = crate::R<Cfg0JtagidSpec>;
#[doc = "Register `CFG0_JTAGID` writer"]
pub type W = crate::W<Cfg0JtagidSpec>;
#[doc = "Field `JTAGID_LSB` reader - 0:0\\]
Always 1"]
pub type JtagidLsbR = crate::BitReader;
#[doc = "Field `JTAGID_LSB` writer - 0:0\\]
Always 1"]
pub type JtagidLsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAGID_MFG` reader - 11:1\\]
Indicates manufacturer"]
pub type JtagidMfgR = crate::FieldReader<u16>;
#[doc = "Field `JTAGID_MFG` writer - 11:1\\]
Indicates manufacturer"]
pub type JtagidMfgW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `JTAGID_PARTNO` reader - 27:12\\]
Part number for boundary scan"]
pub type JtagidPartnoR = crate::FieldReader<u16>;
#[doc = "Field `JTAGID_PARTNO` writer - 27:12\\]
Part number for boundary scan"]
pub type JtagidPartnoW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `JTAGID_VARIANT` reader - 31:28\\]
Used to indicate new PGs"]
pub type JtagidVariantR = crate::FieldReader;
#[doc = "Field `JTAGID_VARIANT` writer - 31:28\\]
Used to indicate new PGs"]
pub type JtagidVariantW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Always 1"]
    #[inline(always)]
    pub fn jtagid_lsb(&self) -> JtagidLsbR {
        JtagidLsbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:11 - 11:1\\]
Indicates manufacturer"]
    #[inline(always)]
    pub fn jtagid_mfg(&self) -> JtagidMfgR {
        JtagidMfgR::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Part number for boundary scan"]
    #[inline(always)]
    pub fn jtagid_partno(&self) -> JtagidPartnoR {
        JtagidPartnoR::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Used to indicate new PGs"]
    #[inline(always)]
    pub fn jtagid_variant(&self) -> JtagidVariantR {
        JtagidVariantR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Always 1"]
    #[inline(always)]
    #[must_use]
    pub fn jtagid_lsb(&mut self) -> JtagidLsbW<Cfg0JtagidSpec> {
        JtagidLsbW::new(self, 0)
    }
    #[doc = "Bits 1:11 - 11:1\\]
Indicates manufacturer"]
    #[inline(always)]
    #[must_use]
    pub fn jtagid_mfg(&mut self) -> JtagidMfgW<Cfg0JtagidSpec> {
        JtagidMfgW::new(self, 1)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Part number for boundary scan"]
    #[inline(always)]
    #[must_use]
    pub fn jtagid_partno(&mut self) -> JtagidPartnoW<Cfg0JtagidSpec> {
        JtagidPartnoW::new(self, 12)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Used to indicate new PGs"]
    #[inline(always)]
    #[must_use]
    pub fn jtagid_variant(&mut self) -> JtagidVariantW<Cfg0JtagidSpec> {
        JtagidVariantW::new(self, 28)
    }
}
#[doc = "CFG0_JTAGID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_jtagid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_jtagid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0JtagidSpec;
impl crate::RegisterSpec for Cfg0JtagidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_jtagid::R`](R) reader structure"]
impl crate::Readable for Cfg0JtagidSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_jtagid::W`](W) writer structure"]
impl crate::Writable for Cfg0JtagidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_JTAGID to value 0x5792_8047"]
impl crate::Resettable for Cfg0JtagidSpec {
    const RESET_VALUE: u32 = 0x5792_8047;
}
