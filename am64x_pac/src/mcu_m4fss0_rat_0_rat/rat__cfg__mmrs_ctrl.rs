#[doc = "Register `RAT__CFG__MMRS_ctrl` reader"]
pub type R = crate::R<Rat_Cfg_MmrsCtrlSpec>;
#[doc = "Register `RAT__CFG__MMRS_ctrl` writer"]
pub type W = crate::W<Rat_Cfg_MmrsCtrlSpec>;
#[doc = "Field `SIZE` reader - 5:0\\]
Size of the Region in Address Bits. 0 = 1 byte, 1 = 2B, 2 = 4B, 3 = 8B, etc. up to 32 = 4GB."]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - 5:0\\]
Size of the Region in Address Bits. 0 = 1 byte, 1 = 2B, 2 = 4B, 3 = 8B, etc. up to 32 = 4GB."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EN` reader - 31:31\\]
Enable for the Region"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 31:31\\]
Enable for the Region"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Size of the Region in Address Bits. 0 = 1 byte, 1 = 2B, 2 = 4B, 3 = 8B, etc. up to 32 = 4GB."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable for the Region"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Size of the Region in Address Bits. 0 = 1 byte, 1 = 2B, 2 = 4B, 3 = 8B, etc. up to 32 = 4GB."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<Rat_Cfg_MmrsCtrlSpec> {
        SizeW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable for the Region"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Rat_Cfg_MmrsCtrlSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "The Control for Region a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rat_Cfg_MmrsCtrlSpec;
impl crate::RegisterSpec for Rat_Cfg_MmrsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rat__cfg__mmrs_ctrl::R`](R) reader structure"]
impl crate::Readable for Rat_Cfg_MmrsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rat__cfg__mmrs_ctrl::W`](W) writer structure"]
impl crate::Writable for Rat_Cfg_MmrsCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAT__CFG__MMRS_ctrl to value 0"]
impl crate::Resettable for Rat_Cfg_MmrsCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
