#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TOCV` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsTocvSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TOCV` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsTocvSpec>;
#[doc = "Field `TOC` reader - 15:0\\]
Timeout Counter"]
pub type TocR = crate::FieldReader<u16>;
#[doc = "Field `TOC` writer - 15:0\\]
Timeout Counter"]
pub type TocW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Timeout Counter"]
    #[inline(always)]
    pub fn toc(&self) -> TocR {
        TocR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TocW<McanWrap_McanCfgVbp_McanRegsTocvSpec> {
        TocW::new(self, 0)
    }
}
#[doc = "Read/reset timeout counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tocv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tocv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsTocvSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsTocvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tocv::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsTocvSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tocv::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsTocvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TOCV to value 0x0006_5535"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsTocvSpec {
    const RESET_VALUE: u32 = 0x0006_5535;
}
