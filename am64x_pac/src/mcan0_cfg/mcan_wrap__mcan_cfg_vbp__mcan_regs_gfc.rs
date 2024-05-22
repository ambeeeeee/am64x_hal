#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_GFC` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsGfcSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_GFC` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsGfcSpec>;
#[doc = "Field `RRFE` reader - 0:0\\]
reject Remote Frames Extended"]
pub type RrfeR = crate::BitReader;
#[doc = "Field `RRFE` writer - 0:0\\]
reject Remote Frames Extended"]
pub type RrfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRFS` reader - 1:1\\]
reject Remote Frames Standard"]
pub type RrfsR = crate::BitReader;
#[doc = "Field `RRFS` writer - 1:1\\]
reject Remote Frames Standard"]
pub type RrfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANFE` reader - 3:2\\]
Accept Non-matching Frames Extended"]
pub type AnfeR = crate::FieldReader;
#[doc = "Field `ANFE` writer - 3:2\\]
Accept Non-matching Frames Extended"]
pub type AnfeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ANFS` reader - 5:4\\]
Accept Non-matching Frames Standard"]
pub type AnfsR = crate::FieldReader;
#[doc = "Field `ANFS` writer - 5:4\\]
Accept Non-matching Frames Standard"]
pub type AnfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
reject Remote Frames Extended"]
    #[inline(always)]
    pub fn rrfe(&self) -> RrfeR {
        RrfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
reject Remote Frames Standard"]
    #[inline(always)]
    pub fn rrfs(&self) -> RrfsR {
        RrfsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub fn anfe(&self) -> AnfeR {
        AnfeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub fn anfs(&self) -> AnfsR {
        AnfsR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
reject Remote Frames Extended"]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RrfeW<McanWrap_McanCfgVbp_McanRegsGfcSpec> {
        RrfeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
reject Remote Frames Standard"]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RrfsW<McanWrap_McanCfgVbp_McanRegsGfcSpec> {
        RrfsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Accept Non-matching Frames Extended"]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> AnfeW<McanWrap_McanCfgVbp_McanRegsGfcSpec> {
        AnfeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Accept Non-matching Frames Standard"]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> AnfsW<McanWrap_McanCfgVbp_McanRegsGfcSpec> {
        AnfsW::new(self, 4)
    }
}
#[doc = "Handling of non-matching frames and remote frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_gfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_gfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsGfcSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsGfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_gfc::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsGfcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_gfc::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsGfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_GFC to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsGfcSpec {
    const RESET_VALUE: u32 = 0;
}
