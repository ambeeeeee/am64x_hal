#[doc = "Register `BCDMA_GCFG_EMU_CTRL` reader"]
pub type R = crate::R<BcdmaGcfgEmuCtrlSpec>;
#[doc = "Register `BCDMA_GCFG_EMU_CTRL` writer"]
pub type W = crate::W<BcdmaGcfgEmuCtrlSpec>;
#[doc = "Field `FREE` reader - 0:0\\]
Free"]
pub type FreeR = crate::BitReader;
#[doc = "Field `FREE` writer - 0:0\\]
Free"]
pub type FreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFT` reader - 1:1\\]
Soft"]
pub type SoftR = crate::BitReader;
#[doc = "Field `SOFT` writer - 1:1\\]
Soft"]
pub type SoftW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Free"]
    #[inline(always)]
    pub fn free(&self) -> FreeR {
        FreeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Soft"]
    #[inline(always)]
    pub fn soft(&self) -> SoftR {
        SoftR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Free"]
    #[inline(always)]
    #[must_use]
    pub fn free(&mut self) -> FreeW<BcdmaGcfgEmuCtrlSpec> {
        FreeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Soft"]
    #[inline(always)]
    #[must_use]
    pub fn soft(&mut self) -> SoftW<BcdmaGcfgEmuCtrlSpec> {
        SoftW::new(self, 1)
    }
}
#[doc = "The emulation control register is used to control the behavior of the DMA when the emususp input is asserted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_emu_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_emu_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaGcfgEmuCtrlSpec;
impl crate::RegisterSpec for BcdmaGcfgEmuCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_gcfg_emu_ctrl::R`](R) reader structure"]
impl crate::Readable for BcdmaGcfgEmuCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_gcfg_emu_ctrl::W`](W) writer structure"]
impl crate::Writable for BcdmaGcfgEmuCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_GCFG_EMU_CTRL to value 0"]
impl crate::Resettable for BcdmaGcfgEmuCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
