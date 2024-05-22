#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_isr` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsIsrSpec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_isr` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsIsrSpec>;
#[doc = "Field `CTR_WKV` reader - 3:0\\]
AES mode 0 enabled region violated Wrt Once Per Wrt Key rule"]
pub type CtrWkvR = crate::FieldReader;
#[doc = "Field `CTR_WKV` writer - 3:0\\]
AES mode 0 enabled region violated Wrt Once Per Wrt Key rule"]
pub type CtrWkvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REGION_BV` reader - 7:4\\]
Region overflow boundary event caused by a burst transaction crossed a start or end of a region"]
pub type RegionBvR = crate::FieldReader;
#[doc = "Field `REGION_BV` writer - 7:4\\]
Region overflow boundary event caused by a burst transaction crossed a start or end of a region"]
pub type RegionBvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRT_ERR` reader - 11:8\\]
Write error"]
pub type WrtErrR = crate::FieldReader;
#[doc = "Field `WRT_ERR` writer - 11:8\\]
Write error"]
pub type WrtErrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAC_ERR` reader - 15:12\\]
MAC error"]
pub type MacErrR = crate::FieldReader;
#[doc = "Field `MAC_ERR` writer - 15:12\\]
MAC error"]
pub type MacErrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
AES mode 0 enabled region violated Wrt Once Per Wrt Key rule"]
    #[inline(always)]
    pub fn ctr_wkv(&self) -> CtrWkvR {
        CtrWkvR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Region overflow boundary event caused by a burst transaction crossed a start or end of a region"]
    #[inline(always)]
    pub fn region_bv(&self) -> RegionBvR {
        RegionBvR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Write error"]
    #[inline(always)]
    pub fn wrt_err(&self) -> WrtErrR {
        WrtErrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
MAC error"]
    #[inline(always)]
    pub fn mac_err(&self) -> MacErrR {
        MacErrR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
AES mode 0 enabled region violated Wrt Once Per Wrt Key rule"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_wkv(&mut self) -> CtrWkvW<Fsas_FsasOtfaCfg_FsasOtfaRegsIsrSpec> {
        CtrWkvW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Region overflow boundary event caused by a burst transaction crossed a start or end of a region"]
    #[inline(always)]
    #[must_use]
    pub fn region_bv(&mut self) -> RegionBvW<Fsas_FsasOtfaCfg_FsasOtfaRegsIsrSpec> {
        RegionBvW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Write error"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_err(&mut self) -> WrtErrW<Fsas_FsasOtfaCfg_FsasOtfaRegsIsrSpec> {
        WrtErrW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
MAC error"]
    #[inline(always)]
    #[must_use]
    pub fn mac_err(&mut self) -> MacErrW<Fsas_FsasOtfaCfg_FsasOtfaRegsIsrSpec> {
        MacErrW::new(self, 12)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_isr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsIsrSpec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_isr::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_isr::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsIsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_isr to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsIsrSpec {
    const RESET_VALUE: u32 = 0;
}
