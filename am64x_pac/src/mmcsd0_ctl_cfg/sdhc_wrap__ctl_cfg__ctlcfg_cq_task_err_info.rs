#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_err_info` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_err_info` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec>;
#[doc = "Field `RESP_MODE_CMD_INDEX` reader - 5:0\\]
Response Mode Error Command Index This field indicates the index of the command which was executed on the command line when an error occurred. The field is updated if a command transaction is in progress when an error is detected by CQE, or indicated by eMMC controller."]
pub type RespModeCmdIndexR = crate::FieldReader;
#[doc = "Field `RESP_MODE_CMD_INDEX` writer - 5:0\\]
Response Mode Error Command Index This field indicates the index of the command which was executed on the command line when an error occurred. The field is updated if a command transaction is in progress when an error is detected by CQE, or indicated by eMMC controller."]
pub type RespModeCmdIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESP_MODE_TASK_ID` reader - 12:8\\]
Response Mode Error Task ID This field indicates the ID of the task which was executed on the command line when an error occurred. The field is updated if a command transaction is in progress when an error is detected by CQE, or indicated by eMMC controller."]
pub type RespModeTaskIdR = crate::FieldReader;
#[doc = "Field `RESP_MODE_TASK_ID` writer - 12:8\\]
Response Mode Error Task ID This field indicates the ID of the task which was executed on the command line when an error occurred. The field is updated if a command transaction is in progress when an error is detected by CQE, or indicated by eMMC controller."]
pub type RespModeTaskIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESP_MODE_VALID` reader - 15:15\\]
Response Mode Error Fields Valid This bit is updated when an error is detected by CQE, or indicated by eMMC controller. If a command transaction is in progress when the error is detected/indicated, the bit is set to 1. If a no command transaction is in progress when the error is detected/indicated, the bit is cleared to 0."]
pub type RespModeValidR = crate::BitReader;
#[doc = "Field `RESP_MODE_VALID` writer - 15:15\\]
Response Mode Error Fields Valid This bit is updated when an error is detected by CQE, or indicated by eMMC controller. If a command transaction is in progress when the error is detected/indicated, the bit is set to 1. If a no command transaction is in progress when the error is detected/indicated, the bit is cleared to 0."]
pub type RespModeValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATERR_CMD_INDEX` reader - 21:16\\]
Data Transfer Error Command Index This field indicates the index of the command which was executed on the data lines when an error occurred. The index shall be set to EXECUTE_READ_TASK\\[CMD46\\]
or EXECUTE_WRITE_TASK \\[CMD47\\]
accord-ing to the data direction. The field is updated if a data transfer is in progress when an error is detected by CQE, or indicated by eMMC controller."]
pub type DaterrCmdIndexR = crate::FieldReader;
#[doc = "Field `DATERR_CMD_INDEX` writer - 21:16\\]
Data Transfer Error Command Index This field indicates the index of the command which was executed on the data lines when an error occurred. The index shall be set to EXECUTE_READ_TASK\\[CMD46\\]
or EXECUTE_WRITE_TASK \\[CMD47\\]
accord-ing to the data direction. The field is updated if a data transfer is in progress when an error is detected by CQE, or indicated by eMMC controller."]
pub type DaterrCmdIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DATERR_TASK_ID` reader - 28:24\\]
Data Transfer Error Task ID This field indicates the ID of the task which was executed on the data lines when an error occurred. The field is updated if a data transfer is in progress when an error is detected by CQE, or indicated by eMMC controller"]
pub type DaterrTaskIdR = crate::FieldReader;
#[doc = "Field `DATERR_TASK_ID` writer - 28:24\\]
Data Transfer Error Task ID This field indicates the ID of the task which was executed on the data lines when an error occurred. The field is updated if a data transfer is in progress when an error is detected by CQE, or indicated by eMMC controller"]
pub type DaterrTaskIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DATERR_VALID` reader - 31:31\\]
Data Transfer Error Fields Valid This bit is updated when an error is detected by CQE, or indicated by eMMC controller. If a data transfer is in progress when the error is detected/indicated, the bit is set to 1. If a no data transfer is in progress when the error is detected/indicated, the bit is cleared to 0."]
pub type DaterrValidR = crate::BitReader;
#[doc = "Field `DATERR_VALID` writer - 31:31\\]
Data Transfer Error Fields Valid This bit is updated when an error is detected by CQE, or indicated by eMMC controller. If a data transfer is in progress when the error is detected/indicated, the bit is set to 1. If a no data transfer is in progress when the error is detected/indicated, the bit is cleared to 0."]
pub type DaterrValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Response Mode Error Command Index This field indicates the index of the command which was executed on the command line when an error occurred. The field is updated if a command transaction is in progress when an error is detected by CQE, or indicated by eMMC controller."]
    #[inline(always)]
    pub fn resp_mode_cmd_index(&self) -> RespModeCmdIndexR {
        RespModeCmdIndexR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Response Mode Error Task ID This field indicates the ID of the task which was executed on the command line when an error occurred. The field is updated if a command transaction is in progress when an error is detected by CQE, or indicated by eMMC controller."]
    #[inline(always)]
    pub fn resp_mode_task_id(&self) -> RespModeTaskIdR {
        RespModeTaskIdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Response Mode Error Fields Valid This bit is updated when an error is detected by CQE, or indicated by eMMC controller. If a command transaction is in progress when the error is detected/indicated, the bit is set to 1. If a no command transaction is in progress when the error is detected/indicated, the bit is cleared to 0."]
    #[inline(always)]
    pub fn resp_mode_valid(&self) -> RespModeValidR {
        RespModeValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Data Transfer Error Command Index This field indicates the index of the command which was executed on the data lines when an error occurred. The index shall be set to EXECUTE_READ_TASK\\[CMD46\\]
or EXECUTE_WRITE_TASK \\[CMD47\\]
accord-ing to the data direction. The field is updated if a data transfer is in progress when an error is detected by CQE, or indicated by eMMC controller."]
    #[inline(always)]
    pub fn daterr_cmd_index(&self) -> DaterrCmdIndexR {
        DaterrCmdIndexR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Data Transfer Error Task ID This field indicates the ID of the task which was executed on the data lines when an error occurred. The field is updated if a data transfer is in progress when an error is detected by CQE, or indicated by eMMC controller"]
    #[inline(always)]
    pub fn daterr_task_id(&self) -> DaterrTaskIdR {
        DaterrTaskIdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Data Transfer Error Fields Valid This bit is updated when an error is detected by CQE, or indicated by eMMC controller. If a data transfer is in progress when the error is detected/indicated, the bit is set to 1. If a no data transfer is in progress when the error is detected/indicated, the bit is cleared to 0."]
    #[inline(always)]
    pub fn daterr_valid(&self) -> DaterrValidR {
        DaterrValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Response Mode Error Command Index This field indicates the index of the command which was executed on the command line when an error occurred. The field is updated if a command transaction is in progress when an error is detected by CQE, or indicated by eMMC controller."]
    #[inline(always)]
    #[must_use]
    pub fn resp_mode_cmd_index(
        &mut self,
    ) -> RespModeCmdIndexW<SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec> {
        RespModeCmdIndexW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Response Mode Error Task ID This field indicates the ID of the task which was executed on the command line when an error occurred. The field is updated if a command transaction is in progress when an error is detected by CQE, or indicated by eMMC controller."]
    #[inline(always)]
    #[must_use]
    pub fn resp_mode_task_id(
        &mut self,
    ) -> RespModeTaskIdW<SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec> {
        RespModeTaskIdW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
Response Mode Error Fields Valid This bit is updated when an error is detected by CQE, or indicated by eMMC controller. If a command transaction is in progress when the error is detected/indicated, the bit is set to 1. If a no command transaction is in progress when the error is detected/indicated, the bit is cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn resp_mode_valid(&mut self) -> RespModeValidW<SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec> {
        RespModeValidW::new(self, 15)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Data Transfer Error Command Index This field indicates the index of the command which was executed on the data lines when an error occurred. The index shall be set to EXECUTE_READ_TASK\\[CMD46\\]
or EXECUTE_WRITE_TASK \\[CMD47\\]
accord-ing to the data direction. The field is updated if a data transfer is in progress when an error is detected by CQE, or indicated by eMMC controller."]
    #[inline(always)]
    #[must_use]
    pub fn daterr_cmd_index(&mut self) -> DaterrCmdIndexW<SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec> {
        DaterrCmdIndexW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Data Transfer Error Task ID This field indicates the ID of the task which was executed on the data lines when an error occurred. The field is updated if a data transfer is in progress when an error is detected by CQE, or indicated by eMMC controller"]
    #[inline(always)]
    #[must_use]
    pub fn daterr_task_id(&mut self) -> DaterrTaskIdW<SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec> {
        DaterrTaskIdW::new(self, 24)
    }
    #[doc = "Bit 31 - 31:31\\]
Data Transfer Error Fields Valid This bit is updated when an error is detected by CQE, or indicated by eMMC controller. If a data transfer is in progress when the error is detected/indicated, the bit is set to 1. If a no data transfer is in progress when the error is detected/indicated, the bit is cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn daterr_valid(&mut self) -> DaterrValidW<SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec> {
        DaterrValidW::new(self, 31)
    }
}
#[doc = "This register is updated by CQE when an error occurs on data or command related to a task activity. When such error is detected by CQE or indicated by the eMMC controller CQE stores in CQTERRI the task IDs and the command indices of the commands which were executed on the 343 command line and data lines when the error occurred.Software is expected to use this information in the error recovery procedure.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_err_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_err_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_err_info::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_err_info::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_err_info to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec {
    const RESET_VALUE: u32 = 0;
}
