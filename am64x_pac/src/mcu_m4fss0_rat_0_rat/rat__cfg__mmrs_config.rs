#[doc = "Register `RAT__CFG__MMRS_config` reader"]
pub type R = crate::R<Rat_Cfg_MmrsConfigSpec>;
#[doc = "Register `RAT__CFG__MMRS_config` writer"]
pub type W = crate::W<Rat_Cfg_MmrsConfigSpec>;
#[doc = "Field `REGIONS` reader - 7:0\\]
Number of regions"]
pub type RegionsR = crate::FieldReader;
#[doc = "Field `REGIONS` writer - 7:0\\]
Number of regions"]
pub type RegionsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDRS` reader - 15:8\\]
Number of addresses"]
pub type AddrsR = crate::FieldReader;
#[doc = "Field `ADDRS` writer - 15:8\\]
Number of addresses"]
pub type AddrsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR_WIDTH` reader - 23:16\\]
Number of address bits"]
pub type AddrWidthR = crate::FieldReader;
#[doc = "Field `ADDR_WIDTH` writer - 23:16\\]
Number of address bits"]
pub type AddrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Number of regions"]
    #[inline(always)]
    pub fn regions(&self) -> RegionsR {
        RegionsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of addresses"]
    #[inline(always)]
    pub fn addrs(&self) -> AddrsR {
        AddrsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Number of address bits"]
    #[inline(always)]
    pub fn addr_width(&self) -> AddrWidthR {
        AddrWidthR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Number of regions"]
    #[inline(always)]
    #[must_use]
    pub fn regions(&mut self) -> RegionsW<Rat_Cfg_MmrsConfigSpec> {
        RegionsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of addresses"]
    #[inline(always)]
    #[must_use]
    pub fn addrs(&mut self) -> AddrsW<Rat_Cfg_MmrsConfigSpec> {
        AddrsW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Number of address bits"]
    #[inline(always)]
    #[must_use]
    pub fn addr_width(&mut self) -> AddrWidthW<Rat_Cfg_MmrsConfigSpec> {
        AddrWidthW::new(self, 16)
    }
}
#[doc = "The Config Register contains the configuration values for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rat_Cfg_MmrsConfigSpec;
impl crate::RegisterSpec for Rat_Cfg_MmrsConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rat__cfg__mmrs_config::R`](R) reader structure"]
impl crate::Readable for Rat_Cfg_MmrsConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`rat__cfg__mmrs_config::W`](W) writer structure"]
impl crate::Writable for Rat_Cfg_MmrsConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAT__CFG__MMRS_config to value 0x0036_0108"]
impl crate::Resettable for Rat_Cfg_MmrsConfigSpec {
    const RESET_VALUE: u32 = 0x0036_0108;
}
