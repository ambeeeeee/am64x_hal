#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCLAIMSET` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimsetSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCLAIMSET` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimsetSpec>;
#[doc = "Field `SET` reader - "]
pub type SetR = crate::FieldReader;
#[doc = "Field `SET` writer - "]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimsetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "This is used in conjunction with Claim Tag Clear Register, STMCLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimsetSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimset::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimsetSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimset::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCLAIMSET to value 0x15"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimsetSpec {
    const RESET_VALUE: u32 = 0x15;
}
