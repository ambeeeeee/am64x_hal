#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMAUTHSTATUS` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatusSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMAUTHSTATUS` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatusSpec>;
#[doc = "Field `NSID` reader - "]
pub type NsidR = crate::FieldReader;
#[doc = "Field `NSID` writer - "]
pub type NsidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NSNID` reader - "]
pub type NsnidR = crate::FieldReader;
#[doc = "Field `NSNID` writer - "]
pub type NsnidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SID` reader - "]
pub type SidR = crate::FieldReader;
#[doc = "Field `SID` writer - "]
pub type SidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNID` reader - "]
pub type SnidR = crate::FieldReader;
#[doc = "Field `SNID` writer - "]
pub type SnidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn nsid(&self) -> NsidR {
        NsidR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn nsnid(&self) -> NsnidR {
        NsnidR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn snid(&self) -> SnidR {
        SnidR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn nsid(&mut self) -> NsidW<Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatusSpec> {
        NsidW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn nsnid(&mut self) -> NsnidW<Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatusSpec> {
        NsnidW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn sid(&mut self) -> SidW<Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatusSpec> {
        SidW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn snid(&mut self) -> SnidW<Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatusSpec> {
        SnidW::new(self, 6)
    }
}
#[doc = "Reports the required security level and current status of the authentication interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauthstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauthstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatusSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauthstatus::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauthstatus::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMAUTHSTATUS to value 0xaa"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatusSpec {
    const RESET_VALUE: u32 = 0xaa;
}
