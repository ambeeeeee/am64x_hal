#[doc = "Register `REG_QEPCTL` reader"]
pub type R = crate::R<RegQepctlSpec>;
#[doc = "Register `REG_QEPCTL` writer"]
pub type W = crate::W<RegQepctlSpec>;
#[doc = "Field `WDE` reader - 0:0\\]
QEP watchdog enable"]
pub type WdeR = crate::BitReader;
#[doc = "Field `WDE` writer - 0:0\\]
QEP watchdog enable"]
pub type WdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTE` reader - 1:1\\]
QEP unit timer enable"]
pub type UteR = crate::BitReader;
#[doc = "Field `UTE` writer - 1:1\\]
QEP unit timer enable"]
pub type UteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QCLM` reader - 2:2\\]
QEP capture latch mode"]
pub type QclmR = crate::BitReader;
#[doc = "Field `QCLM` writer - 2:2\\]
QEP capture latch mode"]
pub type QclmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QPEN` reader - 3:3\\]
Quadrature position counter enable/software reset"]
pub type QpenR = crate::BitReader;
#[doc = "Field `QPEN` writer - 3:3\\]
Quadrature position counter enable/software reset"]
pub type QpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEL` reader - 5:4\\]
Index event latch of position counter \\[software index marker\\]"]
pub type IelR = crate::FieldReader;
#[doc = "Field `IEL` writer - 5:4\\]
Index event latch of position counter \\[software index marker\\]"]
pub type IelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SEL` reader - 6:6\\]
Strobe event latch of position counter"]
pub type SelR = crate::BitReader;
#[doc = "Field `SEL` writer - 6:6\\]
Strobe event latch of position counter"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI` reader - 7:7\\]
Software init position counter"]
pub type SwiR = crate::BitReader;
#[doc = "Field `SWI` writer - 7:7\\]
Software init position counter"]
pub type SwiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEI` reader - 9:8\\]
Index event init of position count"]
pub type IeiR = crate::FieldReader;
#[doc = "Field `IEI` writer - 9:8\\]
Index event init of position count"]
pub type IeiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SEI` reader - 11:10\\]
Strobe event initialization of position counter"]
pub type SeiR = crate::FieldReader;
#[doc = "Field `SEI` writer - 11:10\\]
Strobe event initialization of position counter"]
pub type SeiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCRM` reader - 13:12\\]
Postion counter reset"]
pub type PcrmR = crate::FieldReader;
#[doc = "Field `PCRM` writer - 13:12\\]
Postion counter reset"]
pub type PcrmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FREE_SOFT` reader - 15:14\\]
Emulation mode"]
pub type FreeSoftR = crate::FieldReader;
#[doc = "Field `FREE_SOFT` writer - 15:14\\]
Emulation mode"]
pub type FreeSoftW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
QEP watchdog enable"]
    #[inline(always)]
    pub fn wde(&self) -> WdeR {
        WdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
QEP unit timer enable"]
    #[inline(always)]
    pub fn ute(&self) -> UteR {
        UteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
QEP capture latch mode"]
    #[inline(always)]
    pub fn qclm(&self) -> QclmR {
        QclmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Quadrature position counter enable/software reset"]
    #[inline(always)]
    pub fn qpen(&self) -> QpenR {
        QpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Index event latch of position counter \\[software index marker\\]"]
    #[inline(always)]
    pub fn iel(&self) -> IelR {
        IelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Strobe event latch of position counter"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software init position counter"]
    #[inline(always)]
    pub fn swi(&self) -> SwiR {
        SwiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Index event init of position count"]
    #[inline(always)]
    pub fn iei(&self) -> IeiR {
        IeiR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Strobe event initialization of position counter"]
    #[inline(always)]
    pub fn sei(&self) -> SeiR {
        SeiR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Postion counter reset"]
    #[inline(always)]
    pub fn pcrm(&self) -> PcrmR {
        PcrmR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Emulation mode"]
    #[inline(always)]
    pub fn free_soft(&self) -> FreeSoftR {
        FreeSoftR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
QEP watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn wde(&mut self) -> WdeW<RegQepctlSpec> {
        WdeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
QEP unit timer enable"]
    #[inline(always)]
    #[must_use]
    pub fn ute(&mut self) -> UteW<RegQepctlSpec> {
        UteW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
QEP capture latch mode"]
    #[inline(always)]
    #[must_use]
    pub fn qclm(&mut self) -> QclmW<RegQepctlSpec> {
        QclmW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Quadrature position counter enable/software reset"]
    #[inline(always)]
    #[must_use]
    pub fn qpen(&mut self) -> QpenW<RegQepctlSpec> {
        QpenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Index event latch of position counter \\[software index marker\\]"]
    #[inline(always)]
    #[must_use]
    pub fn iel(&mut self) -> IelW<RegQepctlSpec> {
        IelW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Strobe event latch of position counter"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<RegQepctlSpec> {
        SelW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software init position counter"]
    #[inline(always)]
    #[must_use]
    pub fn swi(&mut self) -> SwiW<RegQepctlSpec> {
        SwiW::new(self, 7)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Index event init of position count"]
    #[inline(always)]
    #[must_use]
    pub fn iei(&mut self) -> IeiW<RegQepctlSpec> {
        IeiW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Strobe event initialization of position counter"]
    #[inline(always)]
    #[must_use]
    pub fn sei(&mut self) -> SeiW<RegQepctlSpec> {
        SeiW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Postion counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrm(&mut self) -> PcrmW<RegQepctlSpec> {
        PcrmW::new(self, 12)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Emulation mode"]
    #[inline(always)]
    #[must_use]
    pub fn free_soft(&mut self) -> FreeSoftW<RegQepctlSpec> {
        FreeSoftW::new(self, 14)
    }
}
#[doc = "QEP Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qepctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qepctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQepctlSpec;
impl crate::RegisterSpec for RegQepctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_qepctl::R`](R) reader structure"]
impl crate::Readable for RegQepctlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qepctl::W`](W) writer structure"]
impl crate::Writable for RegQepctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_QEPCTL to value 0"]
impl crate::Resettable for RegQepctlSpec {
    const RESET_VALUE: u16 = 0;
}
