#[doc = "Register `VBUS_MDCTL` reader"]
pub type R = crate::R<VbusMdctlSpec>;
#[doc = "Register `VBUS_MDCTL` writer"]
pub type W = crate::W<VbusMdctlSpec>;
#[doc = "Field `NEXT` reader - 4:0\\]
Module Next State"]
pub type NextR = crate::FieldReader;
#[doc = "Field `NEXT` writer - 4:0\\]
Module Next State"]
pub type NextW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LRSTZ` reader - 8:8\\]
Module local reset control"]
pub type LrstzR = crate::BitReader;
#[doc = "Field `LRSTZ` writer - 8:8\\]
Module local reset control"]
pub type LrstzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMURSTIE` reader - 9:9\\]
Emulation Alter Reset Interrupt Enable"]
pub type EmurstieR = crate::BitReader;
#[doc = "Field `EMURSTIE` writer - 9:9\\]
Emulation Alter Reset Interrupt Enable"]
pub type EmurstieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMUIHBIE` reader - 10:10\\]
Emulation Alters Module State. Inhibits Module Inactive or Force Module Active."]
pub type EmuihbieR = crate::BitReader;
#[doc = "Field `EMUIHBIE` writer - 10:10\\]
Emulation Alters Module State. Inhibits Module Inactive or Force Module Active."]
pub type EmuihbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKCHIP1RST` reader - 11:11\\]
Block Chip_1_Reset"]
pub type Blkchip1rstR = crate::BitReader;
#[doc = "Field `BLKCHIP1RST` writer - 11:11\\]
Block Chip_1_Reset"]
pub type Blkchip1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETISO` reader - 12:12\\]
Reset Isolation"]
pub type ResetisoR = crate::BitReader;
#[doc = "Field `RESETISO` writer - 12:12\\]
Reset Isolation"]
pub type ResetisoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE` reader - 31:31\\]
Force Bit"]
pub type ForceR = crate::BitReader;
#[doc = "Field `FORCE` writer - 31:31\\]
Force Bit"]
pub type ForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Module Next State"]
    #[inline(always)]
    pub fn next(&self) -> NextR {
        NextR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Module local reset control"]
    #[inline(always)]
    pub fn lrstz(&self) -> LrstzR {
        LrstzR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Emulation Alter Reset Interrupt Enable"]
    #[inline(always)]
    pub fn emurstie(&self) -> EmurstieR {
        EmurstieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Emulation Alters Module State. Inhibits Module Inactive or Force Module Active."]
    #[inline(always)]
    pub fn emuihbie(&self) -> EmuihbieR {
        EmuihbieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Block Chip_1_Reset"]
    #[inline(always)]
    pub fn blkchip1rst(&self) -> Blkchip1rstR {
        Blkchip1rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Reset Isolation"]
    #[inline(always)]
    pub fn resetiso(&self) -> ResetisoR {
        ResetisoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Force Bit"]
    #[inline(always)]
    pub fn force(&self) -> ForceR {
        ForceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Module Next State"]
    #[inline(always)]
    #[must_use]
    pub fn next(&mut self) -> NextW<VbusMdctlSpec> {
        NextW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Module local reset control"]
    #[inline(always)]
    #[must_use]
    pub fn lrstz(&mut self) -> LrstzW<VbusMdctlSpec> {
        LrstzW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Emulation Alter Reset Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn emurstie(&mut self) -> EmurstieW<VbusMdctlSpec> {
        EmurstieW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Emulation Alters Module State. Inhibits Module Inactive or Force Module Active."]
    #[inline(always)]
    #[must_use]
    pub fn emuihbie(&mut self) -> EmuihbieW<VbusMdctlSpec> {
        EmuihbieW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Block Chip_1_Reset"]
    #[inline(always)]
    #[must_use]
    pub fn blkchip1rst(&mut self) -> Blkchip1rstW<VbusMdctlSpec> {
        Blkchip1rstW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Reset Isolation"]
    #[inline(always)]
    #[must_use]
    pub fn resetiso(&mut self) -> ResetisoW<VbusMdctlSpec> {
        ResetisoW::new(self, 12)
    }
    #[doc = "Bit 31 - 31:31\\]
Force Bit"]
    #[inline(always)]
    #[must_use]
    pub fn force(&mut self) -> ForceW<VbusMdctlSpec> {
        ForceW::new(self, 31)
    }
}
#[doc = "This register provides specific control for the individual module. One register per module on the device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_mdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_mdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusMdctlSpec;
impl crate::RegisterSpec for VbusMdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_mdctl::R`](R) reader structure"]
impl crate::Readable for VbusMdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_mdctl::W`](W) writer structure"]
impl crate::Writable for VbusMdctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_MDCTL to value 0"]
impl crate::Resettable for VbusMdctlSpec {
    const RESET_VALUE: u32 = 0;
}
