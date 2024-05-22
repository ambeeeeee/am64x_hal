#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEIDR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmheidrSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEIDR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmheidrSpec>;
#[doc = "Field `CLASS` reader - "]
pub type ClassR = crate::FieldReader;
#[doc = "Field `CLASS` writer - "]
pub type ClassW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLASSREV` reader - "]
pub type ClassrevR = crate::FieldReader;
#[doc = "Field `CLASSREV` writer - "]
pub type ClassrevW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VENDSPEC` reader - "]
pub type VendspecR = crate::FieldReader;
#[doc = "Field `VENDSPEC` writer - "]
pub type VendspecW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn class(&self) -> ClassR {
        ClassR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn classrev(&self) -> ClassrevR {
        ClassrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn vendspec(&self) -> VendspecR {
        VendspecR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn class(&mut self) -> ClassW<Vbusp2apbWrap_CxstmCfg_StmRegsStmheidrSpec> {
        ClassW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn classrev(&mut self) -> ClassrevW<Vbusp2apbWrap_CxstmCfg_StmRegsStmheidrSpec> {
        ClassrevW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn vendspec(&mut self) -> VendspecW<Vbusp2apbWrap_CxstmCfg_StmRegsStmheidrSpec> {
        VendspecW::new(self, 8)
    }
}
#[doc = "Indicates the features of hardware event tracing in the STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmheidrSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmheidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheidr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmheidrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheidr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmheidrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEIDR to value 0x11"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmheidrSpec {
    const RESET_VALUE: u32 = 0x11;
}
