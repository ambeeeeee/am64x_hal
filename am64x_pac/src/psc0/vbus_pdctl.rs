#[doc = "Register `VBUS_PDCTL` reader"]
pub type R = crate::R<VbusPdctlSpec>;
#[doc = "Register `VBUS_PDCTL` writer"]
pub type W = crate::W<VbusPdctlSpec>;
#[doc = "Field `NEXT` reader - 0:0\\]
User_Desired Next Power Domain State"]
pub type NextR = crate::BitReader;
#[doc = "Field `NEXT` writer - 0:0\\]
User_Desired Next Power Domain State"]
pub type NextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPCGOOD` reader - 8:8\\]
External Power Control Power Good Indication"]
pub type EpcgoodR = crate::BitReader;
#[doc = "Field `EPCGOOD` writer - 8:8\\]
External Power Control Power Good Indication"]
pub type EpcgoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMUIHBIE` reader - 9:9\\]
Emulation alters domain state"]
pub type EmuihbieR = crate::BitReader;
#[doc = "Field `EMUIHBIE` writer - 9:9\\]
Emulation alters domain state"]
pub type EmuihbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMODE` reader - 14:12\\]
Power Down mode"]
pub type PdmodeR = crate::FieldReader;
#[doc = "Field `PDMODE` writer - 14:12\\]
Power Down mode"]
pub type PdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WAKECNT` reader - 23:16\\]
RAM wake count delay value"]
pub type WakecntR = crate::FieldReader;
#[doc = "Field `WAKECNT` writer - 23:16\\]
RAM wake count delay value"]
pub type WakecntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ISO` reader - 28:28\\]
Isolation Cell control"]
pub type IsoR = crate::BitReader;
#[doc = "Field `ISO` writer - 28:28\\]
Isolation Cell control"]
pub type IsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSW` reader - 29:29\\]
Power shorting Switch Control"]
pub type PwrswR = crate::BitReader;
#[doc = "Field `PWRSW` writer - 29:29\\]
Power shorting Switch Control"]
pub type PwrswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE` reader - 31:31\\]
Force Bit"]
pub type ForceR = crate::BitReader;
#[doc = "Field `FORCE` writer - 31:31\\]
Force Bit"]
pub type ForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
User_Desired Next Power Domain State"]
    #[inline(always)]
    pub fn next(&self) -> NextR {
        NextR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
External Power Control Power Good Indication"]
    #[inline(always)]
    pub fn epcgood(&self) -> EpcgoodR {
        EpcgoodR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Emulation alters domain state"]
    #[inline(always)]
    pub fn emuihbie(&self) -> EmuihbieR {
        EmuihbieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Power Down mode"]
    #[inline(always)]
    pub fn pdmode(&self) -> PdmodeR {
        PdmodeR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
RAM wake count delay value"]
    #[inline(always)]
    pub fn wakecnt(&self) -> WakecntR {
        WakecntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Isolation Cell control"]
    #[inline(always)]
    pub fn iso(&self) -> IsoR {
        IsoR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Power shorting Switch Control"]
    #[inline(always)]
    pub fn pwrsw(&self) -> PwrswR {
        PwrswR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Force Bit"]
    #[inline(always)]
    pub fn force(&self) -> ForceR {
        ForceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
User_Desired Next Power Domain State"]
    #[inline(always)]
    #[must_use]
    pub fn next(&mut self) -> NextW<VbusPdctlSpec> {
        NextW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
External Power Control Power Good Indication"]
    #[inline(always)]
    #[must_use]
    pub fn epcgood(&mut self) -> EpcgoodW<VbusPdctlSpec> {
        EpcgoodW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Emulation alters domain state"]
    #[inline(always)]
    #[must_use]
    pub fn emuihbie(&mut self) -> EmuihbieW<VbusPdctlSpec> {
        EmuihbieW::new(self, 9)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Power Down mode"]
    #[inline(always)]
    #[must_use]
    pub fn pdmode(&mut self) -> PdmodeW<VbusPdctlSpec> {
        PdmodeW::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
RAM wake count delay value"]
    #[inline(always)]
    #[must_use]
    pub fn wakecnt(&mut self) -> WakecntW<VbusPdctlSpec> {
        WakecntW::new(self, 16)
    }
    #[doc = "Bit 28 - 28:28\\]
Isolation Cell control"]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> IsoW<VbusPdctlSpec> {
        IsoW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Power shorting Switch Control"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsw(&mut self) -> PwrswW<VbusPdctlSpec> {
        PwrswW::new(self, 29)
    }
    #[doc = "Bit 31 - 31:31\\]
Force Bit"]
    #[inline(always)]
    #[must_use]
    pub fn force(&mut self) -> ForceW<VbusPdctlSpec> {
        ForceW::new(self, 31)
    }
}
#[doc = "This is a control register. One register per power domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_pdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_pdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusPdctlSpec;
impl crate::RegisterSpec for VbusPdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_pdctl::R`](R) reader structure"]
impl crate::Readable for VbusPdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_pdctl::W`](W) writer structure"]
impl crate::Writable for VbusPdctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_PDCTL to value 0"]
impl crate::Resettable for VbusPdctlSpec {
    const RESET_VALUE: u32 = 0;
}
