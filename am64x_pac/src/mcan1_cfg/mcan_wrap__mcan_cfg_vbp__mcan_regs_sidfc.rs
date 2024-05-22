#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_SIDFC` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsSidfcSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_SIDFC` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsSidfcSpec>;
#[doc = "Field `FLSSA` reader - 15:2\\]
Filter List Standard Start Address"]
pub type FlssaR = crate::FieldReader<u16>;
#[doc = "Field `FLSSA` writer - 15:2\\]
Filter List Standard Start Address"]
pub type FlssaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LSS` reader - 23:16\\]
List Size Standard"]
pub type LssR = crate::FieldReader;
#[doc = "Field `LSS` writer - 23:16\\]
List Size Standard"]
pub type LssW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 2:15 - 15:2\\]
Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flssa(&self) -> FlssaR {
        FlssaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
List Size Standard"]
    #[inline(always)]
    pub fn lss(&self) -> LssR {
        LssR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - 15:2\\]
Filter List Standard Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn flssa(&mut self) -> FlssaW<McanWrap_McanCfgVbp_McanRegsSidfcSpec> {
        FlssaW::new(self, 2)
    }
    #[doc = "Bits 16:23 - 23:16\\]
List Size Standard"]
    #[inline(always)]
    #[must_use]
    pub fn lss(&mut self) -> LssW<McanWrap_McanCfgVbp_McanRegsSidfcSpec> {
        LssW::new(self, 16)
    }
}
#[doc = "Number of filter elements, pointer to start of filter list\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_sidfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_sidfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsSidfcSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsSidfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_sidfc::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsSidfcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_sidfc::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsSidfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_SIDFC to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsSidfcSpec {
    const RESET_VALUE: u32 = 0;
}
