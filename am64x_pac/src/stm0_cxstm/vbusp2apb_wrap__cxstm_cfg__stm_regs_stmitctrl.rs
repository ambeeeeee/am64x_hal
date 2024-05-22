#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITCTRL` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmitctrlSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITCTRL` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmitctrlSpec>;
#[doc = "Field `IME` reader - "]
pub type ImeR = crate::BitReader;
#[doc = "Field `IME` writer - "]
pub type ImeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ime(&self) -> ImeR {
        ImeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ime(&mut self) -> ImeW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitctrlSpec> {
        ImeW::new(self, 0)
    }
}
#[doc = "Used to enable topology detection. See the CoreSight Architecture Specification for more information. This register enables the component to switch between functional mode and integration mode. The default behavior is functional mode. In integration mode the inputs and outputs of the STM can be directly controlled for integration testing and topology solving. &lt;p/>Note: When a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmitctrlSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmitctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitctrl::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitctrl::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITCTRL to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitctrlSpec {
    const RESET_VALUE: u32 = 0;
}
