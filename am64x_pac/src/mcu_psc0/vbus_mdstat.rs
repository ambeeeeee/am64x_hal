#[doc = "Register `VBUS_MDSTAT` reader"]
pub type R = crate::R<VbusMdstatSpec>;
#[doc = "Register `VBUS_MDSTAT` writer"]
pub type W = crate::W<VbusMdstatSpec>;
#[doc = "Field `STATE` reader - 5:0\\]
These bits indicate the current module state"]
pub type StateR = crate::FieldReader;
#[doc = "Field `STATE` writer - 5:0\\]
These bits indicate the current module state"]
pub type StateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LRSTZ` reader - 8:8\\]
Module local reset actual status"]
pub type LrstzR = crate::BitReader;
#[doc = "Field `LRSTZ` writer - 8:8\\]
Module local reset actual status"]
pub type LrstzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LRSTDONE` reader - 9:9\\]
Module local reset initialization done status"]
pub type LrstdoneR = crate::BitReader;
#[doc = "Field `LRSTDONE` writer - 9:9\\]
Module local reset initialization done status"]
pub type LrstdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRSTZ` reader - 10:10\\]
Module reset actual status"]
pub type MrstzR = crate::BitReader;
#[doc = "Field `MRSTZ` writer - 10:10\\]
Module reset actual status"]
pub type MrstzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRSTDONE` reader - 11:11\\]
Module reset initialization done status"]
pub type MrstdoneR = crate::BitReader;
#[doc = "Field `MRSTDONE` writer - 11:11\\]
Module reset initialization done status"]
pub type MrstdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKOUT` reader - 12:12\\]
Actual modclk output to module"]
pub type MckoutR = crate::BitReader;
#[doc = "Field `MCKOUT` writer - 12:12\\]
Actual modclk output to module"]
pub type MckoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMURST` reader - 16:16\\]
Emulation Alters Reset"]
pub type EmurstR = crate::BitReader;
#[doc = "Field `EMURST` writer - 16:16\\]
Emulation Alters Reset"]
pub type EmurstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMUIHB` reader - 17:17\\]
Emulation Alters Module State. Inhibits Module Inactive or Force Module Active."]
pub type EmuihbR = crate::BitReader;
#[doc = "Field `EMUIHB` writer - 17:17\\]
Emulation Alters Module State. Inhibits Module Inactive or Force Module Active."]
pub type EmuihbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
These bits indicate the current module state"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Module local reset actual status"]
    #[inline(always)]
    pub fn lrstz(&self) -> LrstzR {
        LrstzR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Module local reset initialization done status"]
    #[inline(always)]
    pub fn lrstdone(&self) -> LrstdoneR {
        LrstdoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Module reset actual status"]
    #[inline(always)]
    pub fn mrstz(&self) -> MrstzR {
        MrstzR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Module reset initialization done status"]
    #[inline(always)]
    pub fn mrstdone(&self) -> MrstdoneR {
        MrstdoneR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Actual modclk output to module"]
    #[inline(always)]
    pub fn mckout(&self) -> MckoutR {
        MckoutR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Emulation Alters Reset"]
    #[inline(always)]
    pub fn emurst(&self) -> EmurstR {
        EmurstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Emulation Alters Module State. Inhibits Module Inactive or Force Module Active."]
    #[inline(always)]
    pub fn emuihb(&self) -> EmuihbR {
        EmuihbR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
These bits indicate the current module state"]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> StateW<VbusMdstatSpec> {
        StateW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Module local reset actual status"]
    #[inline(always)]
    #[must_use]
    pub fn lrstz(&mut self) -> LrstzW<VbusMdstatSpec> {
        LrstzW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Module local reset initialization done status"]
    #[inline(always)]
    #[must_use]
    pub fn lrstdone(&mut self) -> LrstdoneW<VbusMdstatSpec> {
        LrstdoneW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Module reset actual status"]
    #[inline(always)]
    #[must_use]
    pub fn mrstz(&mut self) -> MrstzW<VbusMdstatSpec> {
        MrstzW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Module reset initialization done status"]
    #[inline(always)]
    #[must_use]
    pub fn mrstdone(&mut self) -> MrstdoneW<VbusMdstatSpec> {
        MrstdoneW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Actual modclk output to module"]
    #[inline(always)]
    #[must_use]
    pub fn mckout(&mut self) -> MckoutW<VbusMdstatSpec> {
        MckoutW::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Emulation Alters Reset"]
    #[inline(always)]
    #[must_use]
    pub fn emurst(&mut self) -> EmurstW<VbusMdstatSpec> {
        EmurstW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Emulation Alters Module State. Inhibits Module Inactive or Force Module Active."]
    #[inline(always)]
    #[must_use]
    pub fn emuihb(&mut self) -> EmuihbW<VbusMdstatSpec> {
        EmuihbW::new(self, 17)
    }
}
#[doc = "This register shows the status of each module. Requires one register per module on the device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_mdstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_mdstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusMdstatSpec;
impl crate::RegisterSpec for VbusMdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_mdstat::R`](R) reader structure"]
impl crate::Readable for VbusMdstatSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_mdstat::W`](W) writer structure"]
impl crate::Writable for VbusMdstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_MDSTAT to value 0"]
impl crate::Resettable for VbusMdstatSpec {
    const RESET_VALUE: u32 = 0;
}
