#[doc = "Register `REG_QPOSCTL` reader"]
pub type R = crate::R<RegQposctlSpec>;
#[doc = "Register `REG_QPOSCTL` writer"]
pub type W = crate::W<RegQposctlSpec>;
#[doc = "Field `PCSPW` reader - 11:0\\]
Select-position-compare sync output pulse width"]
pub type PcspwR = crate::FieldReader<u16>;
#[doc = "Field `PCSPW` writer - 11:0\\]
Select-position-compare sync output pulse width"]
pub type PcspwW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PCE` reader - 12:12\\]
Position compare enable/disable"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - 12:12\\]
Position compare enable/disable"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCPOL` reader - 13:13\\]
Polarity of sync output"]
pub type PcpolR = crate::BitReader;
#[doc = "Field `PCPOL` writer - 13:13\\]
Polarity of sync output"]
pub type PcpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLOAD` reader - 14:14\\]
Position compare of shadow load"]
pub type PcloadR = crate::BitReader;
#[doc = "Field `PCLOAD` writer - 14:14\\]
Position compare of shadow load"]
pub type PcloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSHDW` reader - 15:15\\]
Position compare of shadow enable"]
pub type PcshdwR = crate::BitReader;
#[doc = "Field `PCSHDW` writer - 15:15\\]
Position compare of shadow enable"]
pub type PcshdwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Select-position-compare sync output pulse width"]
    #[inline(always)]
    pub fn pcspw(&self) -> PcspwR {
        PcspwR::new(self.bits & 0x0fff)
    }
    #[doc = "Bit 12 - 12:12\\]
Position compare enable/disable"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Polarity of sync output"]
    #[inline(always)]
    pub fn pcpol(&self) -> PcpolR {
        PcpolR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Position compare of shadow load"]
    #[inline(always)]
    pub fn pcload(&self) -> PcloadR {
        PcloadR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Position compare of shadow enable"]
    #[inline(always)]
    pub fn pcshdw(&self) -> PcshdwR {
        PcshdwR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Select-position-compare sync output pulse width"]
    #[inline(always)]
    #[must_use]
    pub fn pcspw(&mut self) -> PcspwW<RegQposctlSpec> {
        PcspwW::new(self, 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Position compare enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PceW<RegQposctlSpec> {
        PceW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Polarity of sync output"]
    #[inline(always)]
    #[must_use]
    pub fn pcpol(&mut self) -> PcpolW<RegQposctlSpec> {
        PcpolW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Position compare of shadow load"]
    #[inline(always)]
    #[must_use]
    pub fn pcload(&mut self) -> PcloadW<RegQposctlSpec> {
        PcloadW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Position compare of shadow enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcshdw(&mut self) -> PcshdwW<RegQposctlSpec> {
        PcshdwW::new(self, 15)
    }
}
#[doc = "Position Compare Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQposctlSpec;
impl crate::RegisterSpec for RegQposctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_qposctl::R`](R) reader structure"]
impl crate::Readable for RegQposctlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qposctl::W`](W) writer structure"]
impl crate::Writable for RegQposctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_QPOSCTL to value 0"]
impl crate::Resettable for RegQposctlSpec {
    const RESET_VALUE: u16 = 0;
}
