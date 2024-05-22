#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TSCV` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsTscvSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TSCV` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsTscvSpec>;
#[doc = "Field `TSC` reader - 15:0\\]
Timestamp Counter"]
pub type TscR = crate::FieldReader<u16>;
#[doc = "Field `TSC` writer - 15:0\\]
Timestamp Counter"]
pub type TscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Timestamp Counter"]
    #[inline(always)]
    pub fn tsc(&self) -> TscR {
        TscR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Timestamp Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tsc(&mut self) -> TscW<McanWrap_McanCfgVbp_McanRegsTscvSpec> {
        TscW::new(self, 0)
    }
}
#[doc = "Read/reset timestamp counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tscv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tscv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsTscvSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsTscvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tscv::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsTscvSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tscv::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsTscvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TSCV to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsTscvSpec {
    const RESET_VALUE: u32 = 0;
}
