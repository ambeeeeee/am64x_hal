#[doc = "Register `VBUS_PDSTAT` reader"]
pub type R = crate::R<VbusPdstatSpec>;
#[doc = "Register `VBUS_PDSTAT` writer"]
pub type W = crate::W<VbusPdstatSpec>;
#[doc = "Field `STATE` reader - 4:0\\]
Current Power Domain State"]
pub type StateR = crate::FieldReader;
#[doc = "Field `STATE` writer - 4:0\\]
Current Power Domain State"]
pub type StateW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORZ` reader - 8:8\\]
PORz output actual status"]
pub type PorzR = crate::BitReader;
#[doc = "Field `PORZ` writer - 8:8\\]
PORz output actual status"]
pub type PorzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORDONE` reader - 9:9\\]
POR Done Input Status"]
pub type PordoneR = crate::BitReader;
#[doc = "Field `PORDONE` writer - 9:9\\]
POR Done Input Status"]
pub type PordoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRBAD` reader - 10:10\\]
Power Bad error"]
pub type PwrbadR = crate::BitReader;
#[doc = "Field `PWRBAD` writer - 10:10\\]
Power Bad error"]
pub type PwrbadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMUIHB` reader - 11:11\\]
Emulation Alters Domain State"]
pub type EmuihbR = crate::BitReader;
#[doc = "Field `EMUIHB` writer - 11:11\\]
Emulation Alters Domain State"]
pub type EmuihbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Current Power Domain State"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
PORz output actual status"]
    #[inline(always)]
    pub fn porz(&self) -> PorzR {
        PorzR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
POR Done Input Status"]
    #[inline(always)]
    pub fn pordone(&self) -> PordoneR {
        PordoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Power Bad error"]
    #[inline(always)]
    pub fn pwrbad(&self) -> PwrbadR {
        PwrbadR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Emulation Alters Domain State"]
    #[inline(always)]
    pub fn emuihb(&self) -> EmuihbR {
        EmuihbR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Current Power Domain State"]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> StateW<VbusPdstatSpec> {
        StateW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
PORz output actual status"]
    #[inline(always)]
    #[must_use]
    pub fn porz(&mut self) -> PorzW<VbusPdstatSpec> {
        PorzW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
POR Done Input Status"]
    #[inline(always)]
    #[must_use]
    pub fn pordone(&mut self) -> PordoneW<VbusPdstatSpec> {
        PordoneW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Power Bad error"]
    #[inline(always)]
    #[must_use]
    pub fn pwrbad(&mut self) -> PwrbadW<VbusPdstatSpec> {
        PwrbadW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Emulation Alters Domain State"]
    #[inline(always)]
    #[must_use]
    pub fn emuihb(&mut self) -> EmuihbW<VbusPdstatSpec> {
        EmuihbW::new(self, 11)
    }
}
#[doc = "This is a status register. One register per power domain. Each register contains the status for the given power domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_pdstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_pdstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusPdstatSpec;
impl crate::RegisterSpec for VbusPdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_pdstat::R`](R) reader structure"]
impl crate::Readable for VbusPdstatSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_pdstat::W`](W) writer structure"]
impl crate::Writable for VbusPdstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_PDSTAT to value 0"]
impl crate::Resettable for VbusPdstatSpec {
    const RESET_VALUE: u32 = 0;
}
