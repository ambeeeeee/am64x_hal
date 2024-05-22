#[doc = "Register `REG_QEPSRCSEL` reader"]
pub type R = crate::R<RegQepsrcselSpec>;
#[doc = "Register `REG_QEPSRCSEL` writer"]
pub type W = crate::W<RegQepsrcselSpec>;
#[doc = "Field `QEPASEL` reader - 3:0\\]
QEPA source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
pub type QepaselR = crate::FieldReader;
#[doc = "Field `QEPASEL` writer - 3:0\\]
QEPA source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
pub type QepaselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `QEPBSEL` reader - 7:4\\]
QEPB source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
pub type QepbselR = crate::FieldReader;
#[doc = "Field `QEPBSEL` writer - 7:4\\]
QEPB source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
pub type QepbselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `QEPISEL` reader - 11:8\\]
QEP Index source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
pub type QepiselR = crate::FieldReader;
#[doc = "Field `QEPISEL` writer - 11:8\\]
QEP Index source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
pub type QepiselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `QEPSSEL` reader - 15:12\\]
QEP Strobe source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
pub type QepsselR = crate::FieldReader;
#[doc = "Field `QEPSSEL` writer - 15:12\\]
QEP Strobe source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
pub type QepsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
QEPA source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
    #[inline(always)]
    pub fn qepasel(&self) -> QepaselR {
        QepaselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
QEPB source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
    #[inline(always)]
    pub fn qepbsel(&self) -> QepbselR {
        QepbselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
QEP Index source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
    #[inline(always)]
    pub fn qepisel(&self) -> QepiselR {
        QepiselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
QEP Strobe source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
    #[inline(always)]
    pub fn qepssel(&self) -> QepsselR {
        QepsselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
QEPA source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
    #[inline(always)]
    #[must_use]
    pub fn qepasel(&mut self) -> QepaselW<RegQepsrcselSpec> {
        QepaselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
QEPB source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
    #[inline(always)]
    #[must_use]
    pub fn qepbsel(&mut self) -> QepbselW<RegQepsrcselSpec> {
        QepbselW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
QEP Index source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
    #[inline(always)]
    #[must_use]
    pub fn qepisel(&mut self) -> QepiselW<RegQepsrcselSpec> {
        QepiselW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
QEP Strobe source select:0000: From device Pins \\[Default\\].0001-1111: Device dependent."]
    #[inline(always)]
    #[must_use]
    pub fn qepssel(&mut self) -> QepsselW<RegQepsrcselSpec> {
        QepsselW::new(self, 12)
    }
}
#[doc = "QEP Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qepsrcsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qepsrcsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQepsrcselSpec;
impl crate::RegisterSpec for RegQepsrcselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_qepsrcsel::R`](R) reader structure"]
impl crate::Readable for RegQepsrcselSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qepsrcsel::W`](W) writer structure"]
impl crate::Writable for RegQepsrcselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_QEPSRCSEL to value 0"]
impl crate::Resettable for RegQepsrcselSpec {
    const RESET_VALUE: u32 = 0;
}
