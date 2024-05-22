#[doc = "Register `CFG0_PBIST_EN` reader"]
pub type R = crate::R<Cfg0PbistEnSpec>;
#[doc = "Register `CFG0_PBIST_EN` writer"]
pub type W = crate::W<Cfg0PbistEnSpec>;
#[doc = "Field `PBIST_EN_EMMC0` reader - 0:0\\]
Activates PBIST Access to MMC0 Memories"]
pub type PbistEnEmmc0R = crate::BitReader;
#[doc = "Field `PBIST_EN_EMMC0` writer - 0:0\\]
Activates PBIST Access to MMC0 Memories"]
pub type PbistEnEmmc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIST_EN_EMMC1` reader - 1:1\\]
Activates PBIST Access to MMC1 Memories"]
pub type PbistEnEmmc1R = crate::BitReader;
#[doc = "Field `PBIST_EN_EMMC1` writer - 1:1\\]
Activates PBIST Access to MMC1 Memories"]
pub type PbistEnEmmc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIST_EN_USB0` reader - 4:4\\]
Activates PBIST Access to USB0 Memories"]
pub type PbistEnUsb0R = crate::BitReader;
#[doc = "Field `PBIST_EN_USB0` writer - 4:4\\]
Activates PBIST Access to USB0 Memories"]
pub type PbistEnUsb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIST_EN_PCIE0` reader - 8:8\\]
Activates PBIST Access to PCIE0 Memories"]
pub type PbistEnPcie0R = crate::BitReader;
#[doc = "Field `PBIST_EN_PCIE0` writer - 8:8\\]
Activates PBIST Access to PCIE0 Memories"]
pub type PbistEnPcie0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Activates PBIST Access to MMC0 Memories"]
    #[inline(always)]
    pub fn pbist_en_emmc0(&self) -> PbistEnEmmc0R {
        PbistEnEmmc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Activates PBIST Access to MMC1 Memories"]
    #[inline(always)]
    pub fn pbist_en_emmc1(&self) -> PbistEnEmmc1R {
        PbistEnEmmc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Activates PBIST Access to USB0 Memories"]
    #[inline(always)]
    pub fn pbist_en_usb0(&self) -> PbistEnUsb0R {
        PbistEnUsb0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Activates PBIST Access to PCIE0 Memories"]
    #[inline(always)]
    pub fn pbist_en_pcie0(&self) -> PbistEnPcie0R {
        PbistEnPcie0R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Activates PBIST Access to MMC0 Memories"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_en_emmc0(&mut self) -> PbistEnEmmc0W<Cfg0PbistEnSpec> {
        PbistEnEmmc0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Activates PBIST Access to MMC1 Memories"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_en_emmc1(&mut self) -> PbistEnEmmc1W<Cfg0PbistEnSpec> {
        PbistEnEmmc1W::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Activates PBIST Access to USB0 Memories"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_en_usb0(&mut self) -> PbistEnUsb0W<Cfg0PbistEnSpec> {
        PbistEnUsb0W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Activates PBIST Access to PCIE0 Memories"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_en_pcie0(&mut self) -> PbistEnPcie0W<Cfg0PbistEnSpec> {
        PbistEnPcie0W::new(self, 8)
    }
}
#[doc = "CFG0_PBIST_EN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pbist_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pbist_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PbistEnSpec;
impl crate::RegisterSpec for Cfg0PbistEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pbist_en::R`](R) reader structure"]
impl crate::Readable for Cfg0PbistEnSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pbist_en::W`](W) writer structure"]
impl crate::Writable for Cfg0PbistEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PBIST_EN to value 0"]
impl crate::Resettable for Cfg0PbistEnSpec {
    const RESET_VALUE: u32 = 0;
}
